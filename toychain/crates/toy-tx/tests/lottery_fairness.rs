//! Fairness of the work lottery over a million blocks (Bauplan M3b): the
//! expected reward per ticket is the same in every pot, so an owner of half
//! of all tickets receives half of everything paid, base pot and jackpots
//! together. Uses the consensus functions of `toy_tx::lottery` directly.

#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_possible_truncation
)]

use std::collections::VecDeque;

use toy_crypto::hash::blake3;
use toy_tx::lottery::{Jackpot, TriggerInputs, draw_base, draw_jackpot, next_mix, trigger};
use toy_types::params::JACKPOT_MAX_SHARE_OF_YEAR;
use toy_types::{Address, Amount, Hash, Ppm};

const BLOCKS: u64 = 1_000_000;
const TICKETS_PER_BLOCK: u32 = 8;
const EPOCH: u64 = 1_440;

fn owner(n: u32) -> Address {
    let mut b = [0u8; 20];
    b[16..].copy_from_slice(&n.to_be_bytes());
    Address::from_bytes(b)
}

#[test]
fn half_of_the_tickets_win_half_of_all_prizes() {
    // Inflow of 1 TOY per block, split like spec 9.3.
    let inflow_base = Amount::from_units(850_000_000_000_000_000);
    let inflow_jackpots = [
        Amount::from_units(80_000_000_000_000_000),
        Amount::from_units(50_000_000_000_000_000),
        Amount::from_units(20_000_000_000_000_000),
    ];
    // Emission of the last 365 epochs for the jackpot cap.
    let cap = Amount::from_toy(EPOCH * 365)
        .mul_ppm(JACKPOT_MAX_SHARE_OF_YEAR)
        .unwrap();
    let a = owner(0);
    let mut base = Amount::ZERO;
    let mut pots = [Amount::ZERO; 3];
    let mut mix = Hash::default();
    let mut pending: VecDeque<Vec<(Hash, Address)>> = VecDeque::new();
    let mut recent: VecDeque<Vec<Address>> = VecDeque::new();
    let mut epoch_owners: Vec<Address> = Vec::new();
    let mut prev_epoch: Vec<Address> = Vec::new();
    let (mut to_a, mut to_all) = (0u128, 0u128);
    let mut fired = [0u32; 3];
    for height in 1..=BLOCKS {
        if height % EPOCH == 1 && height > 1 {
            prev_epoch = core::mem::take(&mut epoch_owners);
        }
        mix = next_mix(&mix, &blake3(&height.to_be_bytes()));
        base = base.checked_add(inflow_base).unwrap();
        for (pot, add) in pots.iter_mut().zip(inflow_jackpots) {
            *pot = pot.checked_add(add).unwrap();
        }
        // Tickets of this block: half belong to `a`, half to others.
        let tickets: Vec<(Hash, Address)> = (0..TICKETS_PER_BLOCK)
            .map(|i| {
                let id = blake3(&[height.to_be_bytes(), u64::from(i).to_be_bytes()].concat());
                let who = if i % 2 == 0 {
                    a
                } else {
                    owner(1 + (height as u32 % 1_000) * TICKETS_PER_BLOCK + i)
                };
                (id, who)
            })
            .collect();
        let owners: Vec<Address> = tickets.iter().map(|(_, o)| *o).collect();
        epoch_owners.extend_from_slice(&owners);
        recent.push_back(owners);
        pending.push_back(tickets);
        let mut pay = |prizes: &[(Address, Amount)]| -> Amount {
            let mut sum = Amount::ZERO;
            for (w, p) in prizes {
                sum = sum.checked_add(*p).unwrap();
                if *w == a {
                    to_a += p.units();
                }
                to_all += p.units();
            }
            sum
        };
        // Base pot: tickets of block H − 2.
        if pending.len() > 2 {
            let drawn = pending.pop_front().unwrap();
            let prizes = draw_base(&drawn, base, &mix, height - 2, |_| true).unwrap();
            base = base.checked_sub(pay(&prizes)).unwrap();
        }
        // Jackpots: Funke from blocks H − 61 … H − 2, the others from the
        // previous epoch.
        for (i, jackpot) in Jackpot::ALL.into_iter().enumerate() {
            let inputs = TriggerInputs {
                pot: pots[i],
                inflow: inflow_jackpots[i],
                tide: Ppm::ONE,
                participation: Ppm::ONE,
            };
            if trigger(jackpot, &inputs, &mix, height).unwrap().is_none() {
                continue;
            }
            let candidates: Vec<Address> = match jackpot {
                Jackpot::Funke => {
                    let n = recent.len();
                    recent
                        .iter()
                        .take(n.saturating_sub(2))
                        .flatten()
                        .copied()
                        .collect()
                }
                _ => prev_epoch.clone(),
            };
            let prizes =
                draw_jackpot(jackpot, &candidates, pots[i], cap, &mix, height, |_| true).unwrap();
            if !prizes.is_empty() {
                fired[i] += 1;
            }
            pots[i] = pots[i].checked_sub(pay(&prizes)).unwrap();
        }
        while recent.len() > 62 {
            recent.pop_front();
        }
    }
    // Everything paid plus what is left in the pots equals the inflow.
    let left = Amount::checked_sum([base, pots[0], pots[1], pots[2]]).unwrap();
    assert_eq!(
        to_all + left.units(),
        u128::from(BLOCKS) * 1_000_000_000_000_000_000
    );
    // Roughly the target intervals: Funke 240, Welle 7,200, Gezeiten
    // 86,400 blocks.
    assert!((3_000..5_500).contains(&fired[0]), "{fired:?}");
    assert!((90..200).contains(&fired[1]), "{fired:?}");
    assert!((6..25).contains(&fired[2]), "{fired:?}");
    // Deviation below 1 %: |share − 0.5| < 0.005, in integers. Measured
    // on 25.10.2026: share 0.4988 (deviation 0.24 %), Funke 4,973 times,
    // Welle 181 times, Gezeiten 13 times.
    // |2 × to_a − to_all| × 100 < to_all.
    let deviation = (2 * to_a).abs_diff(to_all);
    assert!(
        deviation * 100 < to_all,
        "to_a {to_a}, to_all {to_all}, fired {fired:?}"
    );
}
