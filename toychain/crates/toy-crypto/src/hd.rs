//! Recovery phrases (BIP-39) and key derivation (BIP-32), spec section 10.
//!
//! Path from phrase to account:
//!
//! ```text
//! phrase (12 or 24 English words) + optional passphrase
//!   --BIP-39 (PBKDF2-HMAC-SHA512)--> 64-byte seed
//!   --BIP-32 along m/44'/60'/0'/0/n--> secp256k1 secret key
//!   --Keccak-256--> address
//! ```
//!
//! This is the same path MetaMask and most Ethereum wallets use, so a
//! phrase gives the same addresses everywhere (spec section 10 notes the
//! consequence: the key is shared with other EVM chains).
//!
//! Wraps the `bip39` crate (rust-bitcoin) and the `bip32` crate
//! (iqlusion, on top of `k256`). All secret intermediate values are erased
//! when dropped; see the limitations in `docs/STATUS.md` (M2).

use core::fmt;
use core::str::FromStr;

use zeroize::Zeroizing;

use crate::secp256k1::SecretKey;
use crate::{CryptoError, random};

/// Number of words of a recovery phrase. The spec allows 12 or 24.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WordCount {
    /// 12 words, 128 bits of entropy.
    Twelve,
    /// 24 words, 256 bits of entropy.
    TwentyFour,
}

impl WordCount {
    const fn entropy_len(self) -> usize {
        match self {
            Self::Twelve => 16,
            Self::TwentyFour => 32,
        }
    }
}

/// A BIP-39 recovery phrase (English word list).
///
/// Erased from memory when dropped. `Debug` never prints the words and
/// there is deliberately no `Clone` or `Display`.
pub struct Mnemonic(bip39::Mnemonic);

impl Mnemonic {
    /// Creates a new random phrase.
    ///
    /// # Errors
    /// [`CryptoError::Randomness`] if the secure generator fails.
    pub fn generate(word_count: WordCount) -> Result<Self, CryptoError> {
        let mut entropy = Zeroizing::new([0u8; 32]);
        let len = word_count.entropy_len();
        let buf = entropy.get_mut(..len).ok_or(CryptoError::Randomness)?;
        random::fill(buf)?;
        Self::from_entropy(buf)
    }

    /// Builds the phrase for given entropy (16 or 32 bytes).
    ///
    /// # Errors
    /// [`CryptoError::InvalidEntropyLength`] for other lengths.
    pub fn from_entropy(entropy: &[u8]) -> Result<Self, CryptoError> {
        if entropy.len() != 16 && entropy.len() != 32 {
            return Err(CryptoError::InvalidEntropyLength(entropy.len()));
        }
        bip39::Mnemonic::from_entropy_in(bip39::Language::English, entropy)
            .map(Self)
            .map_err(|_| CryptoError::InvalidEntropyLength(entropy.len()))
    }

    /// Reads a phrase: 12 or 24 English words, separated by whitespace,
    /// lower case, with a valid checksum.
    ///
    /// # Errors
    /// [`CryptoError::UnsupportedWordCount`], [`CryptoError::UnknownWord`]
    /// (with the 0-based position) or [`CryptoError::BadPhraseChecksum`].
    pub fn parse(phrase: &str) -> Result<Self, CryptoError> {
        let count = phrase.split_whitespace().count();
        if count != 12 && count != 24 {
            return Err(CryptoError::UnsupportedWordCount(count));
        }
        bip39::Mnemonic::parse_in(bip39::Language::English, phrase)
            .map(Self)
            .map_err(|error| match error {
                bip39::Error::UnknownWord(position) => CryptoError::UnknownWord(position),
                bip39::Error::BadWordCount(n) => CryptoError::UnsupportedWordCount(n),
                _ => CryptoError::BadPhraseChecksum,
            })
    }

    /// Number of words (12 or 24).
    #[must_use]
    pub fn word_count(&self) -> usize {
        self.0.word_count()
    }

    /// The words separated by single spaces, for showing to the owner
    /// once during backup. The copy is erased when dropped.
    #[must_use]
    pub fn phrase(&self) -> Zeroizing<String> {
        let mut out = Zeroizing::new(String::with_capacity(24 * 9));
        for (i, word) in self.0.words().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            out.push_str(word);
        }
        out
    }

    /// Computes the 64-byte BIP-39 seed. `passphrase` is the optional
    /// extra word ("25th word"); use `""` for none. Different passphrases
    /// give unrelated seeds, without any error.
    #[must_use]
    pub fn to_seed(&self, passphrase: &str) -> Seed {
        let raw = Zeroizing::new(self.0.to_seed(passphrase));
        Seed(Zeroizing::new(raw.to_vec()))
    }
}

impl fmt::Debug for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mnemonic({} words, <redacted>)", self.word_count())
    }
}

/// A BIP-32 seed (normally the 64-byte BIP-39 seed). Erased when dropped.
pub struct Seed(Zeroizing<Vec<u8>>);

impl Seed {
    /// Wraps raw seed bytes (16, 32 or 64 bytes, as allowed by the `bip32`
    /// crate). Used for the official BIP-32 test vectors.
    ///
    /// # Errors
    /// [`CryptoError::InvalidSeedLength`] for other lengths.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        match bytes.len() {
            16 | 32 | 64 => Ok(Self(Zeroizing::new(bytes.to_vec()))),
            n => Err(CryptoError::InvalidSeedLength(n)),
        }
    }

    /// The raw seed bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Derives the secret key at `path` (BIP-32, private derivation).
    ///
    /// # Errors
    /// [`CryptoError::KeyDerivation`] in the practically impossible case
    /// that an intermediate key is invalid (BIP-32: probability below
    /// 2^-127).
    pub fn derive(&self, path: &DerivationPath) -> Result<SecretKey, CryptoError> {
        let xprv = bip32::XPrv::derive_from_path(self.0.as_slice(), &path.0)
            .map_err(|_| CryptoError::KeyDerivation)?;
        let bytes = Zeroizing::new(xprv.to_bytes());
        SecretKey::from_bytes(&bytes)
    }
}

impl fmt::Debug for Seed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Seed({} bytes, <redacted>)", self.0.len())
    }
}

/// A BIP-32 derivation path such as `m/44'/60'/0'/0/0`.
#[derive(Clone, PartialEq, Eq)]
pub struct DerivationPath(bip32::DerivationPath);

impl DerivationPath {
    /// Highest non-hardened index (BIP-32 indices below 2^31).
    pub const MAX_INDEX: u32 = 0x7fff_ffff;

    /// The standard Ethereum account path `m/44'/60'/0'/0/{index}`
    /// (BIP-44, coin type 60), used by MetaMask and most wallets.
    ///
    /// # Errors
    /// [`CryptoError::InvalidDerivationPath`] if `index` > [`Self::MAX_INDEX`].
    pub fn ethereum(index: u32) -> Result<Self, CryptoError> {
        if index > Self::MAX_INDEX {
            return Err(CryptoError::InvalidDerivationPath);
        }
        Self::parse(&format!("m/44'/60'/0'/0/{index}"))
    }

    /// Reads a path in the usual notation (`m/0'/1/2'`), with `'` marking
    /// hardened indices.
    ///
    /// # Errors
    /// [`CryptoError::InvalidDerivationPath`] for malformed text or an
    /// index of 2^31 or more.
    pub fn parse(text: &str) -> Result<Self, CryptoError> {
        bip32::DerivationPath::from_str(text)
            .map(Self)
            .map_err(|_| CryptoError::InvalidDerivationPath)
    }

    /// Number of levels below the master key.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for DerivationPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for DerivationPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DerivationPath({self})")
    }
}
