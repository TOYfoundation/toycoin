//! BLS12-381 signatures for validators (spec sections 5.1 and 10).
//!
//! Wraps `blst` (Supranational, externally audited, used by Ethereum
//! consensus clients). The ciphersuite is the one Ethereum uses:
//! public keys in G1 (48 bytes), signatures in G2 (96 bytes),
//! `BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_POP_` (IETF draft
//! "BLS Signatures", proof-of-possession scheme).
//!
//! Rogue-key protection: a committee vote is checked with
//! [`fast_aggregate_verify`], which only accepts keys whose proof of
//! possession was verified ([`PossessionVerifiedKey`]). The type system
//! makes it impossible to forget this step.
//!
//! Every public key is validated on construction (not infinity, in the
//! prime-order subgroup). Every signature is subgroup-checked on decoding.

use core::fmt;
use std::collections::HashSet;

use blst::BLST_ERROR;
use blst::min_pk::{AggregateSignature, PublicKey, SecretKey, Signature};
use zeroize::Zeroizing;

use crate::{CryptoError, random};

/// Domain separation tag for ordinary signatures (votes, checkpoints).
pub const SIGNATURE_DST: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_POP_";

/// Domain separation tag for proofs of possession (IETF draft, PopProve).
pub const POSSESSION_DST: &[u8] = b"BLS_POP_BLS12381G2_XMD:SHA-256_SSWU_RO_POP_";

/// Length of a compressed public key.
pub const PUBLIC_KEY_LEN: usize = 48;

/// Length of a compressed signature.
pub const SIGNATURE_LEN: usize = 96;

fn map_error(error: BLST_ERROR) -> CryptoError {
    match error {
        BLST_ERROR::BLST_PK_IS_INFINITY => CryptoError::InfinityPublicKey,
        BLST_ERROR::BLST_POINT_NOT_IN_GROUP => CryptoError::PointNotInGroup,
        BLST_ERROR::BLST_VERIFY_FAIL => CryptoError::VerificationFailed,
        BLST_ERROR::BLST_BAD_SCALAR => CryptoError::InvalidSecretKey,
        _ => CryptoError::InvalidSignature,
    }
}

fn check(result: BLST_ERROR) -> Result<(), CryptoError> {
    if result == BLST_ERROR::BLST_SUCCESS {
        Ok(())
    } else {
        Err(map_error(result))
    }
}

/// A BLS secret key. Generated per device, never derived from the
/// recovery phrase (spec 5.5, point 3). Erased when dropped; `Debug`
/// never prints it; deliberately no `Clone`.
pub struct BlsSecretKey(SecretKey);

impl BlsSecretKey {
    /// Generates a key from 32 bytes of secure randomness (IETF KeyGen).
    ///
    /// # Errors
    /// [`CryptoError::Randomness`] if the generator fails.
    pub fn generate() -> Result<Self, CryptoError> {
        let mut ikm = Zeroizing::new([0u8; 32]);
        random::fill(ikm.as_mut())?;
        SecretKey::key_gen(ikm.as_ref(), &[])
            .map(Self)
            .map_err(|_| CryptoError::InvalidSecretKey)
    }

    /// Reads a 32-byte big-endian secret scalar.
    ///
    /// # Errors
    /// [`CryptoError::InvalidSecretKey`] if it is zero or not below the
    /// group order.
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, CryptoError> {
        SecretKey::from_bytes(bytes)
            .map(Self)
            .map_err(|_| CryptoError::InvalidSecretKey)
    }

    /// The secret scalar (for the encrypted keystore). Erased when dropped.
    #[must_use]
    pub fn to_bytes(&self) -> Zeroizing<[u8; 32]> {
        Zeroizing::new(self.0.to_bytes())
    }

    /// The matching public key.
    #[must_use]
    pub fn public_key(&self) -> BlsPublicKey {
        BlsPublicKey(self.0.sk_to_pk())
    }

    /// Signs `message` with [`SIGNATURE_DST`].
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> BlsSignature {
        BlsSignature(self.0.sign(message, SIGNATURE_DST, &[]))
    }

    /// Signs with a custom domain separation tag (used by the VRF).
    pub(crate) fn sign_with_dst(&self, message: &[u8], dst: &[u8]) -> BlsSignature {
        BlsSignature(self.0.sign(message, dst, &[]))
    }

    /// Creates the proof of possession for this key: a signature over the
    /// compressed public key with [`POSSESSION_DST`]. Published once when
    /// the key is registered.
    #[must_use]
    pub fn prove_possession(&self) -> ProofOfPossession {
        let public_key = self.public_key().to_bytes();
        ProofOfPossession(self.sign_with_dst(&public_key, POSSESSION_DST))
    }
}

impl fmt::Debug for BlsSecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BlsSecretKey(<redacted>)")
    }
}

/// A validated BLS public key: never the point at infinity, always in the
/// prime-order subgroup.
#[derive(Clone)]
pub struct BlsPublicKey(PublicKey);

