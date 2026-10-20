//! Replay protection with a chain ID (EIP-155, spec section 10).
//!
//! A signature made for ToyChain must never be valid on another chain,
//! and the other way round. EIP-155 achieves this twice over:
//!
//! 1. The chain ID is part of the signed data, so the signing hash differs
//!    on every chain (built by the transaction code, milestone M3).
//! 2. The chain ID is encoded in `v = 2 · chain_id + 35 + y_parity`, so a
//!    verifier can reject a foreign signature before doing any curve work.
//!
//! [`recover_eip155`] enforces point 2 and rejects signatures without a
//! chain ID (`v` = 27 or 28). Point 1 is the caller's job: it must compute
//! `signing_hash` with its own chain ID.

use toy_types::{Address, Hash, params};

use crate::CryptoError;
use crate::secp256k1::{RecoverableSignature, SecretKey};

/// A chain ID as used in EIP-155 signatures.
///
/// Always in `1..=ChainId::MAX`. ToyChain's own IDs (decision 116) are
/// available as [`ChainId::TOYCHAIN_MAINNET`], [`ChainId::TOYCHAIN_TESTNET`]
/// and [`ChainId::TOYCHAIN_DEVNET`]; their values live in
/// `toy_types::params`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChainId(u64);

impl ChainId {
    /// ToyChain mainnet, 7630713 (`0x746F79`, "toy").
    pub const TOYCHAIN_MAINNET: Self = Self::from_param(params::CHAIN_ID_MAINNET);

    /// Public ToyChain testnet, 7630714.
    pub const TOYCHAIN_TESTNET: Self = Self::from_param(params::CHAIN_ID_TESTNET);

    /// Local development networks, 7630715.
    pub const TOYCHAIN_DEVNET: Self = Self::from_param(params::CHAIN_ID_DEVNET);

    /// Builds a constant from a protocol parameter. An invalid parameter
    /// stops compilation (const evaluation), so it can never reach a node.
    #[allow(
        clippy::panic,
        reason = "only evaluated at compile time for the constants above"
    )]
    const fn from_param(id: u64) -> Self {
        match Self::new(id) {
            Ok(chain_id) => chain_id,
            Err(_) => panic!("chain ID parameter out of range"),
        }
    }

    /// Largest chain ID whose `v` still fits into 64 bits
    /// (`floor(u64::MAX / 2) - 36`, EIP-2294).
    pub const MAX: u64 = u64::MAX / 2 - 36;

    /// Creates a chain ID.
    ///
    /// # Errors
    /// [`CryptoError::InvalidChainId`] for 0 or values above [`Self::MAX`].
    pub const fn new(id: u64) -> Result<Self, CryptoError> {
        if id == 0 || id > Self::MAX {
            Err(CryptoError::InvalidChainId(id))
        } else {
            Ok(Self(id))
        }
    }

    /// The numeric value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// The EIP-155 `v` value for this chain and parity bit.
    #[must_use]
    pub const fn v(self, y_parity: bool) -> u64 {
        // Cannot overflow: self.0 <= MAX = u64::MAX / 2 - 36.
        let base = self.0.saturating_mul(2).saturating_add(35);
        if y_parity {
            base.saturating_add(1)
        } else {
            base
        }
    }

    /// Splits an EIP-155 `v` value into chain ID and parity bit.
    ///
    /// # Errors
    /// - [`CryptoError::MissingChainId`] for `v` = 27 or 28.
    /// - [`CryptoError::InvalidV`] for any other `v` below 37.
    pub fn from_v(v: u64) -> Result<(Self, bool), CryptoError> {
        if v == 27 || v == 28 {
            return Err(CryptoError::MissingChainId);
        }
        let rest = v.checked_sub(35).ok_or(CryptoError::InvalidV(v))?;
        let y_parity = rest & 1 == 1;
        let id = rest >> 1;
        let chain_id = Self::new(id).map_err(|_| CryptoError::InvalidV(v))?;
        Ok((chain_id, y_parity))
    }
}

/// A legacy Ethereum transaction signature with the chain ID in `v`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Eip155Signature {
    /// `v = 2 · chain_id + 35 + y_parity`.
    pub v: u64,
    /// `r` (32 bytes, big-endian).
    pub r: [u8; 32],
    /// `s` (32 bytes, big-endian).
    pub s: [u8; 32],
}

