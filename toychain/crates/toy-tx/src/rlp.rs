//! Strict RLP (Recursive Length Prefix), the encoding of Ethereum
//! transactions (Ethereum Yellow Paper, appendix B).
//!
//! RLP is an encoding, not cryptography; this module follows the
//! specification exactly and is checked against the official vectors of
//! ethereum/tests (`RLPTests`). The decoder is **strict**: every value has
//! exactly one accepted encoding, so a transaction cannot be re-encoded
//! into a second valid form (malleability):
//!
//! - a single byte below `0x80` must be encoded as itself,
//! - the short form must be used for payloads of up to 55 bytes,
//! - lengths have no leading zero bytes,
//! - the whole input must be consumed,
//! - integers ([`Item::as_u64`], [`Item::as_u128`]) have no leading zeros.
//!
//! Nesting is limited to [`MAX_DEPTH`] levels, so hostile input cannot
//! exhaust the stack.

use core::fmt;

/// Deepest list nesting accepted by the decoder.
pub const MAX_DEPTH: usize = 8;

/// An RLP item: a byte string or a list of items.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item<'a> {
    /// A byte string.
    Bytes(&'a [u8]),
    /// A list.
    List(Vec<Item<'a>>),
}

/// RLP decoding error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RlpError {
    /// The input ends before the item.
    UnexpectedEnd,
    /// Bytes follow after the top-level item.
    TrailingBytes,
    /// A value has a longer encoding than necessary.
    NonCanonical,
    /// A length does not fit into memory limits.
    TooLong,
    /// Lists are nested deeper than [`MAX_DEPTH`].
    TooDeep,
    /// A byte string was expected, a list found (or the other way round).
    UnexpectedType,
    /// An integer has a leading zero byte.
    LeadingZero,
    /// An integer is too large for the target type.
    IntegerTooLarge,
    /// A fixed-size field has the wrong length.
    WrongLength,
}

impl fmt::Display for RlpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::UnexpectedEnd => "RLP: unexpected end of input",
            Self::TrailingBytes => "RLP: trailing bytes",
            Self::NonCanonical => "RLP: non-canonical encoding",
            Self::TooLong => "RLP: length too large",
            Self::TooDeep => "RLP: nesting too deep",
            Self::UnexpectedType => "RLP: unexpected item type",
            Self::LeadingZero => "RLP: integer with leading zero",
            Self::IntegerTooLarge => "RLP: integer too large",
            Self::WrongLength => "RLP: field has the wrong length",
        };
        f.write_str(text)
    }
}

impl std::error::Error for RlpError {}

/// Decodes exactly one item that spans the whole input.
///
/// # Errors
/// Any [`RlpError`] for malformed or non-canonical input.
pub fn decode(input: &[u8]) -> Result<Item<'_>, RlpError> {
    let (item, rest) = decode_item(input, 0)?;
    if rest.is_empty() {
        Ok(item)
    } else {
        Err(RlpError::TrailingBytes)
    }
}

fn split(input: &[u8], len: usize) -> Result<(&[u8], &[u8]), RlpError> {
    if input.len() < len {
        return Err(RlpError::UnexpectedEnd);
    }
    Ok(input.split_at(len))
}

/// Reads a big-endian length of `len_of_len` bytes (no leading zero).
fn read_length(input: &[u8], len_of_len: usize) -> Result<(usize, &[u8]), RlpError> {
    let (bytes, rest) = split(input, len_of_len)?;
    if bytes.first() == Some(&0) {
        return Err(RlpError::NonCanonical);
    }
    if len_of_len > core::mem::size_of::<usize>() {
        return Err(RlpError::TooLong);
    }
    let mut len = 0usize;
    for b in bytes {
        len = len
            .checked_shl(8)
            .ok_or(RlpError::TooLong)?
            .checked_add(usize::from(*b))
            .ok_or(RlpError::TooLong)?;
    }
    if len <= 55 {
        // The short form must be used.
        return Err(RlpError::NonCanonical);
    }
    Ok((len, rest))
}

fn decode_item(input: &[u8], depth: usize) -> Result<(Item<'_>, &[u8]), RlpError> {
    let (&prefix, rest) = input.split_first().ok_or(RlpError::UnexpectedEnd)?;
    match prefix {
        0x00..=0x7f => {
            let (byte, _) = input.split_at(1);
            Ok((Item::Bytes(byte), rest))
        }
        0x80..=0xb7 => {
            let len = usize::from(prefix.wrapping_sub(0x80));
            let (payload, rest) = split(rest, len)?;
            if len == 1 && payload.first().is_some_and(|b| *b < 0x80) {
                return Err(RlpError::NonCanonical);
            }
            Ok((Item::Bytes(payload), rest))
        }
        0xb8..=0xbf => {
            let (len, rest) = read_length(rest, usize::from(prefix.wrapping_sub(0xb7)))?;
            let (payload, rest) = split(rest, len)?;
            Ok((Item::Bytes(payload), rest))
        }
        0xc0..=0xf7 => {
            let len = usize::from(prefix.wrapping_sub(0xc0));
            let (payload, rest) = split(rest, len)?;
            Ok((Item::List(decode_list(payload, depth)?), rest))
        }
        0xf8..=0xff => {
            let (len, rest) = read_length(rest, usize::from(prefix.wrapping_sub(0xf7)))?;
            let (payload, rest) = split(rest, len)?;
            Ok((Item::List(decode_list(payload, depth)?), rest))
        }
    }
}

