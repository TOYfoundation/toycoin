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

// ---- Emission (spec 9, 9.0, 9.1, decisions 136, 137) ------------------

/// Number of rows of the K table (one every 30 plan days, 200 years).
pub const K_TABLE_ROWS: usize = 2_434;

/// Plan days between two rows of the K table.
pub const K_TABLE_STEP_DAYS: u64 = 30;

/// `k` in the K table is given in units of 10^-12 per epoch.
pub const K_SCALE: u128 = 1_000_000_000_000;

/// BLAKE3 hash of `toy-emission/data/k-tabelle.csv` (constitution, spec 13
/// no. 3, decision 136): the emission curve can never change.
pub const K_TABLE_BLAKE3: [u8; 32] = [
    0xcf, 0x0e, 0xa8, 0xcd, 0xf3, 0x29, 0x82, 0x08, 0x07, 0xbf, 0x03, 0xd8, 0x9d, 0xb4, 0x21, 0x16,
    0x84, 0x78, 0x91, 0x30, 0x94, 0x4b, 0xbd, 0xf4, 0xac, 0xf3, 0x0a, 0x81, 0x7c, 0x8b, 0xf0, 0x32,
];

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

/// Largest work share `w(t)` in the K table (45 %, falls to 25 %).
pub const WORK_SHARE_MAX: Ppm = Ppm::new(450_000);

/// Share of each block emission for the committee pot (spec 9.2).
pub const COMMITTEE_SHARE: Ppm = Ppm::new(50_000);

/// Share of each block emission for the full-node pot (tiers 4 to 7).
pub const FULL_NODE_SHARE: Ppm = Ppm::new(80_000);

/// Share of each block emission for the refund pool (plus rounding
/// remainders). The staking pot gets the rest of every block.
pub const REFUND_POOL_SHARE: Ppm = Ppm::new(20_000);

// ---- Bootstrap phase (spec 5.3, decisions 110, 139 to 141) ------------

/// Number of start validators named in the genesis block.
pub const START_VALIDATORS: u32 = 64;

/// Start wage on plan day 0 (25 % of the emission).
pub const START_REWARD_INITIAL_SHARE: Ppm = Ppm::new(250_000);

/// The start wage falls linearly to zero over this many plan days.
pub const START_REWARD_DAYS: u64 = 365;

/// Start validator rewards stay locked for this many epochs (time lock,
/// no stake and no voting weight).
pub const START_REWARD_LOCK_EPOCHS: u64 = 365;

/// Earliest handover out of the bootstrap phase, in plan days.
pub const HANDOVER_MIN_DAYS: u64 = 90;

/// Minimum number of new validators for the handover.
pub const HANDOVER_MIN_NEW_VALIDATORS: u32 = 21;

/// Stake of new validators counts for the handover only after this many epochs.
pub const HANDOVER_MIN_BONDED_EPOCHS: u64 = 30;

/// Handover target: 5 % of all minted TOY until plan day 180 ...
pub const HANDOVER_TARGET: Ppm = Ppm::new(50_000);

/// ... then falling linearly to 1 % by plan day 540 (decision 140).
pub const HANDOVER_TARGET_FLOOR: Ppm = Ppm::new(10_000);

/// Plan day on which the handover target starts to fall.
pub const HANDOVER_TARGET_FALL_FROM_DAYS: u64 = 180;

/// Plan day on which the handover target reaches its floor.
pub const HANDOVER_TARGET_FALL_UNTIL_DAYS: u64 = 540;

const _: () = assert!(
    WORK_SHARE_MAX.get()
        + START_REWARD_INITIAL_SHARE.get()
        + COMMITTEE_SHARE.get()
        + FULL_NODE_SHARE.get()
        + REFUND_POOL_SHARE.get()
        < Ppm::SCALE,
    "fixed shares must leave a staking share in every block"
);

// ---- Stake and committee (spec 4, 5, 14) -------------------------------

/// Maximum committee size.
pub const COMMITTEE_MAX: u32 = 128;

/// Fixed committee seats for the largest effective stake (decision 143).
pub const COMMITTEE_FIXED_SEATS: u32 = 64;

/// Committee seats drawn by stake among the other validators.
pub const COMMITTEE_LOTTERY_SEATS: u32 = 64;

/// Reserve seats drawn from tiers 6 and 7.
pub const COMMITTEE_RESERVE_SEATS: u32 = 32;

/// Duty draws per epoch: attester places (tiers 3 to 7).
pub const DUTY_ATTESTERS: u32 = 2_048;

