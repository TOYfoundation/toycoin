//! Address text format with the EIP-55 mixed-case checksum.
//!
//! An address is written as `0x` followed by 40 hex digits. EIP-55 puts a
//! checksum into the upper/lower case of the letters: letter `i` is upper
//! case exactly when nibble `i` of `keccak256(lower-case hex)` is 8 or
//! more. A typo then almost always breaks the case pattern.

use toy_types::Address;

use crate::CryptoError;
use crate::hash::keccak256;

/// Whether a parsed address carried a checksum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChecksumState {
    /// The text is exactly the EIP-55 form of the address.
    Verified,
    /// All lower or all upper case that is not the EIP-55 form: valid,
    /// but nothing was checked. Wallets should warn before sending to
    /// such an address.
    Absent,
}

/// An address read from text, together with its checksum state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParsedAddress {
    /// The address bytes.
    pub address: Address,
    /// Whether the text carried a verified checksum.
    pub checksum: ChecksumState,
}

const HEX_LOWER: &[u8; 16] = b"0123456789abcdef";

fn hex_value(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => c.checked_sub(b'0'),
        b'a'..=b'f' => c.checked_sub(b'a').and_then(|v| v.checked_add(10)),
        b'A'..=b'F' => c.checked_sub(b'A').and_then(|v| v.checked_add(10)),
        _ => None,
    }
}

/// Lower-case hex of the address, without prefix (40 ASCII bytes).
fn lower_hex(address: &Address) -> [u8; 40] {
    let mut out = [0u8; 40];
    for (pair, byte) in out.chunks_exact_mut(2).zip(address.as_bytes()) {
        if let [hi, lo] = pair {
            *hi = HEX_LOWER
                .get(usize::from(byte >> 4))
                .copied()
                .unwrap_or(b'0');
            *lo = HEX_LOWER
                .get(usize::from(byte & 0x0f))
                .copied()
                .unwrap_or(b'0');
        }
    }
    out
}

/// Nibble `i` (0..40) of a 32-byte hash, high nibble first.
fn hash_nibble(hash: &[u8; 32], i: usize) -> u8 {
    let byte = hash.get(i / 2).copied().unwrap_or(0);
    if i.is_multiple_of(2) {
        byte >> 4
    } else {
        byte & 0x0f
    }
}

/// Writes the address with its EIP-55 checksum, e.g.
/// `0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed`.
#[must_use]
pub fn to_checksum_string(address: &Address) -> String {
    let lower = lower_hex(address);
    let hash = keccak256(&lower);
    let mut out = String::with_capacity(42);
    out.push_str("0x");
    for (i, &c) in lower.iter().enumerate() {
        let upper = c.is_ascii_alphabetic() && hash_nibble(hash.as_bytes(), i) >= 8;
        out.push(char::from(if upper { c.to_ascii_uppercase() } else { c }));
    }
    out
}

/// Reads an address from text and checks the EIP-55 checksum if present.
///
/// Accepted: `0x` + 40 hex digits. Text equal to the EIP-55 form gives
/// [`ChecksumState::Verified`]. Other mixed case is rejected; other
/// all-lower or all-upper text is accepted with [`ChecksumState::Absent`].
///
/// # Errors
/// - [`CryptoError::InvalidAddress`] for a wrong prefix, length or digit.
/// - [`CryptoError::BadAddressChecksum`] for mixed case that does not
///   match the checksum.
pub fn parse_address(text: &str) -> Result<ParsedAddress, CryptoError> {
    let digits = text
        .strip_prefix("0x")
        .ok_or(CryptoError::InvalidAddress)?
        .as_bytes();
    if digits.len() != 40 {
        return Err(CryptoError::InvalidAddress);
    }
    let mut bytes = [0u8; 20];
    for (byte, pair) in bytes.iter_mut().zip(digits.chunks_exact(2)) {
        let (hi, lo) = match pair {
            [hi, lo] => (hex_value(*hi), hex_value(*lo)),
            _ => (None, None),
        };
        match (hi, lo) {
            (Some(hi), Some(lo)) => *byte = (hi << 4) | lo,
            _ => return Err(CryptoError::InvalidAddress),
        }
    }
    let address = Address::from_bytes(bytes);

    // Text that equals the EIP-55 form is verified, even if its case
    // pattern happens to be all upper or all lower (EIP-55 "All caps").
    let expected = to_checksum_string(&address);
    let checksum = if expected.as_bytes().get(2..) == Some(digits) {
        ChecksumState::Verified
    } else if digits.iter().any(u8::is_ascii_lowercase) && digits.iter().any(u8::is_ascii_uppercase)
    {
        return Err(CryptoError::BadAddressChecksum);
    } else {
        ChecksumState::Absent
    };
    Ok(ParsedAddress { address, checksum })
}
