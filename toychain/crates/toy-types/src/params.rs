//! Protocol constants (spec section 14, "Parameter-Startwerte").
//!
//! Values marked *constitution* are fixed forever (spec section 13): no
//! update and no future vote may change them. All other values are start
//! assumptions for the testnet and may be tuned after measurements.

use crate::amount::Amount;
use crate::ppm::Ppm;

// ---- Coin (constitution) ----------------------------------------------

/// Decimal places of TOY (constitution).
pub const DECIMALS: u32 = 18;

/// Smallest units per TOY: 10^18 (constitution).
pub const UNITS_PER_TOY: u128 = 1_000_000_000_000_000_000;

/// Supply cap: 100,000,000 TOY = 10^26 smallest units (constitution).
/// The sum of all emissions never exceeds this value.
pub const MAX_SUPPLY: Amount = Amount::from_units(100_000_000 * UNITS_PER_TOY);

// ---- Time (spec 5.2, decision 58) -------------------------------------

/// Target block interval in seconds.
pub const TARGET_BLOCK_SECONDS: u64 = 60;

/// A block timestamp must be at least this many seconds after its parent.
pub const MIN_BLOCK_GAP_SECONDS: u64 = 10;

/// A block timestamp may be at most this far ahead of the verifier's clock.
pub const MAX_FUTURE_DRIFT_SECONDS: u64 = 15;

/// An epoch has at least this many blocks (the genesis block is not counted).
pub const EPOCH_MIN_BLOCKS: u64 = 1_440;

/// An epoch lasts at least this many seconds (24 hours).
pub const EPOCH_MIN_SECONDS: u64 = 86_400;

/// Seconds per plan day.
pub const SECONDS_PER_DAY: u64 = 86_400;

/// Plan time (decision 110): each block gap counts at most this many seconds.
pub const PLAN_GAP_CAP_SECONDS: u64 = 600;

// ---- Emission (spec 9, 9.0, 9.1) --------------------------------------

/// Base emission rate k0 = 0.0505 % per epoch = 505 / 1,000,000.
pub const K0: Ppm = Ppm::new(505);

/// Lower bound of the emission factor m (0.5).
pub const FACTOR_MIN: Ppm = Ppm::new(500_000);

/// Upper bound of the emission factor m (2.0).
pub const FACTOR_MAX: Ppm = Ppm::new(2_000_000);

/// Maximum change of m per epoch (0.05).
pub const FACTOR_MAX_STEP: Ppm = Ppm::new(50_000);

/// The emission of an epoch is spread over its first this many blocks.
pub const EMISSION_BLOCKS_PER_EPOCH: u64 = 1_440;

/// Block capacity used for the usage measurement `u` (transactions per block).
pub const USAGE_CAPACITY_PER_BLOCK: u64 = 1_000;

/// Share of each block emission for consensus (stake tiers 2 to 4).
pub const CONSENSUS_SHARE: Ppm = Ppm::new(500_000);

/// Share of each block emission for the work lottery.
pub const WORK_SHARE: Ppm = Ppm::new(400_000);

/// Share of each block emission for the refund pool (plus rounding remainders).
pub const REFUND_POOL_SHARE: Ppm = Ppm::new(100_000);

const _: () = assert!(
    CONSENSUS_SHARE.get() + WORK_SHARE.get() + REFUND_POOL_SHARE.get() == Ppm::SCALE,
    "emission shares must add up to 100 %"
);

// ---- Bootstrap phase (spec 5.3, decisions 70, 78, 110, 111) -----------

/// Number of start validators named in the genesis block.
pub const START_VALIDATORS: u32 = 21;

/// Share of the emission paid to start validators on plan day 0 (50 %).
pub const START_REWARD_INITIAL_SHARE: Ppm = Ppm::new(500_000);

/// The start validator reward falls linearly to zero over this many plan days.
pub const START_REWARD_DAYS: u64 = 180;

/// Start validator rewards stay bonded for this many epochs.
pub const START_REWARD_LOCK_EPOCHS: u64 = 365;

/// Earliest handover out of the bootstrap phase, in plan days.
pub const HANDOVER_MIN_DAYS: u64 = 90;

/// Minimum number of new validators for the handover.
pub const HANDOVER_MIN_NEW_VALIDATORS: u32 = 21;

/// Stake of new validators counts for the handover only after this many epochs.
pub const HANDOVER_MIN_BONDED_EPOCHS: u64 = 30;

const _: () = assert!(
    START_REWARD_INITIAL_SHARE.get() <= CONSENSUS_SHARE.get(),
    "start validators can never get more than the consensus share"
);

// ---- Stake and committee (spec 4, 5, 14) -------------------------------

/// Maximum committee size.
pub const COMMITTEE_MAX: u32 = 128;

/// Stake of tier 2 (light stake).
pub const STAKE_TIER_2: Amount = Amount::from_toy(10);

/// Stake of tier 3 (full node).
pub const STAKE_TIER_3: Amount = Amount::from_toy(100);

/// Stake of tier 4 (validator).
pub const STAKE_TIER_4: Amount = Amount::from_toy(1_000);

/// Unbonding period in epochs.
pub const UNBONDING_EPOCHS: u64 = 21;

// ---- Accounts and lottery (spec 6.1, 9) --------------------------------

/// Activation deposit on the first incoming transfer: 0.1 TOY.
pub const ACTIVATION_DEPOSIT: Amount = Amount::from_units(UNITS_PER_TOY / 10);

/// Lowest value the activation deposit may ever be lowered to: 0.01 TOY.
pub const ACTIVATION_DEPOSIT_FLOOR: Amount = Amount::from_units(UNITS_PER_TOY / 100);

/// Minimum prize of the work lottery: 0.2 TOY.
pub const LOTTERY_MIN_PRIZE: Amount = Amount::from_units(UNITS_PER_TOY / 5);

/// Maximum number of lottery winners per block.
pub const LOTTERY_MAX_WINNERS_PER_BLOCK: u32 = 64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constitution_values() {
        assert_eq!(DECIMALS, 18);
        assert_eq!(UNITS_PER_TOY, 10u128.pow(DECIMALS));
        assert_eq!(MAX_SUPPLY.units(), 10u128.pow(26));
    }

    #[test]
    fn deposit_and_prize_values() {
        assert_eq!(ACTIVATION_DEPOSIT.to_string(), "0.1 TOY");
        assert_eq!(ACTIVATION_DEPOSIT_FLOOR.to_string(), "0.01 TOY");
        assert_eq!(LOTTERY_MIN_PRIZE.to_string(), "0.2 TOY");
        assert!(ACTIVATION_DEPOSIT_FLOOR < ACTIVATION_DEPOSIT);
    }
}