/// Duty draws per epoch: reachability probes (tiers 2 to 7).
pub const DUTY_PROBES: u32 = 1_024;

const _: () = assert!(
    COMMITTEE_FIXED_SEATS + COMMITTEE_LOTTERY_SEATS == COMMITTEE_MAX,
    "fixed and lottery seats make up the committee"
);

/// Stake tier 1, saver (decision 142).
pub const STAKE_TIER_1: Amount = Amount::from_toy(1);

/// Stake tier 2, helper.
pub const STAKE_TIER_2: Amount = Amount::from_toy(5);

/// Stake tier 3, attester.
pub const STAKE_TIER_3: Amount = Amount::from_toy(10);

/// Stake tier 4, full node.
pub const STAKE_TIER_4: Amount = Amount::from_toy(25);

/// Stake tier 5, proof node.
pub const STAKE_TIER_5: Amount = Amount::from_toy(50);

/// Stake tier 6, reserve validator.
pub const STAKE_TIER_6: Amount = Amount::from_toy(75);

/// Stake tier 7, validator.
pub const STAKE_TIER_7: Amount = Amount::from_toy(100);

/// All stake tiers 1 to 7 in rising order.
pub const STAKE_TIERS: [Amount; 7] = [
    STAKE_TIER_1,
    STAKE_TIER_2,
    STAKE_TIER_3,
    STAKE_TIER_4,
    STAKE_TIER_5,
    STAKE_TIER_6,
    STAKE_TIER_7,
];

/// Minimum stake of a validator (tier 7).
pub const VALIDATOR_MIN_STAKE: Amount = STAKE_TIER_7;

/// Weight of tier 1 (saver) in the staking pot; tiers 2 to 7 count 1.0.
pub const SAVER_WEIGHT: Ppm = Ppm::new(800_000);

/// Stake cap per participant while fewer than [`STAKE_CAP_VALIDATOR_COUNT`]
/// validators are active: 5 % of all bonded stake (decision 111).
pub const STAKE_CAP_FEW_VALIDATORS: Ppm = Ppm::new(50_000);

/// Stake cap per participant from then on: 1 %.
pub const STAKE_CAP: Ppm = Ppm::new(10_000);

/// Number of validators from which the 1 % cap applies.
pub const STAKE_CAP_VALIDATOR_COUNT: usize = 100;

/// Scale of the staking reward counter (spec 9.4).
pub const REWARD_INDEX_SCALE: u128 = 1_000_000_000_000_000;

/// The staking pot distributes at most this much per epoch; the rest waits
/// for the next epoch. With [`REWARD_INDEX_SCALE`] this keeps every
/// product of the counter below 10^38, inside `u128` (spec 9.4).
pub const STAKING_MAX_PER_EPOCH: Amount = Amount::from_toy(100_000);

/// Unbonding period in epochs.
pub const UNBONDING_EPOCHS: u64 = 21;

// ---- Accounts and lottery (spec 6.1, 9) --------------------------------

/// Activation deposit on the first incoming transfer: 0.1 TOY.
pub const ACTIVATION_DEPOSIT: Amount = Amount::from_units(UNITS_PER_TOY / 10);

/// Lowest value the activation deposit may ever be lowered to: 0.01 TOY.
pub const ACTIVATION_DEPOSIT_FLOOR: Amount = Amount::from_units(UNITS_PER_TOY / 100);

/// Minimum prize of the work lottery for an address without account:
/// 0.2 TOY (0.1 TOY of it become the activation deposit).
pub const LOTTERY_MIN_PRIZE: Amount = Amount::from_units(UNITS_PER_TOY / 5);

/// Minimum prize for an address with an active account: 0.01 TOY.
pub const LOTTERY_MIN_PRIZE_ACTIVE: Amount = Amount::from_units(UNITS_PER_TOY / 100);

/// Share of the work reward for the base pot, paid every block
/// (decision 149). The three jackpots get the rest.
pub const LOTTERY_BASE_SHARE: Ppm = Ppm::new(850_000);

/// Share of the work reward for the jackpot "Funke".
pub const JACKPOT_FUNKE_SHARE: Ppm = Ppm::new(80_000);

/// Share of the work reward for the jackpot "Welle".
pub const JACKPOT_WELLE_SHARE: Ppm = Ppm::new(50_000);

/// Share of the work reward for the jackpot "Gezeiten".
pub const JACKPOT_GEZEITEN_SHARE: Ppm = Ppm::new(20_000);

