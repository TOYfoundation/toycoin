//! Jackpots Funke, Welle and Gezeiten in the block executor (spec 9.3,
//! decisions 149 and 150).

#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use std::collections::BTreeSet;

use common::*;
use toy_crypto::secp256k1::SecretKey;
use toy_emission::Phase;
use toy_tx::ticket::sign_ticket;
use toy_tx::{Block, Jackpot, Trigger};
use toy_types::params::{EPOCH_MIN_BLOCKS, JACKPOT_MAX_SHARE_OF_YEAR};
use toy_types::{Address, Amount};

fn ticket_block(net: &Net, who: &SecretKey, nonce: u64) -> Block {
    let epoch = net.next_ctx().epoch;
    Block {
        transactions: vec![],
        tickets: vec![sign_ticket(who, CHAIN, epoch, nonce)],
    }
}

#[test]
fn funke_pays_tickets_of_the_last_60_blocks() {
    let mut net = Net::new(Phase::Normal);
    net.easy_work();
    let devices: Vec<_> = (80..90).map(key).collect();
    // (height, owner) of every ticket.
    let mut sent: Vec<(u64, Address)> = Vec::new();
    let mut nonce = 0;
    for i in 0..3_000u64 {
        let pot = net.state.lottery_pot(toy_state::LotteryPot::Funke);
        let ctx = net.next_ctx();
        let block = if i % 10 == 0 {
            nonce += 1;
            let who = &devices[(i / 10 % 10) as usize];
            sent.push((ctx.height, who.address()));
            ticket_block(&net, who, nonce)
        } else {
            Block::default()
        };
        let r = net.block(block);
        net.i0();
        let Some(payout) = r.jackpots.iter().find(|j| j.jackpot == Jackpot::Funke) else {
            continue;
        };
        // Only owners of tickets from blocks H − 61 … H − 2 can win.
        let window: BTreeSet<Address> = sent
            .iter()
            .filter(|(h, _)| *h + 61 >= ctx.height && *h + 2 <= ctx.height)
            .map(|(_, a)| *a)
            .collect();
        assert!(!payout.prizes.is_empty() && payout.prizes.len() <= 4);
        for (winner, _) in &payout.prizes {
            assert!(window.contains(winner), "winner outside the window");
        }
        let paid = Amount::checked_sum(payout.prizes.iter().map(|(_, p)| *p)).unwrap();
        let available = pot.checked_add(ctx.mint.jackpots[0]).unwrap();
        // 40 % to 100 % of the pot, at most 0.5 % of the emission of the
        // last 365 epochs (here: everything minted so far).
        let cap = ctx.minted_total.mul_ppm(JACKPOT_MAX_SHARE_OF_YEAR).unwrap();
        assert!(paid <= available && paid <= cap);
        let low = available
            .mul_ppm(toy_types::Ppm::new(400_000))
            .unwrap()
            .min(cap);
        // Rounding of the luck levels keeps at most a few units back.
        assert!(paid.checked_add(Amount::from_units(10)).unwrap() >= low);
        assert_eq!(
            net.state.lottery_pot(toy_state::LotteryPot::Funke),
            available.checked_sub(paid).unwrap()
        );
        return;
    }
    panic!("no Funke jackpot in 3,000 blocks");
}

#[test]
fn low_tide_milestone_starts_a_gezeiten_jackpot_for_the_previous_epoch() {
    let mut net = Net::new(Phase::Normal);
    net.easy_work();
    // Epoch 0: tickets of 20 devices.
    let devices: Vec<_> = (100..120).map(key).collect();
    for (i, d) in devices.iter().enumerate() {
        net.block(ticket_block(&net, d, i as u64));
    }
    net.to_next_epoch();
    assert_eq!(net.state.chain().prev_epoch_ticket_owners.len(), 20);
    // Pretend plan day 1,825 (the first low tide after the start) is
    // reached with the next block.
    net.state.chain_mut().plan_days = 1_824;
    let ctx = net.next_ctx();
    let r = net
        .try_block_with(&Block::default(), |c| c.plan_days = 1_825)
        .unwrap();
    let due = net.state.chain().jackpot_due[2];
    if r.jackpots.iter().all(|j| j.jackpot != Jackpot::Gezeiten) {
        let due = due.expect("milestone scheduled");
        assert!(due > ctx.height && due <= ctx.height + EPOCH_MIN_BLOCKS);
    }
    let owners: BTreeSet<Address> = devices.iter().map(SecretKey::address).collect();
    for _ in 0..=EPOCH_MIN_BLOCKS {
        let r = net.block(Block::default());
        net.i0();
        if let Some(j) = r.jackpots.iter().find(|j| j.jackpot == Jackpot::Gezeiten) {
            assert!(matches!(j.trigger, Trigger::Milestone | Trigger::Chance));
            // 64 to 512 winners wanted, but only 20 tickets: each wins once.
            assert!(!j.prizes.is_empty() && j.prizes.len() <= 20);
            for (winner, _) in &j.prizes {
                assert!(owners.contains(winner));
            }
            // At most 0.5 % of the emission of the last 365 epochs.
            let paid = Amount::checked_sum(j.prizes.iter().map(|(_, p)| *p)).unwrap();
            let cap = net.minted().mul_ppm(JACKPOT_MAX_SHARE_OF_YEAR).unwrap();
            assert!(paid <= cap, "{paid:?} > {cap:?}");
            assert_eq!(net.state.chain().jackpot_due[2], None);
            return;
        }
    }
    panic!("milestone jackpot did not fire within one epoch");
}

#[test]
fn jackpots_without_tickets_keep_their_pots() {
    let mut net = Net::new(Phase::Normal);
    let mut last = [Amount::ZERO; 3];
    for _ in 0..1_000 {
        let r = net.block(Block::default());
        assert!(r.jackpots.is_empty());
        for (i, pot) in toy_state::LotteryPot::JACKPOTS.iter().enumerate() {
            let now = net.state.lottery_pot(*pot);
            assert!(now > last[i]);
            last[i] = now;
        }
    }
    net.i0();
}
