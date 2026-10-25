//! Fees, free quota and refund pool (spec 7.2, decisions 106, 117, 118,
//! 120, 145 to 147).
//!
//! - **Required fee** of a paid transaction: `base fee per byte × size ×
//!   2^d`, where `d` grows by one for every 10 further transactions of the
//!   same sender in the same block, at most 10 doublings (factor 1,024,
//!   decision 117). From the 1,000th transaction of a sender in an epoch
//!   the base fee counts twice (heavy users, decision 146).
//! - **Charged fee**: `min(max_fee, required + max_tip)`; a transaction
//!   with `max_fee < required` is invalid. Ordering by fee per byte uses
//!   the charged fee (M4).
//! - **Split** of a paid fee: 40 % staking pot, 20 % proposer, the rest
//!   (40 % plus rounding) refund pool (decision 147).
//! - **Base fee** moves by at most 1/8 per block towards half-full blocks
//!   (EIP-1559), between 0.000001 and 1 TOY per byte.
//! - **Free quota** `Q = min(21 + 28 × (1 − utilization), λ)`, utilization
//!   = bytes of all transactions of the previous epoch ÷ (blocks × 128 KB)
//!   (decisions 118, 145). The water level `λ` is the largest integer ≥ 1
//!   for which `Σ min(free transactions of an account in the previous
//!   epoch, λ)` stays within 90 % of the free space of the previous epoch
//!   (blocks × 64 KB ÷ 150 bytes); with enough space there is no `λ`. New
//!   accounts reach the full quota linearly over 7 epochs after the age
//!   of 100 blocks.
//! - **Refund pool cap** (decision 120): the pool keeps at most 30 epochs
//!   of the largest possible refund demand, `30 × blocks × 64 KB × base
//!   fee`; the surplus goes to the staking pot (decision 138).

use toy_state::FeeSplit;
use toy_types::params::{
    BASE_FEE_CHANGE_DENOMINATOR, BASE_FEE_MAX_PER_BYTE, BASE_FEE_MIN_PER_BYTE,
    BASE_FEE_TARGET_BYTES, EPOCH_MIN_BLOCKS, FEE_SHARE_PROPOSER, FEE_SHARE_STAKING,
    HEAVY_USER_FEE_FACTOR, HEAVY_USER_THRESHOLD, MAX_BLOCK_BYTES, MAX_FREE_BYTES_PER_BLOCK,
    PROGRESSIVE_FEE_MAX_DOUBLINGS, PROGRESSIVE_FEE_STEP, QUOTA_MAX, QUOTA_MIN,
    QUOTA_MIN_ACCOUNT_AGE_BLOCKS, QUOTA_NOMINAL_TX_BYTES, QUOTA_RAMP_BLOCKS, QUOTA_WATER_LEVEL,
    REFUND_BUDGET_DIVISOR, REFUND_POOL_CAP_EPOCHS,
};
use toy_types::{Amount, ArithmeticError};

use crate::error::TxError;

/// Factor of the progressive fee for the `k`-th transaction (1-based) of
/// a sender in a block: `2^min((k − 1) / 10, 10)`.
#[must_use]
pub fn progressive_factor(k: u64) -> u128 {
    let doublings = k.saturating_sub(1) / PROGRESSIVE_FEE_STEP;
    let doublings = u32::try_from(doublings)
        .unwrap_or(u32::MAX)
        .min(PROGRESSIVE_FEE_MAX_DOUBLINGS);
    1u128 << doublings
}

/// `true` for the `n`-th transaction (1-based) of a sender in an epoch
/// from which the heavy-user rule applies (decision 146).
#[must_use]
pub const fn is_heavy_use(n: u64) -> bool {
    n >= HEAVY_USER_THRESHOLD
}

