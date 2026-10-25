//! Work lottery with jackpots and ToyWork difficulty (spec 9.3, decisions
//! 148 to 150).
//!
//! **Randomness.** Every block carries the mix `R_b = BLAKE3(R_(b−1) ‖ VRF
//! output of block b)`. All draws and jackpot triggers of block `H` use
//! `R_H`; the base pot of block `H` pays the tickets of block `H − 2`, the
//! jackpot Funke the tickets of blocks `H − 61` to `H − 2`. Whoever builds
//! a block with tickets cannot know the number that draws them.
//!
//! **Base pot.** Of `n` tickets `k = max(1, ceil(n × q))` win, `q` random
//! between 0.5 and 1.0, at most as many as keep every prize at or above
//! the minimum. Winners are the `k` tickets with the smallest
//! `BLAKE3(R ‖ ticket id)`.
//!
//! **Jackpots** Funke, Welle and Gezeiten fire with the chance
//! `h = F_level × F_special ÷ interval` per block, are forced at four
//! times their target level and by milestones. A jackpot pays a random 40
//! to 100 % of its pot, at most 0.5 % of the emission of the last 365
//! epochs, to 1–4, 8–64 or 64–512 tickets. Tickets of the previous epoch
//! are ranked by their position in the list (`BLAKE3(R ‖ pot ‖ index)`),
//! so the state needs only 20 bytes per ticket.
//!
//! **Prizes.** Every winner draws a luck level from 16 random bits: 1
//! (50 %), 2 (25 %), 3 (12.5 %), 5 (6.25 %) or 8 (6.25 %), and gets
//! `payout × level ÷ sum of levels`, rounded down. Several wins of one
//! address are added up; a total below the minimum prize (0.01 TOY to an
//! active address, 0.2 TOY to a new one) stays in the pot, like every
//! rounding rest. The expected reward per ticket is the same in every pot.

use toy_crypto::hash::blake3_derive;
use toy_types::params::{
    JACKPOT_FORCE_FACTOR, JACKPOT_INTERVALS, JACKPOT_LEVEL_FACTOR_BOUNDS, JACKPOT_MIN_PAYOUT,
    JACKPOT_PARTICIPATION_HIGH, JACKPOT_PARTICIPATION_LOW, JACKPOT_SPECIAL_FACTOR_BOUNDS,
    JACKPOT_WINNERS, LOTTERY_MIN_PRIZE, LOTTERY_MIN_PRIZE_ACTIVE, LOTTERY_MIN_WINNER_SHARE,
    TICKETS_MAX_PER_BLOCK, TICKETS_MIN_PER_BLOCK, TICKETS_TARGET_PER_BLOCK,
    TOYWORK_MAX_ADJUST_FACTOR,
};
use toy_types::{Address, Amount, ArithmeticError, Hash, Height, Ppm};

const MIX_CONTEXT: &str = "ToyChain 2026-10 randomness mix v1";
const DRAW_CONTEXT: &str = "ToyChain 2026-10 work lottery draw v2";
const PPM: u128 = 1_000_000;

/// The three jackpots (spec 9.3, decision 149).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Jackpot {
    /// About every 4 hours, 1 to 4 winners from the last 60 blocks.
    Funke,
    /// About every 5 days, 8 to 64 winners from the previous epoch.
    Welle,
    /// About every 60 days, 64 to 512 winners from the previous epoch.
    Gezeiten,
}

impl Jackpot {
    /// All jackpots, in the order of the pots in the state.
    pub const ALL: [Self; 3] = [Self::Funke, Self::Welle, Self::Gezeiten];