/// Signs `signing_hash` for `chain_id`.
///
/// `signing_hash` must be the Keccak-256 hash of the transaction data
/// that already contains `chain_id` (EIP-155 signing data).
#[must_use]
pub fn sign_eip155(key: &SecretKey, chain_id: ChainId, signing_hash: &Hash) -> Eip155Signature {
    let signature = key.sign_prehash(signing_hash);
    Eip155Signature {
        v: chain_id.v(signature.y_parity()),
        r: *signature.r(),
        s: *signature.s(),
    }
}

/// Recovers the signer of an EIP-155 signature, accepting only signatures
/// for `expected_chain`.
///
/// `signing_hash` must be computed by the verifier with its own chain ID,
/// never taken from the sender.
///
/// # Errors
/// - [`CryptoError::MissingChainId`] for pre-EIP-155 signatures.
/// - [`CryptoError::WrongChainId`] if `v` encodes another chain.
/// - [`CryptoError::InvalidV`], [`CryptoError::InvalidSignature`],
///   [`CryptoError::NonCanonicalSignature`] for malformed values.
/// - [`CryptoError::VerificationFailed`] if no signer can be recovered.
pub fn recover_eip155(
    signing_hash: &Hash,
    signature: &Eip155Signature,
    expected_chain: ChainId,
) -> Result<Address, CryptoError> {
    let (chain_id, y_parity) = ChainId::from_v(signature.v)?;
    if chain_id != expected_chain {
        return Err(CryptoError::WrongChainId {
            expected: expected_chain,
            found: chain_id,
        });
    }
    RecoverableSignature::from_parts(signature.r, signature.s, y_parity)?
        .recover_address(signing_hash)
}

/// Checks that `signature` is a valid EIP-155 signature by
/// `expected_signer` on `expected_chain`.
///
/// # Errors
/// Everything [`recover_eip155`] reports, plus
/// [`CryptoError::VerificationFailed`] if the signer differs.
pub fn verify_eip155(
    signing_hash: &Hash,
    signature: &Eip155Signature,
    expected_chain: ChainId,
    expected_signer: &Address,
) -> Result<(), CryptoError> {
    let signer = recover_eip155(signing_hash, signature, expected_chain)?;
    if signer == *expected_signer {
        Ok(())
    } else {
        Err(CryptoError::VerificationFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v_roundtrip_and_bounds() {
        for id in [1, 2, 1337, 0x0001_2345, ChainId::MAX] {
            let chain = ChainId::new(id).unwrap();
            for parity in [false, true] {
                assert_eq!(ChainId::from_v(chain.v(parity)).unwrap(), (chain, parity));
            }
        }
        assert_eq!(ChainId::new(1).unwrap().v(false), 37);
        assert_eq!(ChainId::new(1).unwrap().v(true), 38);
        assert!(ChainId::new(0).is_err());
        assert!(ChainId::new(ChainId::MAX + 1).is_err());
        // 2 * (2^63 - 1 - 36) + 36 = u64::MAX - 37: no overflow at the top.
        assert_eq!(ChainId::new(ChainId::MAX).unwrap().v(true), u64::MAX - 37);
    }

    #[test]
    fn toychain_ids_match_params() {
        assert_eq!(ChainId::TOYCHAIN_MAINNET.get(), 7_630_713);
        assert_eq!(ChainId::TOYCHAIN_TESTNET.get(), 7_630_714);
        assert_eq!(ChainId::TOYCHAIN_DEVNET.get(), 7_630_715);
        // v values as they appear in legacy (EIP-155) transactions.
        assert_eq!(ChainId::TOYCHAIN_MAINNET.v(false), 15_261_461);
        assert_eq!(ChainId::TOYCHAIN_TESTNET.v(true), 15_261_464);
    }

    #[test]
    fn v_without_or_with_invalid_chain_id_is_rejected() {
        assert_eq!(ChainId::from_v(27), Err(CryptoError::MissingChainId));
        assert_eq!(ChainId::from_v(28), Err(CryptoError::MissingChainId));
        for v in [0, 1, 26, 29, 34, 35, 36, u64::MAX] {
            assert!(
                matches!(ChainId::from_v(v), Err(CryptoError::InvalidV(_))),
                "v = {v}"
            );
        }
    }
}
