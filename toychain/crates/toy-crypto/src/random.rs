//! Secure randomness from the operating system.

use crate::CryptoError;

/// Fills `buf` with random bytes from the operating system's secure
/// generator (`getrandom`, works on Linux, Windows and Android).
pub(crate) fn fill(buf: &mut [u8]) -> Result<(), CryptoError> {
    getrandom::fill(buf).map_err(|_| CryptoError::Randomness)
}
