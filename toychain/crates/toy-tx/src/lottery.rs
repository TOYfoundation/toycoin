//! Work lottery and ToyWork difficulty (spec 9, decisions 106, 107, 119).
//!
//! Tickets committed in block N are drawn with the randomness of block
//! N + 2 (VRF of the proposer, M4), which the builder of block N cannot
//! know. Winners per draw: `min(tickets, pot ÷ 0.2 TOY, 64)`; the pot is
//! split equally, the rounding rest stays in the pot for the next block.
//! A ticket can win at most once; an address with more tickets has
//! proportionally better chances, so the expected reward is proportional
//! to the work.

use toy_crypto::hash::blake3_derive;
use toy_types::params::{
    LOTTERY_MAX_WINNERS_PER_BLOCK, LOTTERY_MIN_PRIZE, TICKETS_TARGET_PER_BLOCK,
    TOYWORK_MAX_ADJUST_FACTOR,
};
use toy_types::{Address, Amount, ArithmeticError, Hash, Height};

const DRAW_CONTEXT: &str = "ToyChain 2026-10 work lottery draw v1";

/// Draws the winners among `owners` (one entry per valid ticket, in
/// block order) and returns the prizes.
///
/// # Errors
/// [`ArithmeticError`] (not reachable).
pub fn draw(
    owners: &[Address],
    pot: Amount,
    randomness: &Hash,
    committed_at: Height,
) -> Result<Vec<(Address, Amount)>, ArithmeticError> {
    let by_pot = pot.checked_div(LOTTERY_MIN_PRIZE.units())?.units();
    let tickets = u128::try_from(owners.len()).map_err(|_| ArithmeticError::Overflow)?;
    let winners = tickets
        .min(by_pot)
        .min(u128::from(LOTTERY_MAX_WINNERS_PER_BLOCK));
    if winners == 0 {
        return Ok(Vec::new());
    }
    let (prize, _rest) = pot.split_even(winners)?;
    let mut remaining: Vec<Address> = owners.to_vec();
    let mut prizes = Vec::new();
    for i in 0..winners {
        let len = u128::try_from(remaining.len()).map_err(|_| ArithmeticError::Overflow)?;
        let mut data = Vec::with_capacity(56);
        data.extend_from_slice(randomness.as_bytes());
        data.extend_from_slice(&committed_at.to_be_bytes());
        data.extend_from_slice(&i.to_be_bytes());
        let hash = blake3_derive(DRAW_CONTEXT, &data);
        let mut head = [0u8; 16];
        if let Some(src) = hash.as_bytes().get(..16) {
            head.copy_from_slice(src);
        }
        // Bias of the modulo is below 2^-120 for at most 16 tickets.
        let pick = u128::from_be_bytes(head)
            .checked_rem(len)
            .ok_or(ArithmeticError::DivisionByZero)?;
        let index = usize::try_from(pick).map_err(|_| ArithmeticError::Overflow)?;
        if index >= remaining.len() {
            return Err(ArithmeticError::Overflow);
        }
        prizes.push((remaining.swap_remove(index), prize));
    }
    Ok(prizes)
}

/// ToyWork threshold for the next epoch: moves the number of valid
/// tickets towards 8 per block, changing by at most a factor of 4 per
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
#[allow(clippy::arithmetic_side_effects)]
mod tests {
    use super::*;

    fn owners(n: u8) -> Vec<Address> {
        (0..n)
            .map(|i| {
                let mut b = [0u8; 20];
                b[19] = i;
                Address::from_bytes(b)
            })
            .collect()
    }

    #[test]
    fn winner_count_rules() {
        let r = Hash::from_bytes([9; 32]);
        // No tickets: nobody wins, pot stays.
        assert!(draw(&[], Amount::from_toy(100), &r, 1).unwrap().is_empty());
        // Pot below the minimum prize: nobody wins.
        let small = LOTTERY_MIN_PRIZE
            .checked_sub(Amount::from_units(1))
            .unwrap();
        assert!(draw(&owners(3), small, &r, 1).unwrap().is_empty());
        // Pot of 0.5 TOY: at most 2 prizes of 0.25 TOY.
        let prizes = draw(
            &owners(5),
            Amount::from_units(500_000_000_000_000_000),
            &r,
            1,
        )
        .unwrap();
        assert_eq!(prizes.len(), 2);
        assert!(prizes.iter().all(|(_, p)| *p >= LOTTERY_MIN_PRIZE));
        // Large pot: one prize per ticket.
        let prizes = draw(&owners(16), Amount::from_toy(1_000), &r, 1).unwrap();
        assert_eq!(prizes.len(), 16);
        // 1,000 TOY / 16 = 62.5 TOY each.
        assert_eq!(prizes[0].1, Amount::from_units(62_500_000_000_000_000_000));
    }

    #[test]
    fn never_more_than_64_winners() {
        let r = Hash::from_bytes([1; 32]);
        let prizes = draw(&owners(200), Amount::from_toy(1_000_000), &r, 1).unwrap();
        assert_eq!(prizes.len(), 64);
    }

    #[test]
    fn each_ticket_wins_at_most_once_and_draw_is_deterministic() {
        let r = Hash::from_bytes([3; 32]);
        let a = draw(&owners(10), Amount::from_toy(1), &r, 7).unwrap();
        let b = draw(&owners(10), Amount::from_toy(1), &r, 7).unwrap();
        assert_eq!(a, b);
        let mut won: Vec<Address> = a.iter().map(|(w, _)| *w).collect();
        won.sort();
        won.dedup();
        assert_eq!(won.len(), a.len());
        // Other randomness, other winners (with overwhelming probability).
        let c = draw(
            &owners(10),
            Amount::from_toy(1),
            &Hash::from_bytes([4; 32]),
            7,
        )
        .unwrap();
        assert_ne!(a, c);
    }

    #[test]
    fn chances_are_proportional_to_tickets() {
        // Address 0 holds 3 of 4 tickets: it should win about 75 % of
        // single-winner draws.
        let mut tickets = vec![owners(1)[0]; 3];
        tickets.push(owners(2)[1]);
        let pot = LOTTERY_MIN_PRIZE;
        let mut wins = 0u32;
        for i in 0u32..2_000 {
            let mut seed = [0u8; 32];
            seed[..4].copy_from_slice(&i.to_be_bytes());
            let prizes = draw(&tickets, pot, &Hash::from_bytes(seed), 1).unwrap();
            if prizes[0].0 == tickets[0] {
                wins += 1;
            }
        }
        assert!((1_400..1_600).contains(&wins), "{wins}");
    }

    #[test]
    fn threshold_adjusts_towards_target_within_factor_4() {
        let t = 1u128 << 100;
        // On target: unchanged.
        assert_eq!(next_threshold(t, 8 * 1_440, 1_440), t);
        // Twice as many tickets: twice as hard.
        assert_eq!(next_threshold(t, 16 * 1_440, 1_440), t / 2);
        // Ten times as many: at most 4 times harder.
        assert_eq!(next_threshold(t, 80 * 1_440, 1_440), t / 4);
        // Half as many: twice as easy.
        assert_eq!(next_threshold(t, 4 * 1_440, 1_440), t * 2);
        // None: 4 times easier, saturating at the top.
        assert_eq!(next_threshold(t, 0, 1_440), t * 4);
        assert_eq!(next_threshold(u128::MAX, 0, 1_440), u128::MAX);
        // Never zero.
        assert_eq!(next_threshold(1, 1_000_000, 1), 1);
    }
}