/// Required fee of a paid transaction; `heavy` doubles the base fee.
///
/// # Errors
/// [`ArithmeticError::Overflow`] (not reachable with the bounded base fee
/// and block size).
pub fn required_fee(
    base_fee_per_byte: Amount,
    size: u64,
    k_in_block: u64,
    heavy: bool,
) -> Result<Amount, ArithmeticError> {
    let base = if heavy {
        base_fee_per_byte.checked_mul(HEAVY_USER_FEE_FACTOR)?
    } else {
        base_fee_per_byte
    };
    base.checked_mul(u128::from(size))?
        .checked_mul(progressive_factor(k_in_block))
}

/// Fee actually charged: `min(max_fee, required + max_tip)`.
///
/// # Errors
/// [`TxError::FeeTooLow`] if `max_fee < required`.
pub fn charged_fee(max_fee: Amount, max_tip: Amount, required: Amount) -> Result<Amount, TxError> {
    if max_fee < required {
        return Err(TxError::FeeTooLow { required, max_fee });
    }
    let offered = required.checked_add(max_tip)?;
    Ok(max_fee.min(offered))
}

/// Splits a paid fee 40 / 20 / 40; rounding remainders go to the pool.
///
/// # Errors
/// [`ArithmeticError`] (not reachable).
pub fn split_fee(fee: Amount) -> Result<FeeSplit, ArithmeticError> {
    let staking = fee.mul_ppm(FEE_SHARE_STAKING)?;
    let proposer = fee.mul_ppm(FEE_SHARE_PROPOSER)?;
    let refund_pool = fee.checked_sub(staking)?.checked_sub(proposer)?;
    Ok(FeeSplit {
        staking,
        proposer,
        refund_pool,
    })
}

/// Base fee for the next block after a block that used `used_bytes`.
///
/// # Errors
/// [`ArithmeticError`] (not reachable with the bounds).
pub fn next_base_fee(current: Amount, used_bytes: u64) -> Result<Amount, ArithmeticError> {
    let target = u128::from(BASE_FEE_TARGET_BYTES);
    let used = u128::from(used_bytes);
    let denominator = target
        .checked_mul(BASE_FEE_CHANGE_DENOMINATOR)
        .ok_or(ArithmeticError::Overflow)?;
    let next = if used > target {
        let excess = used.checked_sub(target).ok_or(ArithmeticError::Underflow)?;
        let delta = current.mul_div_floor(excess, denominator)?;
        // Always rise by at least one unit when above target (EIP-1559).
        current.checked_add(delta.max(Amount::from_units(1)))?
    } else {
        // At target the shortfall is zero and the fee stays.
        let shortfall = target.checked_sub(used).ok_or(ArithmeticError::Underflow)?;
        let delta = current.mul_div_floor(shortfall, denominator)?;
        current.checked_sub(delta)?
    };
    Ok(next.clamp(BASE_FEE_MIN_PER_BYTE, BASE_FEE_MAX_PER_BYTE))
}

/// Nominal fee of a free transaction (what the refund pool covers).
///
/// # Errors
/// [`ArithmeticError::Overflow`] (not reachable).
pub fn nominal_fee(base_fee_per_byte: Amount, size: u64) -> Result<Amount, ArithmeticError> {
    base_fee_per_byte.checked_mul(u128::from(size))
}

/// Utilization part of the free quota: `21 + 28 × (1 − utilization)`,
/// rounded down (decisions 118, 145). `blocks == 0` (before the first
/// epoch) counts as zero utilization.
#[must_use]
pub fn base_quota(prev_epoch_tx_bytes: u64, prev_epoch_blocks: u64) -> u64 {
    let capacity = u128::from(prev_epoch_blocks).saturating_mul(u128::from(MAX_BLOCK_BYTES));
    if capacity == 0 {
        return QUOTA_MAX;
    }
    let used = u128::from(prev_epoch_tx_bytes).min(capacity);
    let span = u128::from(QUOTA_MAX.saturating_sub(QUOTA_MIN));
    let extra = span
        .saturating_mul(capacity.saturating_sub(used))
        .checked_div(capacity)
        .unwrap_or(0);
    QUOTA_MIN
        .saturating_add(u64::try_from(extra).unwrap_or(0))
        .clamp(QUOTA_MIN, QUOTA_MAX)
}