impl BlsPublicKey {
    /// Reads a compressed public key (48 bytes) and validates it.
    ///
    /// # Errors
    /// [`CryptoError::InvalidPublicKey`] for a bad encoding,
    /// [`CryptoError::InfinityPublicKey`] or [`CryptoError::PointNotInGroup`].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        let key = PublicKey::uncompress(bytes).map_err(|error| match error {
            BLST_ERROR::BLST_POINT_NOT_IN_GROUP => CryptoError::PointNotInGroup,
            _ => CryptoError::InvalidPublicKey,
        })?;
        key.validate().map_err(map_error)?;
        Ok(Self(key))
    }

    /// The compressed encoding (48 bytes).
    #[must_use]
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_LEN] {
        self.0.compress()
    }

    /// Verifies a single signature with [`SIGNATURE_DST`].
    ///
    /// # Errors
    /// [`CryptoError::VerificationFailed`] if the signature is invalid.
    pub fn verify(&self, message: &[u8], signature: &BlsSignature) -> Result<(), CryptoError> {
        self.verify_with_dst(message, signature, SIGNATURE_DST)
    }

    pub(crate) fn verify_with_dst(
        &self,
        message: &[u8],
        signature: &BlsSignature,
        dst: &[u8],
    ) -> Result<(), CryptoError> {
        check(signature.0.verify(true, message, dst, &[], &self.0, true))
    }

    /// Checks a proof of possession. Only keys that pass may take part in
    /// aggregate verification.
    ///
    /// # Errors
    /// [`CryptoError::VerificationFailed`] if the proof is invalid.
    pub fn verify_possession(
        &self,
        proof: &ProofOfPossession,
    ) -> Result<PossessionVerifiedKey, CryptoError> {
        self.verify_with_dst(&self.to_bytes(), &proof.0, POSSESSION_DST)?;
        Ok(PossessionVerifiedKey(self.clone()))
    }
}

impl PartialEq for BlsPublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl Eq for BlsPublicKey {}

impl fmt::Debug for BlsPublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BlsPublicKey(0x")?;
        for byte in self.to_bytes() {
            write!(f, "{byte:02x}")?;
        }
        f.write_str(")")
    }
}

/// A BLS signature (or aggregate signature), subgroup-checked.
#[derive(Clone)]
pub struct BlsSignature(Signature);

impl BlsSignature {
    /// Reads a compressed signature (96 bytes) and checks that it lies in
    /// the prime-order subgroup.
    ///
    /// # Errors
    /// [`CryptoError::InvalidSignature`] for a bad encoding,
    /// [`CryptoError::PointNotInGroup`] if not in the subgroup.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        let signature = Signature::uncompress(bytes).map_err(map_error)?;
        signature.validate(false).map_err(map_error)?;
        Ok(Self(signature))
    }

    /// The compressed encoding (96 bytes).
    #[must_use]
    pub fn to_bytes(&self) -> [u8; SIGNATURE_LEN] {
        self.0.compress()
    }
}

impl PartialEq for BlsSignature {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl Eq for BlsSignature {}

impl fmt::Debug for BlsSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BlsSignature(0x")?;
        for byte in self.to_bytes() {
            write!(f, "{byte:02x}")?;
        }
        f.write_str(")")
    }
}

/// Proof that the owner of a BLS public key knows its secret key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofOfPossession(BlsSignature);

impl ProofOfPossession {
    /// Reads a proof (a compressed 96-byte signature).
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
}

/// A public key whose proof of possession has been verified. Only
/// obtainable from [`BlsPublicKey::verify_possession`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PossessionVerifiedKey(BlsPublicKey);

impl PossessionVerifiedKey {
    /// The underlying public key.
    #[must_use]
    pub const fn public_key(&self) -> &BlsPublicKey {
        &self.0
    }
}

/// Combines signatures into one aggregate signature (96 bytes).
///
/// # Errors
/// [`CryptoError::EmptyAggregate`] for an empty list.
pub fn aggregate_signatures(signatures: &[BlsSignature]) -> Result<BlsSignature, CryptoError> {
    let refs: Vec<&Signature> = signatures.iter().map(|s| &s.0).collect();
    if refs.is_empty() {
        return Err(CryptoError::EmptyAggregate);
    }
    // Inputs are subgroup-checked by construction; check again anyway.
    AggregateSignature::aggregate(&refs, true)
        .map(|aggregate| BlsSignature(aggregate.to_signature()))
        .map_err(map_error)
}

/// Verifies an aggregate signature of several keys over the same message
/// (a committee vote).
///
/// # Errors
/// - [`CryptoError::EmptyAggregate`] for an empty key list.
/// - [`CryptoError::DuplicatePublicKey`] if a key appears twice (a vote
///   must not be counted twice).
/// - [`CryptoError::VerificationFailed`] if the signature is invalid.
pub fn fast_aggregate_verify(
    message: &[u8],
    signature: &BlsSignature,
    keys: &[PossessionVerifiedKey],
) -> Result<(), CryptoError> {
    if keys.is_empty() {
        return Err(CryptoError::EmptyAggregate);
    }
    let mut seen = HashSet::with_capacity(keys.len());
    if !keys.iter().all(|key| seen.insert(key.0.to_bytes())) {
        return Err(CryptoError::DuplicatePublicKey);
    }
    let refs: Vec<&PublicKey> = keys.iter().map(|key| &(key.0).0).collect();
    check(
        signature
            .0
            .fast_aggregate_verify(true, message, SIGNATURE_DST, &refs),
    )
}

#[cfg(test)]
impl PossessionVerifiedKey {
    /// Test-only: the Ethereum vectors assume proofs were checked at
    /// registration and do not ship them.
    pub(crate) fn assume_verified_for_test(key: BlsPublicKey) -> Self {
        Self(key)
    }
}

#[cfg(test)]
mod tests;
