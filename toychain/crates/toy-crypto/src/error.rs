use core::fmt;

use crate::eip155::ChainId;

/// Error of a cryptographic operation.
///
/// Error values never contain secret material (keys, phrases, seeds).
/// For an invalid recovery phrase only the position of the bad word is
/// reported, never the word itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CryptoError {
    /// The operating system could not provide secure random bytes.
    Randomness,
    /// The bytes do not form a valid secret key (zero or not below the
    /// group order).
    InvalidSecretKey,
    /// The bytes do not form a valid public key.
    InvalidPublicKey,
    /// The bytes do not form a valid signature.
    InvalidSignature,
    /// The signature is valid but not in canonical "low S" form (EIP-2).
    /// Accepting both forms would make signatures malleable.
    NonCanonicalSignature,
    /// The signature does not verify, or a recovered signer does not
    /// match the expected signer.
    VerificationFailed,
    /// The EIP-155 `v` value is 27 or 28: the signature commits to no
    /// chain ID and could be replayed on any chain.
    MissingChainId,
    /// The EIP-155 `v` value encodes a different chain ID.
    WrongChainId {
        /// The chain ID the verifier expects.
        expected: ChainId,
        /// The chain ID encoded in the signature.
        found: ChainId,
    },
    /// The EIP-155 `v` value is not a valid encoding.
    InvalidV(u64),
    /// The chain ID is 0 or larger than [`ChainId::MAX`].
    InvalidChainId(u64),
    /// The text is not a `0x`-prefixed address of 40 hex digits.
    InvalidAddress,
    /// The address uses mixed case, but the case does not match its
    /// EIP-55 checksum (typo or manipulated address).
    BadAddressChecksum,
    /// The recovery phrase does not have 12 or 24 words.
    UnsupportedWordCount(usize),
    /// The word at this position (0-based) is not in the English BIP-39
    /// word list.
    UnknownWord(usize),
    /// The recovery phrase checksum does not match (typo or wrong order).
    BadPhraseChecksum,
    /// The entropy for a recovery phrase is not 16 or 32 bytes.
    InvalidEntropyLength(usize),
    /// The seed length is not supported (BIP-32 allows 16, 32 or 64 bytes).
    InvalidSeedLength(usize),
    /// The derivation path is malformed or an index is out of range.
    InvalidDerivationPath,
    /// Key derivation produced an invalid key (probability below 2^-127,
    /// BIP-32 says to skip such an index).
    KeyDerivation,
    /// A BLS public key is the point at infinity.
    InfinityPublicKey,
    /// A BLS point is not in the prime-order subgroup.
    PointNotInGroup,
    /// Aggregation or aggregate verification over an empty set.
    EmptyAggregate,
    /// The same BLS public key appears more than once in a set.
    DuplicatePublicKey,
    /// Argon2id (ToyWork) rejected its parameters or inputs.
    WorkFailed,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Randomness => f.write_str("secure random number generator failed"),
            Self::InvalidSecretKey => f.write_str("invalid secret key"),
            Self::InvalidPublicKey => f.write_str("invalid public key"),
            Self::InvalidSignature => f.write_str("invalid signature encoding"),
            Self::NonCanonicalSignature => f.write_str("signature is not in low-S form"),
            Self::VerificationFailed => f.write_str("signature verification failed"),
            Self::MissingChainId => f.write_str("signature has no chain ID (pre-EIP-155)"),
            Self::WrongChainId { expected, found } => write!(
                f,
                "signature is for chain ID {}, expected {}",
                found.get(),
                expected.get()
            ),
            Self::InvalidV(v) => write!(f, "invalid EIP-155 v value {v}"),
            Self::InvalidChainId(id) => write!(f, "invalid chain ID {id}"),
            Self::InvalidAddress => f.write_str("invalid address format"),
            Self::BadAddressChecksum => f.write_str("address checksum (EIP-55) does not match"),
            Self::UnsupportedWordCount(n) => {
                write!(f, "recovery phrase has {n} words, expected 12 or 24")
            }
            Self::UnknownWord(i) => {
                write!(f, "word {} is not in the word list", i.saturating_add(1))
            }
            Self::BadPhraseChecksum => f.write_str("recovery phrase checksum does not match"),
            Self::InvalidEntropyLength(n) => {
                write!(f, "entropy has {n} bytes, expected 16 or 32")
            }
            Self::InvalidSeedLength(n) => write!(f, "seed has {n} bytes, expected 16, 32 or 64"),
            Self::InvalidDerivationPath => f.write_str("invalid derivation path"),
            Self::KeyDerivation => f.write_str("key derivation produced an invalid key"),
            Self::InfinityPublicKey => f.write_str("BLS public key is the point at infinity"),
            Self::PointNotInGroup => f.write_str("BLS point is not in the prime-order subgroup"),
            Self::EmptyAggregate => f.write_str("empty set in BLS aggregation"),
            Self::DuplicatePublicKey => f.write_str("duplicate BLS public key"),
            Self::WorkFailed => f.write_str("ToyWork (Argon2id) computation failed"),
        }
    }
}

impl std::error::Error for CryptoError {}