const _: () = assert!(
    LOTTERY_BASE_SHARE.get()
        + JACKPOT_FUNKE_SHARE.get()
        + JACKPOT_WELLE_SHARE.get()
        + JACKPOT_GEZEITEN_SHARE.get()
        == Ppm::SCALE,
    "lottery pot shares must add up to 100 %"
);

/// Base pot: at least this share of the tickets of a block wins.
pub const LOTTERY_MIN_WINNER_SHARE: Ppm = Ppm::new(500_000);

/// Target interval of the jackpots in blocks: Funke, Welle, Gezeiten.
pub const JACKPOT_INTERVALS: [u64; 3] = [240, 7_200, 86_400];

/// Fewest and most winners of a jackpot: Funke, Welle, Gezeiten.
pub const JACKPOT_WINNERS: [(u64, u64); 3] = [(1, 4), (8, 64), (64, 512)];

/// Funke draws from the tickets of the last this many blocks; Welle and
/// Gezeiten from the tickets of the previous epoch.
pub const JACKPOT_FUNKE_WINDOW_BLOCKS: u64 = 60;

/// A jackpot pays a random share of its pot of at least 40 %.
pub const JACKPOT_MIN_PAYOUT: Ppm = Ppm::new(400_000);

/// One jackpot pays at most 0.5 % of the emission of the last 365 epochs.
pub const JACKPOT_MAX_SHARE_OF_YEAR: Ppm = Ppm::new(5_000);

/// Number of past epochs for [`JACKPOT_MAX_SHARE_OF_YEAR`].
pub const JACKPOT_YEAR_EPOCHS: usize = 365;

/// A pot of this many times its target level is paid out in any case.
pub const JACKPOT_FORCE_FACTOR: u128 = 4;

/// Bounds of the pot-level factor of the trigger chance.
pub const JACKPOT_LEVEL_FACTOR_BOUNDS: (u64, u64) = (250_000, 3_000_000);

/// Bounds of the product of the special factors.
pub const JACKPOT_SPECIAL_FACTOR_BOUNDS: (u64, u64) = (500_000, 3_000_000);

/// Participation of the previous epoch from which Funke and Welle get
/// more likely (×1.3), and below which they get less likely (×0.8).
pub const JACKPOT_PARTICIPATION_HIGH: Ppm = Ppm::new(950_000);

/// See [`JACKPOT_PARTICIPATION_HIGH`].
pub const JACKPOT_PARTICIPATION_LOW: Ppm = Ppm::new(800_000);

/// Milestone: every this many blocks a Gezeiten jackpot follows.
pub const JACKPOT_MILESTONE_HEIGHT: u64 = 1_000_000;

/// Milestone: every full this many TOY minted a Welle jackpot follows.
pub const JACKPOT_MILESTONE_SUPPLY: Amount = Amount::from_toy(10_000_000);

/// Milestone: the lowest tide of the curve, every this many plan days
/// (from the first full wave on), brings a Gezeiten jackpot.
pub const JACKPOT_MILESTONE_TIDE_DAYS: u64 = 1_825;

/// Tickets committed in block N are drawn with the randomness of block
/// N + 2, so the builder of block N cannot steer the draw (decision 106).
pub const LOTTERY_DRAW_DELAY_BLOCKS: u64 = 2;

/// A new account starts its nonce at `height << NONCE_BASE_SHIFT`
/// (spec 6.1: `height × 2^16`), so transactions signed for an earlier,
/// closed account at the same address can never become valid again.
pub const NONCE_BASE_SHIFT: u32 = 16;

/// The penalty history of a closed account is kept this many epochs
/// (spec 6.1), so closing and re-opening gives no clean record.
pub const CLOSED_ACCOUNT_HISTORY_EPOCHS: u64 = 64;

// ---- Transactions and blocks (spec 7.2, decisions 84, 106) -------------

/// Maximum block size in bytes (transactions and tickets).
pub const MAX_BLOCK_BYTES: u64 = 131_072;

/// Free transactions may use at most this many bytes of a block (50 %).
pub const MAX_FREE_BYTES_PER_BLOCK: u64 = MAX_BLOCK_BYTES / 2;

/// Maximum length of the optional payment reference (memo) in bytes.
pub const MEMO_MAX_BYTES: usize = 32;

/// A transaction may expire at most this many blocks after the height of
/// the block that includes it.
pub const MAX_EXPIRY_BLOCKS: u64 = 1_440;

// ---- Free quota and refund pool (spec 7.2, decisions 106, 118, 120) ----

/// Smallest free quota per account and epoch (at full utilization,
/// before the water level, decision 145).
pub const QUOTA_MIN: u64 = 21;

/// Largest free quota per account and epoch (at zero utilization).
pub const QUOTA_MAX: u64 = 49;