/// Water level `λ` (decision 145) from the free transactions per account
/// of the previous epoch: the largest integer `λ ≥ 1` with
/// `Σ min(c, λ) ≤ 90 % × blocks × 64 KB ÷ 150 bytes`. `None` if all
/// counts fit without a limit.
#[must_use]
pub fn water_level(counts: &[u64], prev_epoch_blocks: u64) -> Option<u64> {
    let space = u128::from(prev_epoch_blocks)
        .saturating_mul(u128::from(MAX_FREE_BYTES_PER_BLOCK))
        .checked_div(u128::from(QUOTA_NOMINAL_TX_BYTES))
        .unwrap_or(0);
    let room = space
        .saturating_mul(u128::from(QUOTA_WATER_LEVEL.get()))
        .checked_div(1_000_000)
        .unwrap_or(0);
    let filled = |level: u64| -> u128 {
        counts
            .iter()
            .map(|c| u128::from((*c).min(level)))
            .fold(0u128, u128::saturating_add)
    };
    let highest = counts.iter().copied().max().unwrap_or(0);
    if filled(highest) <= room {
        return None;
    }
    // filled() rises with the level: binary search for the last level
    // that fits. At least 1, even if that does not fit.
    let (mut low, mut high) = (1u64, highest);
    while low < high {
        let mid = low.saturating_add(high.saturating_sub(low).saturating_add(1) / 2);
        if filled(mid) <= room {
            low = mid;
        } else {
            high = mid.saturating_sub(1);
        }
    }
    Some(low)
}

/// Free quota of an epoch: the utilization part, limited by the water
/// level.
#[must_use]
pub fn quota(base: u64, water_level: Option<u64>) -> u64 {
    water_level.map_or(base, |level| base.min(level))
}

/// Free quota of an account that is `age_blocks` old: nothing below 100
/// blocks, then rising linearly over 7 epochs (10,080 blocks) to the full
/// quota, at least one transaction.
#[must_use]
pub fn account_quota(epoch_quota: u64, age_blocks: u64) -> u64 {
    if age_blocks < QUOTA_MIN_ACCOUNT_AGE_BLOCKS {
        return 0;
    }
    let ramp = age_blocks
        .saturating_sub(QUOTA_MIN_ACCOUNT_AGE_BLOCKS)
        .min(QUOTA_RAMP_BLOCKS);
    let share = u128::from(epoch_quota)
        .saturating_mul(u128::from(ramp))
        .checked_div(u128::from(QUOTA_RAMP_BLOCKS))
        .unwrap_or(0);
    u64::try_from(share)
        .unwrap_or(epoch_quota)
        .clamp(1.min(epoch_quota), epoch_quota)
}

/// Refund budget of an epoch: one thirtieth of the pool at its start.
///
/// # Errors
/// [`ArithmeticError`] (not reachable).
pub fn refund_budget(pool: Amount) -> Result<Amount, ArithmeticError> {
    pool.checked_div(REFUND_BUDGET_DIVISOR)
}

/// Largest pool the refunds can need (decision 120): 30 epochs of free
/// blocks at the current base fee. `prev_epoch_blocks` counts at least
/// 1,440 blocks.
///
/// # Errors
/// [`ArithmeticError`] (not reachable with the bounds).
pub fn refund_pool_cap(
    base_fee_per_byte: Amount,
    prev_epoch_blocks: u64,
) -> Result<Amount, ArithmeticError> {
    let blocks = u128::from(prev_epoch_blocks.max(EPOCH_MIN_BLOCKS));
    base_fee_per_byte
        .checked_mul(u128::from(MAX_FREE_BYTES_PER_BLOCK))?
        .checked_mul(blocks)?
        .checked_mul(REFUND_POOL_CAP_EPOCHS)
}

