//! Account state of ToyChain (spec sections 3, 4, 6, 6.1, 8, 9).
//!
//! - [`State`]: all active accounts, the protocol pots and the rule data.
//!   TOY is created only by [`State::mint_block`]; every other operation
//!   moves TOY between pots. [`State::check_i0`] checks rule I0 (sum of all
//!   pots = sum of all emissions) after every block.
//! - [`Pot`]: the single list of all pots (spec 9). New transaction kinds
//!   must add their pot there.
//! - [`Account`]: free balance, activation deposit (0.1 TOY, spec 6.1),
//!   stake, unbonding (21 epochs), bonded start validator rewards (365
//!   epochs), held stake, validator key, per-epoch counters, penalties.
//! - [`penalty`]: protection rules of level A (spec 8). Penalties remove
//!   rights, never balances.
//! - [`State::root`]: BLAKE3 Merkle root over the whole state.
//!
//! Not yet here: dormant accounts and the persistent tree (M5), delegation
//! and slashing (M4), account protection (M12).

mod account;
mod chain;
mod error;
mod merkle;
pub mod penalty;
mod pots;
mod state;

pub use account::{
    Account, BLS_KEY_LEN, EpochActivity, HeldStake, RewardLock, Unbonding, ValidatorInfo,
};
pub use chain::ChainData;
pub use error::{CloseBlocker, StateError};
pub use merkle::{leaf_hash, merkle_root};
pub use penalty::{
    PenaltyOutcome, PenaltyRecord, Right, Violation, inactivity_violated, mass_creation_violated,
};
pub use pots::{Pot, PotTotals};
pub use state::{
    ConsensusMint, CreditOutcome, DetachedRecord, FeeSplit, GenesisConfig, Mint, State,
};

#[cfg(test)]
mod tests;