fn decode_list(mut payload: &[u8], depth: usize) -> Result<Vec<Item<'_>>, RlpError> {
    let depth = depth.checked_add(1).ok_or(RlpError::TooDeep)?;
    if depth > MAX_DEPTH {
        return Err(RlpError::TooDeep);
    }
    let mut items = Vec::new();
    while !payload.is_empty() {
        let (item, rest) = decode_item(payload, depth)?;
        items.push(item);
        payload = rest;
    }
    Ok(items)
}

impl<'a> Item<'a> {
    /// The byte string.
    ///
    /// # Errors
    /// [`RlpError::UnexpectedType`] for a list.
    pub fn as_bytes(&self) -> Result<&'a [u8], RlpError> {
        match self {
            Self::Bytes(b) => Ok(b),
            Self::List(_) => Err(RlpError::UnexpectedType),
        }
    }

    /// The list.
    ///
    /// # Errors
    /// [`RlpError::UnexpectedType`] for a byte string.
    pub fn as_list(&self) -> Result<&[Self], RlpError> {
        match self {
            Self::List(items) => Ok(items),
            Self::Bytes(_) => Err(RlpError::UnexpectedType),
        }
    }

    /// A big-endian unsigned integer of at most `max_len` bytes without
    /// leading zero (0 is the empty string).
    ///
    /// # Errors
    /// [`RlpError::LeadingZero`], [`RlpError::IntegerTooLarge`],
    /// [`RlpError::UnexpectedType`].
    pub fn as_uint_bytes(&self, max_len: usize) -> Result<&'a [u8], RlpError> {
        let bytes = self.as_bytes()?;
        if bytes.first() == Some(&0) {
            return Err(RlpError::LeadingZero);
        }
        if bytes.len() > max_len {
            return Err(RlpError::IntegerTooLarge);
        }
        Ok(bytes)
    }

    /// An integer that fits into `u64`.
    ///
    /// # Errors
    /// See [`Self::as_uint_bytes`].
    pub fn as_u64(&self) -> Result<u64, RlpError> {
        let bytes = self.as_uint_bytes(8)?;
        let mut buf = [0u8; 8];
        let start = buf.len().saturating_sub(bytes.len());
        buf.get_mut(start..)
            .ok_or(RlpError::IntegerTooLarge)?
            .copy_from_slice(bytes);
        Ok(u64::from_be_bytes(buf))
    }

    /// An integer that fits into `u128`.
    ///
    /// # Errors
    /// See [`Self::as_uint_bytes`].
    pub fn as_u128(&self) -> Result<u128, RlpError> {
        let bytes = self.as_uint_bytes(16)?;
        let mut buf = [0u8; 16];
        let start = buf.len().saturating_sub(bytes.len());
        buf.get_mut(start..)
            .ok_or(RlpError::IntegerTooLarge)?
            .copy_from_slice(bytes);
        Ok(u128::from_be_bytes(buf))
    }

    /// A byte string of exactly `N` bytes.
    ///
    /// # Errors
    /// [`RlpError::WrongLength`], [`RlpError::UnexpectedType`].
    pub fn as_array<const N: usize>(&self) -> Result<[u8; N], RlpError> {
        self.as_bytes()?
            .try_into()
            .map_err(|_| RlpError::WrongLength)
    }
}

// ---- Encoding -----------------------------------------------------------

fn minimal_be(bytes: &[u8]) -> &[u8] {
    let start = bytes.iter().position(|b| *b != 0).unwrap_or(bytes.len());
    bytes.get(start..).unwrap_or_default()
}

fn encode_header(out: &mut Vec<u8>, len: usize, short: u8, long: u8) {
    if let Ok(small) = u8::try_from(len)
        && small <= 55
    {
        out.push(short.wrapping_add(small));
        return;
    }
    let len_bytes = len.to_be_bytes();
    let minimal = minimal_be(&len_bytes);
    // At most 8 length bytes, so the prefix stays within its range.
    let len_of_len = u8::try_from(minimal.len()).unwrap_or(8);
    out.push(long.wrapping_add(len_of_len));
    out.extend_from_slice(minimal);
}

/// Appends a byte string.
pub fn encode_bytes(out: &mut Vec<u8>, bytes: &[u8]) {
    if let [single] = bytes
        && *single < 0x80
    {
        out.push(*single);
        return;
    }
    encode_header(out, bytes.len(), 0x80, 0xb7);
    out.extend_from_slice(bytes);
}

/// Appends an unsigned integer (big-endian, no leading zeros).
pub fn encode_u64(out: &mut Vec<u8>, value: u64) {
    encode_bytes(out, minimal_be(&value.to_be_bytes()));
}

/// Appends an unsigned integer (big-endian, no leading zeros).
pub fn encode_u128(out: &mut Vec<u8>, value: u128) {
    encode_bytes(out, minimal_be(&value.to_be_bytes()));
}