#[cfg(test)]
#[allow(clippy::arithmetic_side_effects)]
mod tests {
    use super::*;

    #[test]
    fn progressive_factor_doubles_every_10_and_stops_at_1024() {
        assert_eq!(progressive_factor(1), 1);
        assert_eq!(progressive_factor(10), 1);
        assert_eq!(progressive_factor(11), 2);
        assert_eq!(progressive_factor(20), 2);
        assert_eq!(progressive_factor(21), 4);
        assert_eq!(progressive_factor(100), 512);
        assert_eq!(progressive_factor(101), 1024);
        assert_eq!(progressive_factor(10_000), 1024);
        assert_eq!(progressive_factor(u64::MAX), 1024);
    }

    #[test]
    fn required_and_charged_fee() {
        let base = BASE_FEE_MIN_PER_BYTE;
        let one = required_fee(base, 150, 1, false).unwrap();
        // 150 bytes at 0.000001 TOY = 0.00015 TOY.
        assert_eq!(one, Amount::from_units(150_000_000_000_000));
        assert_eq!(
            required_fee(base, 150, 11, false).unwrap(),
            one.checked_mul(2).unwrap()
        );
        assert_eq!(
            required_fee(base, 150, 101, false).unwrap(),
            one.checked_mul(1024).unwrap()
        );
        // Heavy users pay twice the base fee.
        assert_eq!(
            required_fee(base, 150, 1, true).unwrap(),
            one.checked_mul(2).unwrap()
        );
        assert!(!is_heavy_use(999));
        assert!(is_heavy_use(1_000));
        // Charged: required plus tip, never above max_fee.
        let tip = Amount::from_units(7);
        assert_eq!(
            charged_fee(one.checked_mul(2).unwrap(), tip, one).unwrap(),
            one.checked_add(tip).unwrap()
        );
        let tight = one.checked_add(Amount::from_units(3)).unwrap();
        assert_eq!(charged_fee(tight, tip, one).unwrap(), tight);
        assert!(matches!(
            charged_fee(one.checked_sub(Amount::from_units(1)).unwrap(), tip, one),
            Err(TxError::FeeTooLow { .. })
        ));
    }

    #[test]
    fn split_is_exact_with_remainder_to_pool() {
        for fee in [0u128, 1, 9, 10, 101, 999_999, 150_000_000_000_000] {
            let s = split_fee(Amount::from_units(fee)).unwrap();
            assert_eq!(s.total().unwrap(), Amount::from_units(fee));
            assert_eq!(s.staking.units(), fee * 4 / 10);
            assert_eq!(s.proposer.units(), fee * 2 / 10);
        }
        let s = split_fee(Amount::from_units(101)).unwrap();
        assert_eq!(
            (s.staking.units(), s.proposer.units(), s.refund_pool.units()),
            (40, 20, 41)
        );
    }

    #[test]
    fn base_fee_follows_eip1559() {
        let base = Amount::from_units(8_000_000_000_000);
        // At target: unchanged.
        assert_eq!(next_base_fee(base, BASE_FEE_TARGET_BYTES).unwrap(), base);
        // Full block: +1/8.
        assert_eq!(
            next_base_fee(base, MAX_BLOCK_BYTES).unwrap(),
            Amount::from_units(9_000_000_000_000)
        );
        // Empty block: −1/8.
        assert_eq!(
            next_base_fee(base, 0).unwrap(),
            Amount::from_units(7_000_000_000_000)
        );
        // Never below the minimum, never above the maximum.
        assert_eq!(
            next_base_fee(BASE_FEE_MIN_PER_BYTE, 0).unwrap(),
            BASE_FEE_MIN_PER_BYTE
        );
        assert_eq!(
            next_base_fee(BASE_FEE_MAX_PER_BYTE, MAX_BLOCK_BYTES).unwrap(),
            BASE_FEE_MAX_PER_BYTE
        );
        // 200 full blocks in a row stay bounded and never overflow.
        let mut fee = BASE_FEE_MIN_PER_BYTE;
        for _ in 0..200 {
            fee = next_base_fee(fee, MAX_BLOCK_BYTES).unwrap();
        }
        assert_eq!(fee, BASE_FEE_MAX_PER_BYTE);
    }