    /// Position in [`Self::ALL`] and in the parameter arrays.
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Funke => 0,
            Self::Welle => 1,
            Self::Gezeiten => 2,
        }
    }

    /// Target interval in blocks.
    #[must_use]
    pub const fn interval(self) -> u64 {
        match self {
            Self::Funke => JACKPOT_INTERVALS[0],
            Self::Welle => JACKPOT_INTERVALS[1],
            Self::Gezeiten => JACKPOT_INTERVALS[2],
        }
    }

    /// Smallest and largest number of winners.
    #[must_use]
    pub const fn winners(self) -> (u64, u64) {
        match self {
            Self::Funke => JACKPOT_WINNERS[0],
            Self::Welle => JACKPOT_WINNERS[1],
            Self::Gezeiten => JACKPOT_WINNERS[2],
        }
    }

    const fn tag(self) -> u8 {
        match self {
            Self::Funke => 1,
            Self::Welle => 2,
            Self::Gezeiten => 3,
        }
    }
}

/// Tag of the base pot in the random streams.
const BASE_TAG: u8 = 0;

/// What a random number is used for (separates the streams).
#[derive(Clone, Copy)]
#[repr(u8)]
enum Purpose {
    Share = 1,
    Level = 2,
    Trigger = 3,
    Count = 4,
    Payout = 5,
    Rank = 6,
    MilestoneDelay = 7,
}

/// Next value of the randomness mix.
#[must_use]
pub fn next_mix(previous: &Hash, vrf_output: &Hash) -> Hash {
    let mut data = [0u8; 64];
    let (a, b) = data.split_at_mut(32);
    a.copy_from_slice(previous.as_bytes());
    b.copy_from_slice(vrf_output.as_bytes());
    blake3_derive(MIX_CONTEXT, &data)
}

/// 128 random bits for `(pot, purpose, height, i)` from the mix.
fn random(mix: &Hash, pot: u8, purpose: Purpose, height: Height, i: u64) -> u128 {
    let mut data = Vec::with_capacity(50);
    data.extend_from_slice(mix.as_bytes());
    data.push(pot);
    data.push(purpose as u8);
    data.extend_from_slice(&height.to_be_bytes());
    data.extend_from_slice(&i.to_be_bytes());
    head(&blake3_derive(DRAW_CONTEXT, &data))
}

fn head(hash: &Hash) -> u128 {
    let mut bytes = [0u8; 16];
    if let Some(src) = hash.as_bytes().get(..16) {
        bytes.copy_from_slice(src);
    }
    u128::from_be_bytes(bytes)
}

/// Uniform number in `low..=high`. The modulo bias is below 2^-90 for the
/// ranges used here.
fn in_range(r: u128, low: u64, high: u64) -> u64 {
    let span = u128::from(high.saturating_sub(low)).saturating_add(1);
    let offset = r.checked_rem(span).unwrap_or(0);
    low.saturating_add(u64::try_from(offset).unwrap_or(0))
}

/// Luck level from 16 random bits: 1 (50 %), 2 (25 %), 3 (12.5 %),
/// 5 (6.25 %), 8 (6.25 %).
#[must_use]
pub const fn luck_level(bits: u16) -> u32 {
    match bits {
        0..32_768 => 1,
        32_768..49_152 => 2,
        49_152..57_344 => 3,
        57_344..61_440 => 5,
        _ => 8,
    }
}

