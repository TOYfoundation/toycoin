//! Cryptographic primitives of ToyChain behind stable interfaces
//! (crypto agility, spec section 10).
//!
//! Only thin wrappers around audited, widely used crates; no algorithm is
//! invented here. Callers never see the types of the underlying crates,
//! so an algorithm can be replaced inside this crate without changing
//! the rest of the node.
//!
//! | Purpose | Module | Crate |
//! |---|---|---|
//! | Keccak-256, BLAKE3 | [`hash`] | `sha3`, `blake3` |
//! | Account keys, signatures (secp256k1) | [`secp256k1`] | `k256` |
//! | Address text with EIP-55 checksum | [`address`] | – |
//! | Replay protection with chain ID (EIP-155) | [`eip155`] | `k256` |
//! | Recovery phrase and derivation (BIP-39, BIP-32) | [`hd`] | `bip39`, `bip32` |
//! | Validator signatures, aggregation (BLS12-381) | [`bls`] | `blst` |
//! | Verifiable random function | [`vrf`] | `blst`, `blake3` |
//!
//! Secrets (keys, phrases, seeds) are erased from memory when dropped
//! (`zeroize`), are never printed by `Debug` and never appear in errors.
//!
//! Argon2id (ToyWork tickets, keystore) follows with milestone M3/M8.

pub mod address;
pub mod bls;
pub mod eip155;
mod error;
pub mod hash;
pub mod hd;
mod random;
pub mod secp256k1;
pub mod vrf;

pub use error::CryptoError;
