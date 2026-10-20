//! Verifiable random function (VRF) for committee selection and lotteries
//! (spec sections 5.1 and 10).
//!
//! A VRF turns a secret key and an input into a random-looking 32-byte
//! output plus a proof. Anyone with the public key can check the proof and
//! recompute the output, but nobody can predict the output without the
//! secret key, and the key owner cannot choose between several outputs.
//!
//! # Construction (proposal, see `docs/STATUS.md`, M2)
//!
//! [`BlsVrf`] uses the uniqueness of BLS signatures: for a given key and
//! message there is exactly one valid signature. The proof is the BLS
//! signature over the input (own domain tag [`VRF_DST`]); the output is a
//! hash of that signature. Ethereum (RANDAO) and the Internet Computer use
//! the same principle for their randomness. No new cryptography is
//! invented: it reuses the audited `blst` signature and BLAKE3.
//!
//! The spec names RFC 9381 (ECVRF). No audited Rust implementation of
//! RFC 9381 exists today (October 2026). The [`Vrf`] trait keeps the
//! choice swappable: an RFC 9381 implementation can be added later without
//! touching callers.

use toy_types::Hash;

use crate::CryptoError;
use crate::bls::{BlsPublicKey, BlsSecretKey, BlsSignature, SIGNATURE_LEN};
use crate::hash::blake3_derive;

/// Domain separation tag for VRF proofs. Differs from the signature and
/// proof-of-possession tags, so a vote can never be reused as a VRF proof
/// and the other way round.
pub const VRF_DST: &[u8] = b"TOYCHAIN_VRF_V1_BLS12381G2_XMD:SHA-256_SSWU_RO_";

/// BLAKE3 key-derivation context for VRF outputs.
const OUTPUT_CONTEXT: &str = "ToyChain 2026-10 VRF output v1 (BLS12-381 G2 proof)";

/// Interface of a verifiable random function (crypto agility).
pub trait Vrf {
    /// Secret key type.
    type SecretKey;
    /// Public key type.
    type PublicKey;
    /// Proof type.
    type Proof;

    /// Computes output and proof for `input`.
    fn prove(secret_key: &Self::SecretKey, input: &[u8]) -> (Hash, Self::Proof);

    /// Checks `proof` and returns the output it proves.
    ///
    /// # Errors
    /// [`CryptoError::VerificationFailed`] if the proof is invalid.
    fn verify(
        public_key: &Self::PublicKey,
        input: &[u8],
        proof: &Self::Proof,
    ) -> Result<Hash, CryptoError>;
}

/// Proof of the BLS-based VRF: a subgroup-checked BLS signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlsVrfProof(BlsSignature);

impl BlsVrfProof {
    /// Reads a proof (compressed 96-byte G2 point).
    ///
    /// # Errors
    /// See [`BlsSignature::from_bytes`].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        BlsSignature::from_bytes(bytes).map(Self)
    }

    /// The compressed encoding (96 bytes).
    #[must_use]
    pub fn to_bytes(&self) -> [u8; SIGNATURE_LEN] {
        self.0.to_bytes()
    }

    fn output(&self) -> Hash {
        // Hash the canonical re-encoding, never the received bytes.
        blake3_derive(OUTPUT_CONTEXT, &self.0.to_bytes())
    }
}

/// VRF built from unique BLS signatures (see module documentation).
#[derive(Clone, Copy, Debug)]
pub struct BlsVrf;

impl Vrf for BlsVrf {
    type SecretKey = BlsSecretKey;
    type PublicKey = BlsPublicKey;
    type Proof = BlsVrfProof;

    fn prove(secret_key: &BlsSecretKey, input: &[u8]) -> (Hash, BlsVrfProof) {
        let proof = BlsVrfProof(secret_key.sign_with_dst(input, VRF_DST));
        (proof.output(), proof)
    }

    fn verify(
        public_key: &BlsPublicKey,
        input: &[u8],
        proof: &BlsVrfProof,
    ) -> Result<Hash, CryptoError> {
        public_key.verify_with_dst(input, &proof.0, VRF_DST)?;
        Ok(proof.output())
    }
}
