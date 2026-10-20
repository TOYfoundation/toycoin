//! Shared helpers for the integration tests (test code only).
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_possible_truncation
)]
#![allow(dead_code)]

use std::path::PathBuf;

/// Reads a file from `tests/vectors/`.
pub fn vector_file(name: &str) -> String {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "vectors", name]
        .iter()
        .collect();
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {path:?}: {e}"))
}

/// Decodes hex with or without `0x` prefix.
pub fn hex(text: &str) -> Vec<u8> {
    let digits = text.strip_prefix("0x").unwrap_or(text);
    assert!(digits.len() % 2 == 0, "odd hex length: {text}");
    (0..digits.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&digits[i..i + 2], 16).expect("hex digit"))
        .collect()
}

/// Decodes hex into a fixed-size array.
pub fn hex_array<const N: usize>(text: &str) -> [u8; N] {
    hex(text).try_into().expect("wrong length")
}

/// Lower-case hex without prefix.
pub fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Converts a decimal string into a 32-byte big-endian number.
pub fn decimal_to_be32(text: &str) -> [u8; 32] {
    let mut out = [0u8; 32];
    for c in text.bytes() {
        assert!(c.is_ascii_digit(), "not a decimal digit in {text}");
        let mut carry = u16::from(c - b'0');
        for byte in out.iter_mut().rev() {
            let value = u16::from(*byte) * 10 + carry;
            *byte = (value & 0xff) as u8;
            carry = value >> 8;
        }
        assert_eq!(carry, 0, "decimal value too large: {text}");
    }
    out
}

/// `a - b` for 32-byte big-endian numbers with `a >= b`.
pub fn sub_be32(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let mut borrow = 0i16;
    for i in (0..32).rev() {
        let mut value = i16::from(a[i]) - i16::from(b[i]) - borrow;
        borrow = 0;
        if value < 0 {
            value += 256;
            borrow = 1;
        }
        out[i] = value as u8;
    }
    assert_eq!(borrow, 0, "negative result");
    out
}

/// Secret key inside an official BIP-32 `xprv` string, decoded here
/// without any crate under test (Base58, BIP-32 "Serialization format").
///
/// Layout of the 82 decoded bytes: version (4) ‖ depth (1) ‖ parent
/// fingerprint (4) ‖ child number (4) ‖ chain code (32) ‖ 0x00 ‖ key (32)
/// ‖ checksum (4). The checksum is not recomputed: a decoding mistake
/// makes the test fail, it can never make it pass.
pub fn xprv_secret_independent(xprv: &str) -> [u8; 32] {
    const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let mut number: Vec<u8> = Vec::new(); // big-endian
    for c in xprv.bytes() {
        let digit = ALPHABET
            .iter()
            .position(|&a| a == c)
            .unwrap_or_else(|| panic!("not Base58: {xprv}")) as u32;
        let mut carry = digit;
        for byte in number.iter_mut().rev() {
            let value = u32::from(*byte) * 58 + carry;
            *byte = (value & 0xff) as u8;
            carry = value >> 8;
        }
        while carry > 0 {
            number.insert(0, (carry & 0xff) as u8);
            carry >>= 8;
        }
    }
    assert_eq!(number.len(), 82, "xprv length: {xprv}");
    assert_eq!(
        number[..4],
        [0x04, 0x88, 0xad, 0xe4],
        "mainnet xprv version"
    );
    assert_eq!(number[45], 0x00, "private key marker");
    number[46..78].try_into().expect("32 bytes")
}

/// Order `n` of the secp256k1 group (SEC 2, section 2.4.1).
pub const SECP256K1_N: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
    0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b, 0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36, 0x41, 0x41,
];

/// Small deterministic generator (SplitMix64) for randomized tests.
/// Every failure message includes the seed, so a case can be replayed.
pub struct SplitMix64(u64);

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    pub fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }

    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        (0..len).map(|_| self.next_u64() as u8).collect()
    }

    pub fn array<const N: usize>(&mut self) -> [u8; N] {
        self.bytes(N).try_into().expect("length")
    }
}

/// Minimal RLP encoder, only for building EIP-155 signing data in tests.
/// Anchored to the official EIP-155 example in `eip155_signing_data`.
pub mod rlp {
    fn length_prefix(len: usize, offset: u8) -> Vec<u8> {
        if len < 56 {
            vec![offset + len as u8]
        } else {
            let bytes: Vec<u8> = len
                .to_be_bytes()
                .into_iter()
                .skip_while(|b| *b == 0)
                .collect();
            let mut out = vec![offset + 55 + bytes.len() as u8];
            out.extend(bytes);
            out
        }
    }

    pub fn bytes(data: &[u8]) -> Vec<u8> {
        if data.len() == 1 && data[0] < 0x80 {
            data.to_vec()
        } else {
            let mut out = length_prefix(data.len(), 0x80);
            out.extend_from_slice(data);
            out
        }
    }

    pub fn uint(value: u128) -> Vec<u8> {
        let be: Vec<u8> = value
            .to_be_bytes()
            .into_iter()
            .skip_while(|b| *b == 0)
            .collect();
        bytes(&be)
    }

    pub fn list(items: &[Vec<u8>]) -> Vec<u8> {
        let body: Vec<u8> = items.concat();
        let mut out = length_prefix(body.len(), 0xc0);
        out.extend(body);
        out
    }
}

/// A legacy Ethereum transaction (fields of the EIP-155 example).
pub struct LegacyTx {
    pub nonce: u128,
    pub gas_price: u128,
    pub gas: u128,
    pub to: [u8; 20],
    pub value: u128,
    pub data: Vec<u8>,
}

impl LegacyTx {
    /// The transaction of the EIP-155 example.
    pub fn eip155_example() -> Self {
        Self {
            nonce: 9,
            gas_price: 20_000_000_000,
            gas: 21_000,
            to: [0x35; 20],
            value: 1_000_000_000_000_000_000,
            data: Vec::new(),
        }
    }

    /// EIP-155 signing data: RLP of the six fields plus `chain_id, 0, 0`.
    pub fn eip155_signing_data(&self, chain_id: u64) -> Vec<u8> {
        rlp::list(&[
            rlp::uint(self.nonce),
            rlp::uint(self.gas_price),
            rlp::uint(self.gas),
            rlp::bytes(&self.to),
            rlp::uint(self.value),
            rlp::bytes(&self.data),
            rlp::uint(u128::from(chain_id)),
            rlp::uint(0),
            rlp::uint(0),
        ])
    }
}