/// Water level: the free transactions of the previous epoch must fit into
/// this share of the free space.
pub const QUOTA_WATER_LEVEL: Ppm = Ppm::new(900_000);

/// Nominal size of a free transaction for the water level.
pub const QUOTA_NOMINAL_TX_BYTES: u64 = 150;

/// New accounts reach the full quota over this many blocks after the
/// minimum age (7 epochs).
pub const QUOTA_RAMP_BLOCKS: u64 = 7 * 1_440;

/// An account must be at least this many blocks old to use the free quota.
pub const QUOTA_MIN_ACCOUNT_AGE_BLOCKS: u64 = 100;

/// Per epoch at most `pool / REFUND_BUDGET_DIVISOR` is refunded.
pub const REFUND_BUDGET_DIVISOR: u128 = 30;

/// The refund pool keeps at most this many epochs of the largest possible
/// refund demand; the surplus moves to the work lottery at the start of an
/// epoch (decision 120, Prüfbericht 1 P9). Nothing is burned.
pub const REFUND_POOL_CAP_EPOCHS: u128 = 30;

/// Heavy users: from this many transactions per epoch on, a sender pays
/// at least [`HEAVY_USER_FEE_FACTOR`] times the base fee and cannot use
/// tickets or the free quota (decision 146).
pub const HEAVY_USER_THRESHOLD: u64 = 1_000;

/// Fee factor for heavy users.
pub const HEAVY_USER_FEE_FACTOR: u128 = 2;

// ---- Fees (spec 7.2, decision 117) --------------------------------------

/// Lowest base fee per transaction byte: 10^12 units = 0.000001 TOY
/// (about 0.00015 TOY for a 150-byte transfer).
pub const BASE_FEE_MIN_PER_BYTE: Amount = Amount::from_units(1_000_000_000_000);

/// Highest base fee per byte (1 TOY). Only a bound against overflow; real
/// load cannot plausibly reach it.
pub const BASE_FEE_MAX_PER_BYTE: Amount = Amount::from_units(UNITS_PER_TOY);

/// The base fee moves towards this many used bytes per block (EIP-1559
/// target: half of the block).
pub const BASE_FEE_TARGET_BYTES: u64 = MAX_BLOCK_BYTES / 2;

/// The base fee changes by at most 1/8 per block (EIP-1559).
pub const BASE_FEE_CHANGE_DENOMINATOR: u128 = 8;

/// The fee of a sender doubles with every this many further transactions
/// of the same sender in the same block.
pub const PROGRESSIVE_FEE_STEP: u64 = 10;

/// Highest number of doublings: factor at most 2^10 = 1,024, reached from
/// the 101st transaction of a sender in a block on (decision 117).
pub const PROGRESSIVE_FEE_MAX_DOUBLINGS: u32 = 10;

/// Share of a paid fee for the staking pot (decision 147).
pub const FEE_SHARE_STAKING: Ppm = Ppm::new(400_000);

/// Share of a paid fee for the block proposer. The refund pool gets the
/// rest (40 % plus rounding remainders).
pub const FEE_SHARE_PROPOSER: Ppm = Ppm::new(200_000);

const _: () = assert!(
    FEE_SHARE_STAKING.get() + FEE_SHARE_PROPOSER.get() <= Ppm::SCALE,
    "fee shares must not exceed 100 %"
);

// ---- ToyWork (spec 9, 10, decision 119) ---------------------------------

/// Argon2id memory per ToyWork hash in KiB (16 MiB).
pub const TOYWORK_MEMORY_KIB: u32 = 16 * 1024;

/// Argon2id passes per ToyWork hash.
pub const TOYWORK_ITERATIONS: u32 = 1;

/// Argon2id lanes per ToyWork hash.
pub const TOYWORK_PARALLELISM: u32 = 1;

/// Maximum number of ToyWork tickets per block at a block gap of 60 s or
/// more (standalone and attached together). Shorter gaps allow
/// proportionally fewer, at least [`TICKETS_MIN_PER_BLOCK`]. Verifying one
/// costs about 10 ms on a PC (decision 148).
pub const TICKETS_MAX_PER_BLOCK: u64 = 32;

/// Lower bound of the ticket limit for short block gaps.
pub const TICKETS_MIN_PER_BLOCK: u64 = 8;

/// Maximum number of tickets a node accepts from one peer per block
/// (network layer, M6).
pub const TICKETS_MAX_PER_PEER_PER_BLOCK: u64 = 8;