    #[test]
    fn quota_depends_on_utilization() {
        let blocks = 1_440;
        let cap = blocks * MAX_BLOCK_BYTES;
        assert_eq!(base_quota(0, 0), 49, "before the first epoch");
        assert_eq!(base_quota(0, blocks), 49, "empty chain");
        assert_eq!(base_quota(cap / 4, blocks), 42);
        assert_eq!(base_quota(cap / 2, blocks), 35);
        assert_eq!(base_quota(cap, blocks), 21);
        assert_eq!(base_quota(u64::MAX, blocks), 21, "never below 21");
        // Rounds down: 1 byte of use already costs one transaction.
        assert_eq!(base_quota(1, blocks), 48);
    }

    #[test]
    fn water_level_limits_heavy_free_users_first() {
        // One epoch: 1,440 × 65,536 ÷ 150 = 629,145 free transactions,
        // 90 % = 566,230.
        assert_eq!(water_level(&[], 1_440), None);
        assert_eq!(water_level(&[49; 10_000], 1_440), None, "enough room");
        // 20,000 accounts used 49 each (980,000 > 566,230): λ = 28,
        // because 20,000 × 28 = 560,000 fits and × 29 does not.
        assert_eq!(water_level(&[49; 20_000], 1_440), Some(28));
        // Few-users stay free: 100,000 accounts with 3 and 10,000 with 49.
        let mut counts = vec![3u64; 100_000];
        counts.extend([49u64; 10_000]);
        let level = water_level(&counts, 1_440).unwrap();
        assert_eq!(level, 26);
        assert!(level >= 3);
        // Never below 1.
        assert_eq!(water_level(&[5; 1_000_000], 1_440), Some(1));
        assert_eq!(quota(35, Some(28)), 28);
        assert_eq!(quota(35, None), 35);
        assert_eq!(quota(21, Some(40)), 21);
    }

    #[test]
    fn new_accounts_grow_into_the_quota() {
        assert_eq!(account_quota(49, 99), 0, "too young");
        assert_eq!(account_quota(49, 100), 1, "at least one");
        assert_eq!(account_quota(49, 100 + 5_040), 24, "half way");
        assert_eq!(account_quota(49, 100 + 10_080), 49);
        assert_eq!(account_quota(49, u64::MAX), 49);
        assert_eq!(account_quota(0, 50_000), 0);
    }

    #[test]
    fn refund_budget_and_pool_cap() {
        assert_eq!(
            refund_budget(Amount::from_toy(3_000)).unwrap(),
            Amount::from_toy(100)
        );
        // 30 × 1,440 blocks × 65,536 bytes × 10^12 units ≈ 2,831 TOY.
        let cap = refund_pool_cap(BASE_FEE_MIN_PER_BYTE, 1_440).unwrap();
        assert_eq!(cap, Amount::from_units(2_831_155_200_000_000_000_000));
        // Fewer blocks count as 1,440; more blocks raise the cap.
        assert_eq!(refund_pool_cap(BASE_FEE_MIN_PER_BYTE, 10).unwrap(), cap);
        assert_eq!(
            refund_pool_cap(BASE_FEE_MIN_PER_BYTE, 2_880).unwrap(),
            cap.checked_mul(2).unwrap()
        );
        // At the cap the budget equals one full epoch of free blocks.
        let budget = refund_budget(cap).unwrap();
        assert_eq!(
            budget,
            nominal_fee(BASE_FEE_MIN_PER_BYTE, MAX_FREE_BYTES_PER_BLOCK)
                .unwrap()
                .checked_mul(1_440)
                .unwrap()
        );
    }
}
