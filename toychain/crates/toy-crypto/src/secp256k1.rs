//! Account keys: ECDSA over secp256k1 (spec section 10), wrapping the
//! audited `k256` crate (RustCrypto, reviewed by NCC Group).
//!
//! Signatures are always produced and accepted in canonical "low S" form
//! (EIP-2). Signing is deterministic (RFC 6979).

use core::fmt;

use k256::ecdsa::{RecoveryId, Signature, SigningKey, VerifyingKey};
use toy_types::{Address, Hash};
use zeroize::Zeroizing;

use crate::hash::keccak256;
use crate::{CryptoError, random};

/// A secp256k1 secret key for an account.
///
/// The key material is erased from memory when the value is dropped.
/// `Debug` never prints the key. There is deliberately no `Clone`.
pub struct SecretKey(SigningKey);

impl SecretKey {
    /// Generates a new key from the operating system's secure generator.
    ///
    /// # Errors
    /// [`CryptoError::Randomness`] if the generator fails.
    pub fn generate() -> Result<Self, CryptoError> {
        let mut bytes = Zeroizing::new([0u8; 32]);
        // A random 32-byte value is a valid key with probability
        // 1 - 2^-128; retry in the (practically impossible) other case.
        loop {
            random::fill(bytes.as_mut())?;
            if let Ok(key) = Self::from_bytes(&bytes) {
                return Ok(key);
            }
        }
    }

    /// Wraps a 32-byte big-endian secret scalar.
    ///
    /// # Errors
    /// [`CryptoError::InvalidSecretKey`] if the value is zero or not below
    /// the group order.
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, CryptoError> {
        SigningKey::from_slice(bytes)
            .map(Self)
            .map_err(|_| CryptoError::InvalidSecretKey)
    }

    /// Returns the 32-byte secret scalar. The copy is erased when dropped.
    #[must_use]
    pub fn to_bytes(&self) -> Zeroizing<[u8; 32]> {
        Zeroizing::new(self.0.to_bytes().into())
    }

    /// The matching public key.
    #[must_use]
    pub fn public_key(&self) -> PublicKey {
        PublicKey(*self.0.verifying_key())
    }

    /// The account address of this key.
    #[must_use]
    pub fn address(&self) -> Address {
        self.public_key().address()
    }

    /// Signs a 32-byte hash and returns a recoverable signature in low-S
    /// form.
    ///
    /// `hash` must be the output of a cryptographic hash (Keccak-256) over
    /// the data to be signed, never data chosen directly by someone else.
    #[must_use]
    pub fn sign_prehash(&self, hash: &Hash) -> RecoverableSignature {
        let (signature, recovery_id) = self.0.sign_prehash_recoverable(hash.as_bytes());
        RecoverableSignature::from_k256(&signature, recovery_id)
    }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretKey(<redacted>)")
    }
}

/// A secp256k1 public key (always a valid curve point, never infinity).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PublicKey(VerifyingKey);

impl PublicKey {
    /// Parses a SEC1-encoded key (33 bytes compressed or 65 bytes
    /// uncompressed).
    ///
    /// # Errors
    /// [`CryptoError::InvalidPublicKey`] if the bytes are not a valid point.
    pub fn from_sec1_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        VerifyingKey::from_sec1_bytes(bytes)
            .map(Self)
            .map_err(|_| CryptoError::InvalidPublicKey)
    }

    /// SEC1 uncompressed encoding: `0x04 ‖ x ‖ y` (65 bytes).
    #[must_use]
    pub fn to_uncompressed(&self) -> [u8; 65] {
        let point = self.0.to_sec1_point(false);
        let mut out = [0u8; 65];
        // A valid non-identity point always encodes to exactly 65 bytes.
        if let Some(src) = point.as_bytes().get(..65) {
            out.copy_from_slice(src);
        }
        out
    }

    /// SEC1 compressed encoding: `0x02/0x03 ‖ x` (33 bytes).
    #[must_use]
    pub fn to_compressed(&self) -> [u8; 33] {
        let point = self.0.to_sec1_point(true);
        let mut out = [0u8; 33];
        if let Some(src) = point.as_bytes().get(..33) {
            out.copy_from_slice(src);
        }
        out
    }

    /// The Ethereum-compatible address: the last 20 bytes of the
    /// Keccak-256 hash of the 64-byte public key `x ‖ y` (spec section 10).
    #[must_use]
    pub fn address(&self) -> Address {
        let uncompressed = self.to_uncompressed();
        let hash = keccak256(uncompressed.get(1..).unwrap_or_default());
        let mut address = [0u8; 20];
        if let Some(tail) = hash.as_bytes().get(12..) {
            address.copy_from_slice(tail);
        }
        Address::from_bytes(address)
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PublicKey(0x")?;
        for byte in self.to_compressed() {
            write!(f, "{byte:02x}")?;
        }
        f.write_str(")")
    }
}

/// An ECDSA signature with the parity bit needed to recover the signer.
///
/// Every value of this type is in canonical form: `r` and `s` are in
/// `[1, n-1]` and `s` is in the lower half (EIP-2). Values that are not
/// cannot be constructed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecoverableSignature {
    r: [u8; 32],
    s: [u8; 32],
    y_parity: bool,
}

impl RecoverableSignature {
    fn from_k256(signature: &Signature, recovery_id: RecoveryId) -> Self {
        let (r, s) = signature.split_bytes();
        Self {
            r: r.into(),
            s: s.into(),
            y_parity: recovery_id.is_y_odd(),
        }
    }

    /// Builds a signature from its parts and checks that it is canonical.
    ///
    /// # Errors
    /// - [`CryptoError::InvalidSignature`] if `r` or `s` is zero or not
    ///   below the group order.
    /// - [`CryptoError::NonCanonicalSignature`] if `s` is in the upper half.
    pub fn from_parts(r: [u8; 32], s: [u8; 32], y_parity: bool) -> Result<Self, CryptoError> {
        let signature = Signature::from_scalars(r, s).map_err(|_| CryptoError::InvalidSignature)?;
        if signature.normalize_s() != signature {
            return Err(CryptoError::NonCanonicalSignature);
        }
        Ok(Self { r, s, y_parity })
    }

    /// The `r` value (32 bytes, big-endian).
    #[must_use]
    pub const fn r(&self) -> &[u8; 32] {
        &self.r
    }

    /// The `s` value (32 bytes, big-endian, always in the lower half).
    #[must_use]
    pub const fn s(&self) -> &[u8; 32] {
        &self.s
    }

    /// Whether the y coordinate of the nonce point is odd (Ethereum's
    /// `yParity`, 0 or 1).
    #[must_use]
    pub const fn y_parity(&self) -> bool {
        self.y_parity
    }

    /// Recovers the public key that produced this signature over `hash`.
    ///
    /// # Errors
    /// [`CryptoError::VerificationFailed`] if no valid key can be
    /// recovered.
    pub fn recover(&self, hash: &Hash) -> Result<PublicKey, CryptoError> {
        let signature =
            Signature::from_scalars(self.r, self.s).map_err(|_| CryptoError::InvalidSignature)?;
        let recovery_id = RecoveryId::new(self.y_parity, false);
        VerifyingKey::recover_from_prehash(hash.as_bytes(), &signature, recovery_id)
            .map(PublicKey)
            .map_err(|_| CryptoError::VerificationFailed)
    }

    /// Recovers the signer's address.
    ///
    /// # Errors
    /// See [`Self::recover`].
    pub fn recover_address(&self, hash: &Hash) -> Result<Address, CryptoError> {
        self.recover(hash).map(|key| key.address())
    }
}