/// Splits `payout` over the ranked `winners` (best first) by luck levels.
/// Winners from the end are left out until the smallest prize reaches the
/// minimum for active addresses; a total per address below its minimum
/// (`is_active` decides which) stays in the pot.
fn split_by_luck<F>(
    payout: Amount,
    winners: &[Address],
    mix: &Hash,
    pot: u8,
    height: Height,
    is_active: F,
) -> Result<Vec<(Address, Amount)>, ArithmeticError>
where
    F: Fn(&Address) -> bool,
{
    let mut ranked: Vec<(Address, u32)> = Vec::with_capacity(winners.len());
    for (i, w) in (0u64..).zip(winners) {
        let bits = random(mix, pot, Purpose::Level, height, i);
        let level = luck_level(u16::try_from(bits >> 112).unwrap_or(u16::MAX));
        ranked.push((*w, level));
    }
    loop {
        let sum: u128 = ranked.iter().map(|(_, l)| u128::from(*l)).sum();
        let lowest = ranked.iter().map(|(_, l)| u128::from(*l)).min();
        let Some(lowest) = lowest else {
            return Ok(Vec::new());
        };
        if payout.mul_div_floor(lowest, sum)? >= LOTTERY_MIN_PRIZE_ACTIVE {
            break;
        }
        ranked.pop();
    }
    let sum: u128 = ranked.iter().map(|(_, l)| u128::from(*l)).sum();
    let mut totals: std::collections::BTreeMap<Address, Amount> = std::collections::BTreeMap::new();
    for (winner, level) in &ranked {
        let prize = payout.mul_div_floor(u128::from(*level), sum)?;
        let entry = totals.entry(*winner).or_insert(Amount::ZERO);
        *entry = entry.checked_add(prize)?;
    }
    Ok(totals
        .into_iter()
        .filter(|(w, amount)| {
            let minimum = if is_active(w) {
                LOTTERY_MIN_PRIZE_ACTIVE
            } else {
                LOTTERY_MIN_PRIZE
            };
            *amount >= minimum
        })
        .collect())
}

/// Indices of the `k` smallest scores (ties by index), best first.
fn best(scores: &mut [(u128, usize)], k: usize) -> Vec<usize> {
    scores.sort_unstable();
    scores.iter().take(k).map(|(_, i)| *i).collect()
}

/// Draw of the base pot for the tickets `(id, owner)` committed at
/// `committed_at`, with the mix of the drawing block.
///
/// # Errors
/// [`ArithmeticError`] (not reachable).
pub fn draw_base<F>(
    tickets: &[(Hash, Address)],
    pot: Amount,
    mix: &Hash,
    committed_at: Height,
    is_active: F,
) -> Result<Vec<(Address, Amount)>, ArithmeticError>
where
    F: Fn(&Address) -> bool,
{
    let n = u128::try_from(tickets.len()).map_err(|_| ArithmeticError::Overflow)?;
    let by_pot = pot.checked_div(LOTTERY_MIN_PRIZE_ACTIVE.units())?.units();
    if n == 0 || by_pot == 0 {
        return Ok(Vec::new());
    }
    let low = LOTTERY_MIN_WINNER_SHARE.get();
    let q = u128::from(in_range(
        random(mix, BASE_TAG, Purpose::Share, committed_at, 0),
        low,
        Ppm::SCALE,
    ));
    // k = max(1, ceil(n × q)), n ≤ 32 here.
    let k = n
        .saturating_mul(q)
        .saturating_add(PPM.saturating_sub(1))
        .checked_div(PPM)
        .unwrap_or(1)
        .clamp(1, n)
        .min(by_pot);
    let mut scores: Vec<(u128, usize)> = tickets
        .iter()
        .enumerate()
        .map(|(i, (id, _))| {
            let mut data = Vec::with_capacity(73);
            data.extend_from_slice(mix.as_bytes());
            data.push(BASE_TAG);
            data.push(Purpose::Rank as u8);
            data.extend_from_slice(&committed_at.to_be_bytes());
            data.extend_from_slice(id.as_bytes());
            (head(&blake3_derive(DRAW_CONTEXT, &data)), i)
        })
        .collect();
    let k = usize::try_from(k).map_err(|_| ArithmeticError::Overflow)?;
    let winners: Vec<Address> = best(&mut scores, k)
        .into_iter()
        .filter_map(|i| tickets.get(i).map(|(_, owner)| *owner))
        .collect();
    split_by_luck(pot, &winners, mix, BASE_TAG, committed_at, is_active)
}

/// How a jackpot was started.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigger {
    /// By chance `h` in this block.
    Chance,
    /// The pot reached four times its target level.
    Forced,
    /// A milestone (block height, supply, low tide) fell due.
    Milestone,
}

