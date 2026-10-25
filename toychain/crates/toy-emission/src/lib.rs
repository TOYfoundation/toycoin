//! Emission of ToyChain (spec sections 5.2, 5.3, 9, 9.0 and 9.1).
//!
//! The genesis block holds no balances (total fair launch, decision 62).
//! All 100,000,000 TOY come from this emission:
//!
//! - per epoch `remaining × k(t) × m` with `k(t)` from the fixed K table
//!   of the tide curve ([`ktable`], decision 136) and the dynamic factor
//!   `m` in [0.5, 2.0] ([`factor`]),
//! - `remaining = 10^26 - sum of all emissions` ([`EmissionState`]),
//! - spread over the first 1,440 blocks of the epoch and split into work
//!   reward, start wage, committee, full-node, refund and staking pots
//!   ([`mint`], spec 9.2),
//! - in the bootstrap phase the start wage and the committee pot go in
//!   equal parts to the 64 start validators, the wage falling with plan
//!   days and capped ([`bootstrap`]),
//! - time is measured as plan time with capped block gaps, and an epoch
//!   lasts at least 1,440 blocks and 24 hours ([`clock`]).
//!
//! [`EmissionEngine`] ties these parts together block by block.
//!
//! Safety properties, each covered by tests (and Kani proofs in CI):
//! the sum of all emissions never exceeds 10^26, every block mints exactly
//! its share of the epoch emission, nothing is burned, and slashing or
//! donations can never raise the remaining amount (this crate offers no
//! operation that lowers the issued sum).

pub mod bootstrap;
pub mod clock;
mod engine;
mod error;
pub mod factor;
mod k_table_data;
pub mod ktable;
pub mod mint;
pub mod plan;
mod schedule;
#[cfg(kani)]
mod verification;

pub use engine::{BlockInput, BlockOutcome, EmissionEngine};
pub use error::EmissionError;
pub use factor::EpochMeasurements;
pub use mint::{BlockMint, WorkMint};
pub use schedule::{EmissionState, EpochEmission, EpochStart, Phase, emission_for};