/// Wraps already encoded items into a list.
#[must_use]
pub fn encode_list(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(payload.len().saturating_add(9));
    encode_header(&mut out, payload.len(), 0xc0, 0xf7);
    out.extend_from_slice(payload);
    out
}

/// Re-encodes a decoded item (canonical, so identical to the input).
pub fn encode_item(out: &mut Vec<u8>, item: &Item<'_>) {
    match item {
        Item::Bytes(b) => encode_bytes(out, b),
        Item::List(items) => {
            let mut payload = Vec::new();
            for i in items {
                encode_item(&mut payload, i);
            }
            out.extend_from_slice(&encode_list(&payload));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_examples() {
        assert_eq!(decode(&[0x80]).unwrap(), Item::Bytes(&[]));
        assert_eq!(decode(&[0x7f]).unwrap(), Item::Bytes(&[0x7f]));
        assert_eq!(decode(&[0x81, 0x80]).unwrap(), Item::Bytes(&[0x80]));
        assert_eq!(decode(&[0x81, 0x7f]), Err(RlpError::NonCanonical));
        assert_eq!(decode(&[0xc0]).unwrap(), Item::List(vec![]));
        assert_eq!(decode(&[0x80, 0x80]), Err(RlpError::TrailingBytes));
        assert_eq!(decode(&[0x82, 0x01]), Err(RlpError::UnexpectedEnd));
        // Long form for a short payload.
        assert_eq!(decode(&[0xb8, 0x01, 0xff]), Err(RlpError::NonCanonical));
        // Leading zero in a length.
        let mut long = vec![0xb9, 0x00, 0x38];
        long.extend([0u8; 56]);
        assert_eq!(decode(&long), Err(RlpError::NonCanonical));
    }

    #[test]
    fn integers_are_strict() {
        assert_eq!(decode(&[0x80]).unwrap().as_u64(), Ok(0));
        assert_eq!(
            decode(&[0x82, 0x00, 0x01]).unwrap().as_u64(),
            Err(RlpError::LeadingZero)
        );
        assert_eq!(
            decode(&[0x00]).unwrap().as_u64(),
            Err(RlpError::LeadingZero)
        );
        let nine = [0x89, 1, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(
            decode(&nine).unwrap().as_u64(),
            Err(RlpError::IntegerTooLarge)
        );
        assert_eq!(decode(&nine).unwrap().as_u128(), Ok(1 << 64));
    }

    #[test]
    fn deep_nesting_is_refused() {
        let mut data = vec![0xc0];
        for _ in 0..MAX_DEPTH {
            data = encode_list(&data);
        }
        assert_eq!(decode(&data), Err(RlpError::TooDeep));
        let mut ok = vec![0xc0];
        for _ in 0..MAX_DEPTH - 1 {
            ok = encode_list(&ok);
        }
        assert!(decode(&ok).is_ok());
    }

    /// Random mutations of valid encodings: decoding never panics, and
    /// every accepted input is canonical (re-encodes to the same bytes).
    #[test]
    fn mutated_inputs_are_refused_or_canonical() {
        let mut seed = 0x1234_5678_9abc_def0_u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let mut samples = Vec::new();
        let mut list = Vec::new();
        for n in [0u128, 1, 0x80, 0xffff, u128::MAX] {
            encode_u128(&mut list, n);
        }
        encode_bytes(&mut list, &[0x55; 60]);
        samples.push(encode_list(&list));
        samples.push(encode_list(&encode_list(&[0xc0])));
        let mut accepted = 0;
        for _ in 0..20_000 {
            let base = &samples[usize::try_from(next() % 2).unwrap()];
            let mut data = base.clone();
            for _ in 0..=(next() % 3) {
                let len = data.len() as u64;
                let i = usize::try_from(next() % len).unwrap();
                match next() % 3 {
                    0 => data[i] ^= u8::try_from(next() & 0xff).unwrap(),
                    1 => data.truncate(i),
                    _ => data.insert(i, u8::try_from(next() & 0xff).unwrap()),
                }
                if data.is_empty() {
                    break;
                }
            }
            if let Ok(item) = decode(&data) {
                let mut again = Vec::new();
                encode_item(&mut again, &item);
                assert_eq!(again, data);
                accepted += 1;
            }
        }
        assert!(accepted > 0);
    }

    #[test]
    fn encode_decode_roundtrip() {
        for n in [
            0u128,
            1,
            0x7f,
            0x80,
            0xff,
            0x100,
            u128::from(u64::MAX),
            u128::MAX,
        ] {
            let mut out = Vec::new();
            encode_u128(&mut out, n);
            assert_eq!(decode(&out).unwrap().as_u128(), Ok(n));
        }
        let payload: Vec<u8> = (0..200u8).collect();
        let mut out = Vec::new();
        encode_bytes(&mut out, &payload);
        assert_eq!(decode(&out).unwrap().as_bytes(), Ok(payload.as_slice()));
        let mut again = Vec::new();
        encode_item(&mut again, &decode(&out).unwrap());
        assert_eq!(again, out);
    }
}