/// The difficulty adapts once per epoch towards this many tickets per block.
pub const TICKETS_TARGET_PER_BLOCK: u64 = 16;

/// Start threshold: about one ticket per 1,024 hashes (2^118 of 2^128).
pub const TOYWORK_START_THRESHOLD: u128 = 1 << 118;

/// The threshold changes by at most this factor per epoch, up or down.
pub const TOYWORK_MAX_ADJUST_FACTOR: u128 = 4;

// ---- Protection rules, level A (spec 8, decision 67) --------------------

/// Penalty for a double signature: role locked 7 epochs.
pub const PENALTY_DOUBLE_SIGN_EPOCHS: u64 = 7;

/// Stake of a double signer is held this many epochs until the slashing
/// amount is fixed (spec 5.4; the slashing itself follows in M4).
pub const DOUBLE_SIGN_HOLD_EPOCHS: u64 = 18;

/// Penalty for proposing an invalid block: proposal right locked 7 epochs.
pub const PENALTY_INVALID_PROPOSAL_EPOCHS: u64 = 7;

/// Penalty for an invalid ToyWork ticket: tickets refused for 1 epoch.
pub const PENALTY_INVALID_TICKET_EPOCHS: u64 = 1;

/// Penalty for mass account creation: free quota locked 7 epochs.
pub const PENALTY_MASS_CREATION_EPOCHS: u64 = 7;

/// Mass account creation: more than this many new accounts per epoch from
/// one sender.
pub const MASS_CREATION_LIMIT: u64 = 20;

/// Committee inactivity: missing signatures in this many consecutive
/// epochs pause the role until the node reports back.
pub const INACTIVITY_EPOCHS: u64 = 3;

/// Every further violation doubles the penalty, up to this many epochs.
pub const PENALTY_MAX_EPOCHS: u64 = 64;

/// After this many epochs without a violation, counting starts afresh.
pub const PENALTY_RESET_EPOCHS: u64 = 30;

// ---- Chain IDs (EIP-155, decision 116) ----------------------------------

/// Chain ID of the ToyChain mainnet: `0x746F79`, the ASCII bytes of "toy".
/// Fixed by decision 116; checked as free in ethereum-lists/chains on
/// 2026-10-20, registration follows with the public testnet (M10).
pub const CHAIN_ID_MAINNET: u64 = 0x0074_6f79; // 7_630_713

/// Chain ID of the public ToyChain testnet (decision 40: differs from
/// mainnet, so testnet signatures are never valid on mainnet).
pub const CHAIN_ID_TESTNET: u64 = 7_630_714;

/// Chain ID of local development networks (`toy-cli devnet`, tests).
/// Separate from the public testnet, so local test signatures can never
/// be replayed there.
pub const CHAIN_ID_DEVNET: u64 = 7_630_715;

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
        assert_eq!(LOTTERY_MIN_PRIZE_ACTIVE.to_string(), "0.01 TOY");
        assert!(ACTIVATION_DEPOSIT_FLOOR < ACTIVATION_DEPOSIT);
    }

    #[test]
    fn stake_tiers_rise() {
        assert_eq!(STAKE_TIER_1.to_string(), "1 TOY");
        assert_eq!(VALIDATOR_MIN_STAKE.to_string(), "100 TOY");
        for pair in STAKE_TIERS.windows(2) {
            assert!(pair.first() < pair.get(1));
        }
    }

    #[test]
    fn chain_ids() {
        // Mainnet spells "toy" in hex (decision 116).
        assert_eq!(CHAIN_ID_MAINNET.to_be_bytes()[5..], *b"toy");
        assert_eq!(CHAIN_ID_MAINNET, 7_630_713);
        assert_eq!(CHAIN_ID_TESTNET, 7_630_714);
        assert_eq!(CHAIN_ID_DEVNET, 7_630_715);
        // Well-known IDs that must never be reused: Ethereum, Polygon,
        // and the common local developer IDs.
        let ours = [CHAIN_ID_MAINNET, CHAIN_ID_TESTNET, CHAIN_ID_DEVNET];
        for taken in [1, 137, 1337, 31337] {
            assert!(!ours.contains(&taken), "{taken}");
        }
        // Three distinct networks (decision 40), all below 2^32 (safe for
        // MetaMask and hardware wallets). A runtime test instead of a
        // compile-time `const _: () = assert!(..)`: the Kani verifier in
        // CI failed to build toy-types with such a const item.
        for (i, a) in ours.iter().enumerate() {
            assert!(*a < 1 << 32, "{a}");
            for b in ours.iter().skip(i + 1) {
                assert_ne!(a, b);
            }
        }
    }
}