/// Inputs of the trigger rule of one jackpot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TriggerInputs {
    /// Current pot (after the mint of this block).
    pub pot: Amount,
    /// Inflow of this block into the pot.
    pub inflow: Amount,
    /// Tide `W(t)` of the K table (1.0 = average).
    pub tide: Ppm,
    /// Signature participation of the previous epoch (M4; 1.0 until then).
    pub participation: Ppm,
}

/// `F_level = clamp(pot ÷ (inflow × interval), 0.25, 3)` in ppm. Without
/// inflow a non-empty pot counts as full.
///
/// # Errors
/// [`ArithmeticError`] (not reachable).
pub fn level_factor(jackpot: Jackpot, pot: Amount, inflow: Amount) -> Result<u64, ArithmeticError> {
    let (low, high) = JACKPOT_LEVEL_FACTOR_BOUNDS;
    let target = inflow.checked_mul(u128::from(jackpot.interval()))?;
    if target.is_zero() {
        return Ok(if pot.is_zero() { low } else { high });
    }
    let ratio = pot.mul_div_floor(PPM, target.units())?.units();
    Ok(u64::try_from(ratio.min(u128::from(high)))
        .unwrap_or(high)
        .max(low))
}

/// Product of the special factors in ppm, bounded to 0.5 … 3: tide
/// `1 + 0.8 × (1 − W) ÷ 0.25` (Welle, Gezeiten) and participation of the
/// previous epoch, ×1.3 from 95 %, ×0.8 below 80 % (Funke, Welle).
#[must_use]
pub fn special_factor(jackpot: Jackpot, tide: Ppm, participation: Ppm) -> u64 {
    let scale = i128::from(Ppm::SCALE);
    let tide_factor = match jackpot {
        Jackpot::Funke => scale,
        Jackpot::Welle | Jackpot::Gezeiten => {
            // 1 + 3.2 × (1 − W)
            let below = scale.saturating_sub(i128::from(tide.get()));
            scale.saturating_add(below.saturating_mul(32).saturating_div(10))
        }
    };
    let participation_factor: i128 = match jackpot {
        Jackpot::Gezeiten => scale,
        Jackpot::Funke | Jackpot::Welle => {
            if participation >= JACKPOT_PARTICIPATION_HIGH {
                1_300_000
            } else if participation < JACKPOT_PARTICIPATION_LOW {
                800_000
            } else {
                scale
            }
        }
    };
    let (low, high) = JACKPOT_SPECIAL_FACTOR_BOUNDS;
    let product = tide_factor
        .max(0)
        .saturating_mul(participation_factor)
        .checked_div(scale)
        .unwrap_or(0);
    u64::try_from(product.clamp(i128::from(low), i128::from(high))).unwrap_or(low)
}

/// Decides whether `jackpot` fires in block `height` by chance or force.
///
/// # Errors
/// [`ArithmeticError`] (not reachable).
pub fn trigger(
    jackpot: Jackpot,
    inputs: &TriggerInputs,
    mix: &Hash,
    height: Height,
) -> Result<Option<Trigger>, ArithmeticError> {
    if inputs.pot.is_zero() {
        return Ok(None);
    }
    let interval = u128::from(jackpot.interval());
    let target = inputs.inflow.checked_mul(interval)?;
    if !target.is_zero() && inputs.pot >= target.checked_mul(JACKPOT_FORCE_FACTOR)? {
        return Ok(Some(Trigger::Forced));
    }
    let level = u128::from(level_factor(jackpot, inputs.pot, inputs.inflow)?);
    let special = u128::from(special_factor(jackpot, inputs.tide, inputs.participation));
    // h = level × special ÷ (interval × 10^12); both factors in ppm.
    let chance = level.saturating_mul(special);
    let space = interval.saturating_mul(PPM).saturating_mul(PPM);
    let r = random(mix, jackpot.tag(), Purpose::Trigger, height, 0)
        .checked_rem(space)
        .unwrap_or(u128::MAX);
    Ok((r < chance).then_some(Trigger::Chance))
}

