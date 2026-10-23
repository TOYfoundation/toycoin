//! Official test vectors (ethereum/tests, commit c67e485f, see
//! `tests/vectors/README.md`):
//!
//! - `RLPTests`: every valid case encodes and decodes exactly, every
//!   invalid case is refused.
//! - `TransactionTests` (212 transactions): every transaction Ethereum
//!   accepts either gives exactly the same sender and hash on ToyChain, or
//!   is refused for a documented ToyChain reason (no chain ID, other
//!   transaction type, contract creation or call data, amount above 128
//!   bits). Every transaction Ethereum refuses is refused too.

#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use serde_json::Value;
use toy_crypto::CryptoError;
use toy_crypto::eip155::ChainId;
use toy_tx::TxError;
use toy_tx::ethereum::parse_ethereum;
use toy_tx::rlp::{self, Item};

fn hex(text: &str) -> Vec<u8> {
    let t = text.strip_prefix("0x").unwrap_or(text);
    (0..t.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&t[i..i + 2], 16).unwrap())
        .collect()
}

fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::from("0x");
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

/// Decimal string to minimal big-endian bytes (for the "#bigint" cases).
fn decimal_to_be(decimal: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    for digit in decimal.bytes() {
        let mut carry = u32::from(digit - b'0');
        for b in bytes.iter_mut().rev() {
            let v = u32::from(*b) * 10 + carry;
            *b = (v & 0xff) as u8;
            carry = v >> 8;
        }
        while carry > 0 {
            bytes.insert(0, (carry & 0xff) as u8);
            carry >>= 8;
        }
    }
    bytes
}

fn encode_json(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::String(s) if s.starts_with('#') => rlp::encode_bytes(out, &decimal_to_be(&s[1..])),
        Value::String(s) => rlp::encode_bytes(out, s.as_bytes()),
        Value::Number(n) => rlp::encode_u64(out, n.as_u64().unwrap()),
        Value::Array(items) => {
            let mut payload = Vec::new();
            for i in items {
                encode_json(i, &mut payload);
            }
            out.extend_from_slice(&rlp::encode_list(&payload));
        }
        other => panic!("unexpected input {other:?}"),
    }
}

/// Compares a decoded item with the JSON input of the vector.
fn matches_json(item: &Item<'_>, value: &Value) -> bool {
    match (item, value) {
        (Item::Bytes(b), Value::String(s)) if s.starts_with('#') => *b == decimal_to_be(&s[1..]),
        (Item::Bytes(b), Value::String(s)) => *b == s.as_bytes(),
        (item @ Item::Bytes(_), Value::Number(n)) => item.as_u64() == Ok(n.as_u64().unwrap()),
        (Item::List(items), Value::Array(values)) => {
            items.len() == values.len() && items.iter().zip(values).all(|(i, v)| matches_json(i, v))
        }
        _ => false,
    }
}

#[test]
fn rlp_official_valid_vectors() {
    let data: Value = serde_json::from_str(include_str!("vectors/rlptest.json")).unwrap();
    let cases = data.as_object().unwrap();
    assert_eq!(cases.len(), 28);
    for (name, case) in cases {
        let expected = hex(case["out"].as_str().unwrap());
        let mut encoded = Vec::new();
        encode_json(&case["in"], &mut encoded);
        assert_eq!(to_hex(&encoded), to_hex(&expected), "encode {name}");
        let item = rlp::decode(&expected).unwrap_or_else(|e| panic!("decode {name}: {e}"));
        assert!(matches_json(&item, &case["in"]), "decode {name}");
    }
}

#[test]
fn rlp_official_invalid_vectors() {
    let data: Value = serde_json::from_str(include_str!("vectors/invalidRLPTest.json")).unwrap();
    let cases = data.as_object().unwrap();
    assert_eq!(cases.len(), 26);
    for (name, case) in cases {
        let bytes = hex(case["out"].as_str().unwrap());
        assert!(rlp::decode(&bytes).is_err(), "{name} must be refused");
    }
}

/// Why ToyChain may refuse a transaction that Ethereum accepts.
fn documented_toychain_refusal(error: &TxError) -> bool {
    matches!(
        error,
        // Legacy signatures without chain ID (pre-EIP-155) can be replayed
        // on every chain (spec 10).
        TxError::Crypto(CryptoError::MissingChainId)
            // Only legacy and type 2 are supported (decision 106).
            | TxError::UnsupportedType(_)
            // No contracts, call data or access lists (decision 36).
            | TxError::NotATransfer
            // Amounts and fee caps are 128-bit on ToyChain.
            | TxError::ValueTooLarge
    )
}

#[test]
fn ethereum_official_transaction_vectors() {
    let data: Value =
        serde_json::from_str(include_str!("vectors/ethereum-transaction-tests.json")).unwrap();
    let tests = data["tests"].as_array().unwrap();
    assert_eq!(tests.len(), 210);
    let chain = ChainId::new(1).unwrap();
    let (mut same_sender, mut refused_valid, mut refused_invalid) = (0, 0, 0);
    let mut accepted_transfers = 0;
    for t in tests {
        let name = format!(
            "{}/{}",
            t["file"].as_str().unwrap(),
            t["name"].as_str().unwrap()
        );
        let raw = hex(t["txbytes"].as_str().unwrap());
        let parsed = parse_ethereum(&raw, chain);
        let transfer = parsed
            .clone()
            .and_then(toy_tx::ethereum::EthereumTx::into_transfer);
        if let Some(sender) = t["sender"].as_str() {
            match parsed {
                Ok(tx) => {
                    assert_eq!(tx.sender.to_string(), sender, "{name}");
                    assert_eq!(tx.hash.to_string(), t["hash"].as_str().unwrap(), "{name}");
                    same_sender += 1;
                    match transfer {
                        Ok(_) => accepted_transfers += 1,
                        Err(e) => assert!(documented_toychain_refusal(&e), "{name}: {e:?}"),
                    }
                }
                Err(e) => {
                    assert!(documented_toychain_refusal(&e), "{name}: {e:?}");
                    refused_valid += 1;
                }
            }
        } else {
            assert!(
                transfer.is_err(),
                "{name} ({}) must be refused",
                t["exception"].as_str().unwrap()
            );
            refused_invalid += 1;
        }
    }
    // Recorded on 2026-10-20 so a silent change of the decoder shows up.
    // 49 valid: 16 parse with the same sender and hash (14 of them are
    // transfers ToyChain accepts, 2 carry amounts above 128 bits), 33 are
    // refused while parsing (32 without chain ID, 1 of type 1). 161
    // invalid: all refused.
    assert_eq!(
        (
            same_sender,
            accepted_transfers,
            refused_valid,
            refused_invalid
        ),
        (16, 14, 33, 161)
    );
}
