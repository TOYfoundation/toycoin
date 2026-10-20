//! Hash functions (spec section 10).
//!
//! - Keccak-256 for Ethereum compatibility (addresses, signing hashes).
//! - BLAKE3 for Merkle trees and internal commitments.
//!
//! Every function returns a [`Hash`]. Which algorithm produced a hash is
//! decided by the caller's use; the functions are named after the
//! algorithm so the choice is visible at the call site.

use sha3::{Digest, Keccak256};
use toy_types::Hash;

/// Keccak-256 as used by Ethereum (the original Keccak padding, not the
/// final NIST SHA3-256).
#[must_use]
pub fn keccak256(data: &[u8]) -> Hash {
    Hash::from_bytes(Keccak256::digest(data).into())
}

/// BLAKE3 with the default 32-byte output.
#[must_use]
pub fn blake3(data: &[u8]) -> Hash {
    Hash::from_bytes(*blake3::hash(data).as_bytes())
}

/// BLAKE3 in key-derivation mode: the `context` string separates the
/// domains, so a hash computed for one purpose can never collide with a
/// hash for another purpose.
///
/// `context` must be a hard-coded, globally unique string such as
/// `"ToyChain 2026-10 VRF output v1"`, never user data.
#[must_use]
pub fn blake3_derive(context: &'static str, data: &[u8]) -> Hash {
    Hash::from_bytes(blake3::derive_key(context, data))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(hash: &Hash) -> String {
        hash.to_string()
    }

    #[test]
    fn keccak256_known_values() {
        // Keccak-256 of the empty string (Ethereum's KECCAK_EMPTY constant).
        assert_eq!(
            hex(&keccak256(b"")),
            "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
        );
    }

    #[test]
    fn blake3_known_values() {
        // From the official BLAKE3 test vectors (input length 0).
        assert_eq!(
            hex(&blake3(b"")),
            "0xaf1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
    }

    #[test]
    fn derive_separates_domains() {
        let a = blake3_derive("ToyChain test context A", b"data");
        let b = blake3_derive("ToyChain test context B", b"data");
        assert_ne!(a, b);
        assert_ne!(a, blake3(b"data"));
    }
}