/// Random delay of a milestone jackpot: 1 to `blocks` blocks.
#[must_use]
pub fn milestone_delay(jackpot: Jackpot, mix: &Hash, height: Height, blocks: u64) -> u64 {
    in_range(
        random(mix, jackpot.tag(), Purpose::MilestoneDelay, height, 0),
        1,
        blocks.max(1),
    )
}

/// Draw of a jackpot among `owners` (one entry per ticket): a random 40 to
/// 100 % of `pot`, at most `cap`, to a random number of tickets within the
/// jackpot's range.
///
/// # Errors
/// [`ArithmeticError`] (not reachable).
pub fn draw_jackpot<F>(
    jackpot: Jackpot,
    owners: &[Address],
    pot: Amount,
    cap: Amount,
    mix: &Hash,
    height: Height,
    is_active: F,
) -> Result<Vec<(Address, Amount)>, ArithmeticError>
where
    F: Fn(&Address) -> bool,
{
    if owners.is_empty() || pot.is_zero() {
        return Ok(Vec::new());
    }
    let tag = jackpot.tag();
    let (low, high) = jackpot.winners();
    let count = in_range(random(mix, tag, Purpose::Count, height, 0), low, high);
    let share = in_range(
        random(mix, tag, Purpose::Payout, height, 0),
        JACKPOT_MIN_PAYOUT.get(),
        Ppm::SCALE,
    );
    let payout = pot.mul_ppm(Ppm::new(share))?.min(cap);
    let mut scores: Vec<(u128, usize)> = (0u64..)
        .zip(owners)
        .enumerate()
        .map(|(i, (n, _))| (random(mix, tag, Purpose::Rank, height, n), i))
        .collect();
    let k = usize::try_from(count).map_err(|_| ArithmeticError::Overflow)?;
    let winners: Vec<Address> = best(&mut scores, k)
        .into_iter()
        .filter_map(|i| owners.get(i).copied())
        .collect();
    split_by_luck(payout, &winners, mix, tag, height, is_active)
}

/// Largest number of tickets in a block that follows its predecessor
/// after `gap_seconds`: 32 at 60 seconds, proportionally less, at least 8.
#[must_use]
pub fn ticket_limit(gap_seconds: u64) -> u64 {
    TICKETS_MAX_PER_BLOCK
        .saturating_mul(gap_seconds.min(60))
        .checked_div(60)
        .unwrap_or(0)
        .max(TICKETS_MIN_PER_BLOCK)
}

/// ToyWork threshold for the next epoch: moves the number of valid
/// tickets towards 16 per block, changing by at most a factor of 4 per
/// epoch. No tickets at all makes it 4 times easier.
#[must_use]
pub fn next_threshold(current: u128, tickets: u64, blocks: u64) -> u128 {
    let target = u128::from(TICKETS_TARGET_PER_BLOCK).saturating_mul(u128::from(blocks.max(1)));
    let actual = u128::from(tickets);
    let factor = TOYWORK_MAX_ADJUST_FACTOR;
    let next = if actual == 0 {
        current.saturating_mul(factor)
    } else if actual > target {
        // Harder: threshold × target / actual, at most ÷ 4.
        let scaled = scale(current, target, actual);
        scaled.max(current.checked_div(factor).unwrap_or(0))
    } else {
        // Easier: threshold × target / actual, at most × 4.
        let scaled = scale(current, target, actual);
        scaled.min(current.saturating_mul(factor))
    };
    next.max(1)
}

/// `value × numerator / denominator` without overflow for the ranges here
/// (numerator and denominator stay far below 2^64).
fn scale(value: u128, numerator: u128, denominator: u128) -> u128 {
    if denominator == 0 {
        return value;
    }
    let whole = value.checked_div(denominator).unwrap_or(0);
    let rest = value.checked_rem(denominator).unwrap_or(0);
    whole.saturating_mul(numerator).saturating_add(
        rest.saturating_mul(numerator)
            .checked_div(denominator)
            .unwrap_or(0),
    )
}

