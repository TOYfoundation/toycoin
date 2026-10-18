//! Emission of ToyChain (spec sections 5.2, 5.3, 9, 9.0 and 9.1).
//!
//! The genesis block holds no balances (total fair launch, decision 62).
//! All 100,000,000 TOY come from this emission:
//!
//! - per epoch `floor(remaining * k0 * m)` with `k0 = 505 / 10^6` and the
//!   dynamic factor `m` in [0.5, 2.0] ([`factor`]),
//! - `remaining = 10^26 - sum of all emissions` ([`EmissionState`]),
//! - spread over the first 1,440 blocks of the epoch and split 50/40/10
//!   into consensus, work lottery and refund pool ([`mint`]),
//! - in the bootstrap phase the consensus share goes in equal parts to the
//!   21 start validators, falling with plan days and capped ([`bootstrap`]),
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
pub mod mint;
pub mod plan;
mod schedule;
#[cfg(kani)]
mod verification;

pub use engine::{BlockInput, BlockOutcome, EmissionEngine};
pub use error::EmissionError;
pub use factor::EpochMeasurements;
pub use mint::{BlockMint, ConsensusPayout};
pub use schedule::{EmissionState, EpochEmission, EpochStart, Phase, emission_for};
