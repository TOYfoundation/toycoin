//! Core types and constants of ToyChain.
//!
//! - [`Amount`]: an amount of TOY in smallest units (`u128`, 18 decimals).
//!   All arithmetic is checked; overflow and underflow are errors.
//! - [`Ppm`]: a fixed-point factor in parts per million (1,000,000 = 1.0).
//! - [`params`]: protocol constants from the specification (section 14).
//!   The supply cap of 10^26 smallest units (100,000,000 TOY) is part of
//!   the constitution (spec section 13) and never changes.
//! - [`Address`] and [`Hash`]: fixed-size byte types. Derivation and
//!   checksums are in `toy-crypto`.
//!
//! Floating point is never used (workspace lint `float_arithmetic`).

mod amount;
mod bytes;
mod error;
pub mod params;
mod ppm;

pub use amount::Amount;
pub use bytes::{Address, Hash};
pub use error::ArithmeticError;
pub use ppm::Ppm;

/// Seconds since the Unix epoch, as carried in block timestamps.
pub type UnixSeconds = u64;

/// Block height. The genesis block has height 0.
pub type Height = u64;

/// Epoch number. The genesis block belongs to epoch 0.
pub type EpochNumber = u64;