#[cfg(test)]
#[allow(clippy::arithmetic_side_effects, clippy::indexing_slicing)]
mod tests {
    use super::*;

    fn owner(i: u32) -> Address {
        let mut b = [0u8; 20];
        b[16..].copy_from_slice(&i.to_be_bytes());
        Address::from_bytes(b)
    }

    fn tickets(n: u32) -> Vec<(Hash, Address)> {
        (0..n)
            .map(|i| {
                let mut id = [0u8; 32];
                id[..4].copy_from_slice(&i.to_be_bytes());
                (Hash::from_bytes(id), owner(i))
            })
            .collect()
    }

    fn mix(i: u32) -> Hash {
        let mut b = [0u8; 32];
        b[..4].copy_from_slice(&i.to_be_bytes());
        Hash::from_bytes(b)
    }

    const ACTIVE: fn(&Address) -> bool = |_| true;

    fn total(prizes: &[(Address, Amount)]) -> Amount {
        Amount::checked_sum(prizes.iter().map(|(_, a)| *a)).unwrap()
    }

    #[test]
    fn luck_levels_have_the_specified_odds() {
        let mut counts = [0u32; 9];
        for bits in 0..=u16::MAX {
            counts[luck_level(bits) as usize] += 1;
        }
        assert_eq!(counts[1], 32_768);
        assert_eq!(counts[2], 16_384);
        assert_eq!(counts[3], 8_192);
        assert_eq!(counts[5], 4_096);
        assert_eq!(counts[8], 4_096);
        // Mean level 2.5625: 0.5 + 0.5 + 0.375 + 0.3125 + 0.5.
    }

    #[test]
    fn base_pot_pays_half_to_all_tickets() {
        let pot = Amount::from_toy(10);
        let mut seen = std::collections::BTreeSet::new();
        for i in 0..400 {
            let prizes = draw_base(&tickets(16), pot, &mix(i), 5, ACTIVE).unwrap();
            assert!((8..=16).contains(&prizes.len()), "{}", prizes.len());
            seen.insert(prizes.len());
            assert!(total(&prizes) <= pot);
            // Only rounding stays in the pot.
            assert!(total(&prizes) > pot.checked_sub(Amount::from_units(100)).unwrap());
            assert!(prizes.iter().all(|(_, p)| *p >= LOTTERY_MIN_PRIZE_ACTIVE));
        }
        // ceil(16 × q) is 8 only for q = 0.5 exactly, so 9 to 16 appear.
        assert!(seen.contains(&9) && seen.contains(&16), "{seen:?}");
        // A single ticket always wins the whole pot.
        let one = draw_base(&tickets(1), pot, &mix(1), 5, ACTIVE).unwrap();
        assert_eq!(one, vec![(owner(0), pot)]);
    }

    #[test]
    fn small_pots_keep_the_minimum_prize() {
        // 0.03 TOY for 16 tickets: at most 3 winners of 0.01 TOY, and the
        // luck levels can cut that further.
        let pot = Amount::from_units(30_000_000_000_000_000);
        for i in 0..200 {
            let prizes = draw_base(&tickets(16), pot, &mix(i), 5, ACTIVE).unwrap();
            assert!(!prizes.is_empty() && prizes.len() <= 3);
            assert!(prizes.iter().all(|(_, p)| *p >= LOTTERY_MIN_PRIZE_ACTIVE));
        }
        // Below 0.01 TOY nobody wins, the pot waits.
        let tiny = LOTTERY_MIN_PRIZE_ACTIVE
            .checked_sub(Amount::from_units(1))
            .unwrap();
        assert!(
            draw_base(&tickets(4), tiny, &mix(0), 5, ACTIVE)
                .unwrap()
                .is_empty()
        );
        assert!(draw_base(&[], pot, &mix(0), 5, ACTIVE).unwrap().is_empty());
    }

    #[test]
    fn new_addresses_need_the_higher_minimum() {
        // 0.1 TOY, one ticket: enough for an active address, not for a new.
        let pot = Amount::from_units(100_000_000_000_000_000);
        assert_eq!(
            draw_base(&tickets(1), pot, &mix(0), 5, ACTIVE)
                .unwrap()
                .len(),
            1
        );
        assert!(
            draw_base(&tickets(1), pot, &mix(0), 5, |_| false)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn draw_is_deterministic_and_depends_on_the_mix() {
        let a = draw_base(&tickets(20), Amount::from_toy(1), &mix(3), 7, ACTIVE).unwrap();
        let b = draw_base(&tickets(20), Amount::from_toy(1), &mix(3), 7, ACTIVE).unwrap();
        assert_eq!(a, b);
        let c = draw_base(&tickets(20), Amount::from_toy(1), &mix(4), 7, ACTIVE).unwrap();
        assert_ne!(a, c);
        assert_ne!(next_mix(&mix(1), &mix(2)), next_mix(&mix(2), &mix(1)));
    }

    #[test]
    fn expected_reward_is_proportional_to_tickets() {
        // Owner 0 holds 3 of 4 tickets: about 75 % of the base pot.
        let mut t = tickets(4);
        for entry in t.iter_mut().take(3) {
            entry.1 = owner(0);
        }
        let pot = Amount::from_toy(1);
        let mut got = 0u128;
        for i in 0..2_000 {
            for (w, p) in draw_base(&t, pot, &mix(i), 1, ACTIVE).unwrap() {
                if w == owner(0) {
                    got += p.units();
                }
            }
        }
        let share = got * 1_000 / (2_000 * pot.units());
        assert!((720..780).contains(&share), "{share}");
    }

    #[test]
    fn jackpot_winners_and_payout_ranges() {
        let owners: Vec<Address> = (0..2_000).map(owner).collect();
        let pot = Amount::from_toy(1_000);
        for (jackpot, (low, high)) in [
            (Jackpot::Funke, (1, 4)),
            (Jackpot::Welle, (8, 64)),
            (Jackpot::Gezeiten, (64, 512)),
        ] {
            let mut counts = std::collections::BTreeSet::new();
            for i in 0..300 {
                let prizes = draw_jackpot(
                    jackpot,
                    &owners,
                    pot,
                    Amount::from_toy(10_000),
                    &mix(i),
                    9,
                    ACTIVE,
                )
                .unwrap();
                let n = prizes.len() as u64;
                assert!((low..=high).contains(&n), "{jackpot:?} {n}");
                counts.insert(n);
                let paid = total(&prizes);
                assert!(paid <= pot);
                assert!(paid >= Amount::from_toy(399), "{jackpot:?} {paid:?}");
            }
            assert!(counts.contains(&low) || counts.len() > 2, "{jackpot:?}");
        }
        // The cap limits the payout.
        let cap = Amount::from_toy(5);
        let prizes = draw_jackpot(Jackpot::Welle, &owners, pot, cap, &mix(1), 9, ACTIVE).unwrap();
        assert!(total(&prizes) <= cap);
        // Fewer tickets than winners: each ticket wins at most once.
        let few: Vec<Address> = (0..10).map(owner).collect();
        let prizes = draw_jackpot(Jackpot::Gezeiten, &few, pot, pot, &mix(2), 9, ACTIVE).unwrap();
        assert_eq!(prizes.len(), 10);
        assert!(
            draw_jackpot(Jackpot::Funke, &[], pot, pot, &mix(2), 9, ACTIVE)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn factors_follow_the_spec() {
        let inflow = Amount::from_toy(1);
        let f = |pot: u64| level_factor(Jackpot::Funke, Amount::from_toy(pot), inflow).unwrap();
        assert_eq!(f(240), 1_000_000, "on target");
        assert_eq!(f(480), 2_000_000);
        assert_eq!(f(10), 250_000, "nearly empty: 0.25");
        assert_eq!(f(10_000), 3_000_000, "full: at most 3");
        // Tide: ebb ×1.8, flood ×0.2 (bounded to 0.5).
        let one = Ppm::ONE;
        assert_eq!(
            special_factor(Jackpot::Gezeiten, Ppm::new(750_000), one),
            1_800_000
        );
        assert_eq!(special_factor(Jackpot::Gezeiten, one, one), 1_000_000);
        assert_eq!(
            special_factor(Jackpot::Gezeiten, Ppm::new(1_250_000), one),
            500_000
        );
        // Participation: ×1.3 from 95 %, ×0.8 below 80 %; Gezeiten ignores it.
        assert_eq!(
            special_factor(Jackpot::Funke, one, Ppm::new(950_000)),
            1_300_000
        );
        assert_eq!(
            special_factor(Jackpot::Funke, one, Ppm::new(900_000)),
            1_000_000
        );
        assert_eq!(
            special_factor(Jackpot::Funke, one, Ppm::new(799_999)),
            800_000
        );
        assert_eq!(
            special_factor(Jackpot::Welle, Ppm::new(750_000), Ppm::new(990_000)),
            2_340_000
        );
        assert_eq!(
            special_factor(Jackpot::Gezeiten, Ppm::new(750_000), Ppm::new(100_000)),
            1_800_000
        );
    }

    #[test]
    fn jackpots_fire_about_once_per_interval() {
        let inflow = Amount::from_toy(1);
        let inputs = TriggerInputs {
            pot: Amount::from_toy(240),
            inflow,
            tide: Ppm::ONE,
            participation: Ppm::new(900_000),
        };
        let fired = (0..240_000u64)
            .filter(|h| {
                trigger(Jackpot::Funke, &inputs, &mix(7), *h)
                    .unwrap()
                    .is_some()
            })
            .count();
        // Expected 1,000 (h = 1/240).
        assert!((880..1_120).contains(&fired), "{fired}");
        // Forced at four times the target level.
        let full = TriggerInputs {
            pot: Amount::from_toy(960),
            ..inputs
        };
        assert_eq!(
            trigger(Jackpot::Funke, &full, &mix(1), 1).unwrap(),
            Some(Trigger::Forced)
        );
        // An empty pot never fires.
        let empty = TriggerInputs {
            pot: Amount::ZERO,
            ..inputs
        };
        assert!((0..10_000).all(|h| {
            trigger(Jackpot::Funke, &empty, &mix(1), h)
                .unwrap()
                .is_none()
        }));
    }

    #[test]
    fn ticket_limit_scales_with_the_block_gap() {
        assert_eq!(ticket_limit(60), 32);
        assert_eq!(ticket_limit(600), 32);
        assert_eq!(ticket_limit(30), 16);
        assert_eq!(ticket_limit(10), 8);
        assert_eq!(ticket_limit(0), 8);
    }

    #[test]
    fn milestone_delay_stays_inside_the_next_epoch() {
        for i in 0..500 {
            let d = milestone_delay(Jackpot::Gezeiten, &mix(i), 1_000_000, 1_440);
            assert!((1..=1_440).contains(&d));
        }
    }

    #[test]
    fn threshold_adjusts_towards_target_within_factor_4() {
        let t = 1u128 << 100;
        // On target (16 per block): unchanged.
        assert_eq!(next_threshold(t, 16 * 1_440, 1_440), t);
        // Twice as many tickets: twice as hard.
        assert_eq!(next_threshold(t, 32 * 1_440, 1_440), t / 2);
        // Ten times as many: at most 4 times harder.
        assert_eq!(next_threshold(t, 160 * 1_440, 1_440), t / 4);
        // Half as many: twice as easy.
        assert_eq!(next_threshold(t, 8 * 1_440, 1_440), t * 2);
        // None: 4 times easier, saturating at the top.
        assert_eq!(next_threshold(t, 0, 1_440), t * 4);
        assert_eq!(next_threshold(u128::MAX, 0, 1_440), u128::MAX);
        // Never zero.
        assert_eq!(next_threshold(1, 1_000_000, 1), 1);
    }
}
