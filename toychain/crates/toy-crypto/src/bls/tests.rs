//! Official Ethereum BLS test vectors (ethereum/bls12-381-tests v0.1.2)
//! and behaviour tests for proof of possession and aggregation.
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use serde_json::Value;

use super::*;

const VECTORS: &str = include_str!("../../tests/vectors/bls12-381-tests-v0.1.2.json");

fn hex(text: &str) -> Vec<u8> {
    let digits = text.strip_prefix("0x").unwrap_or(text);
    (0..digits.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&digits[i..i + 2], 16).unwrap())
        .collect()
}

fn category(name: &str) -> Vec<(String, Value)> {
    let all: Value = serde_json::from_str(VECTORS).unwrap();
    let cases: Vec<(String, Value)> = all[name]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    assert!(!cases.is_empty(), "no cases for {name}");
    cases
}

fn public_key(text: &str) -> Result<BlsPublicKey, CryptoError> {
    BlsPublicKey::from_bytes(&hex(text))
}

fn signature(text: &str) -> Result<BlsSignature, CryptoError> {
    BlsSignature::from_bytes(&hex(text))
}

#[test]
fn eth_sign() {
    let cases = category("sign");
    assert_eq!(cases.len(), 10);
    for (name, case) in cases {
        let secret: [u8; 32] = hex(case["input"]["privkey"].as_str().unwrap())
            .try_into()
            .unwrap();
        let message = hex(case["input"]["message"].as_str().unwrap());
        match case["output"].as_str() {
            Some(expected) => {
                let key = BlsSecretKey::from_bytes(&secret).unwrap();
                let sig = key.sign(&message);
                assert_eq!(sig.to_bytes().to_vec(), hex(expected), "{name}");
                key.public_key().verify(&message, &sig).unwrap();
            }
            None => assert_eq!(
                BlsSecretKey::from_bytes(&secret).unwrap_err(),
                CryptoError::InvalidSecretKey,
                "{name}"
            ),
        }
    }
}

#[test]
fn eth_verify() {
    let cases = category("verify");
    assert_eq!(cases.len(), 29);
    for (name, case) in cases {
        let input = &case["input"];
        let message = hex(input["message"].as_str().unwrap());
        let result = public_key(input["pubkey"].as_str().unwrap()).and_then(|pk| {
            let sig = signature(input["signature"].as_str().unwrap())?;
            pk.verify(&message, &sig)
        });
        assert_eq!(result.is_ok(), case["output"].as_bool().unwrap(), "{name}");
    }
}

#[test]
fn eth_aggregate() {
    let cases = category("aggregate");
    assert_eq!(cases.len(), 6);
    for (name, case) in cases {
        let inputs: Vec<BlsSignature> = case["input"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| signature(s.as_str().unwrap()).unwrap())
            .collect();
        match case["output"].as_str() {
            Some(expected) => assert_eq!(
                aggregate_signatures(&inputs).unwrap().to_bytes().to_vec(),
                hex(expected),
                "{name}"
            ),
            None => assert_eq!(
                aggregate_signatures(&inputs).unwrap_err(),
                CryptoError::EmptyAggregate,
                "{name}"
            ),
        }
    }
}

#[test]
fn eth_fast_aggregate_verify() {
    let cases = category("fast_aggregate_verify");
    assert_eq!(cases.len(), 12);
    for (name, case) in cases {
        let input = &case["input"];
        let message = hex(input["message"].as_str().unwrap());
        let result = (|| {
            let keys = input["pubkeys"]
                .as_array()
                .unwrap()
                .iter()
                .map(|k| public_key(k.as_str().unwrap()))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(PossessionVerifiedKey::assume_verified_for_test)
                .collect::<Vec<_>>();
            let sig = signature(input["signature"].as_str().unwrap())?;
            fast_aggregate_verify(&message, &sig, &keys)
        })();
        assert_eq!(result.is_ok(), case["output"].as_bool().unwrap(), "{name}");
    }
}

fn is_infinity(bytes: &[u8]) -> bool {
    bytes.first() == Some(&0xc0) && bytes.iter().skip(1).all(|b| *b == 0)
}

#[test]
fn eth_deserialization() {
    let g1 = category("deserialization_G1");
    assert_eq!(g1.len(), 13);
    for (name, case) in g1 {
        let bytes = hex(case["input"]["pubkey"].as_str().unwrap());
        let result = BlsPublicKey::from_bytes(&bytes);
        if case["output"].as_bool().unwrap() {
            if is_infinity(&bytes) {
                // Stricter than the vector on purpose: an infinity public
                // key decodes in Ethereum but is refused at use (KeyValidate).
                // ToyChain refuses it immediately.
                assert_eq!(
                    result.unwrap_err(),
                    CryptoError::InfinityPublicKey,
                    "{name}"
                );
            } else {
                assert_eq!(result.unwrap().to_bytes().to_vec(), bytes, "{name}");
            }
        } else {
            assert!(result.is_err(), "{name}");
        }
    }

    let g2 = category("deserialization_G2");
    assert_eq!(g2.len(), 15);
    for (name, case) in g2 {
        let bytes = hex(case["input"]["signature"].as_str().unwrap());
        let result = BlsSignature::from_bytes(&bytes);
        if case["output"].as_bool().unwrap() {
            assert_eq!(result.unwrap().to_bytes().to_vec(), bytes, "{name}");
        } else {
            assert!(result.is_err(), "{name}");
        }
    }
}

fn committee(size: usize) -> Vec<(BlsSecretKey, PossessionVerifiedKey)> {
    (0..size)
        .map(|_| {
            let secret = BlsSecretKey::generate().unwrap();
            let proof = secret.prove_possession();
            let verified = secret.public_key().verify_possession(&proof).unwrap();
            (secret, verified)
        })
        .collect()
}

#[test]
fn proof_of_possession() {
    let a = BlsSecretKey::generate().unwrap();
    let b = BlsSecretKey::generate().unwrap();
    let proof_a = a.prove_possession();
    assert!(a.public_key().verify_possession(&proof_a).is_ok());
    // A proof belongs to exactly one key.
    assert_eq!(
        b.public_key().verify_possession(&proof_a).unwrap_err(),
        CryptoError::VerificationFailed
    );
    // An ordinary signature over the key bytes is no proof (separate DST).
    let plain = ProofOfPossession(a.sign(&a.public_key().to_bytes()));
    assert!(a.public_key().verify_possession(&plain).is_err());
    // Roundtrip through bytes.
    let decoded = ProofOfPossession::from_bytes(&proof_a.to_bytes()).unwrap();
    assert_eq!(decoded, proof_a);
}

#[test]
fn committee_vote_with_21_validators() {
    // 21 start validators (spec 5.3) sign the same block hash.
    let members = committee(21);
    let message = b"ToyChain block hash (test)";
    let signatures: Vec<BlsSignature> = members.iter().map(|(s, _)| s.sign(message)).collect();
    let keys: Vec<PossessionVerifiedKey> = members.iter().map(|(_, k)| k.clone()).collect();

    let aggregate = aggregate_signatures(&signatures).unwrap();
    assert_eq!(aggregate.to_bytes().len(), 96); // spec 5.1: one 96-byte signature
    fast_aggregate_verify(message, &aggregate, &keys).unwrap();

    // Wrong message, missing signer, extra key, duplicate key, empty set.
    assert!(fast_aggregate_verify(b"other block", &aggregate, &keys).is_err());
    let partial = aggregate_signatures(&signatures[1..]).unwrap();
    assert!(fast_aggregate_verify(message, &partial, &keys).is_err());
    let mut extra = keys.clone();
    extra.push(committee(1).remove(0).1);
    assert!(fast_aggregate_verify(message, &aggregate, &extra).is_err());
    let mut duplicate = keys[1..].to_vec();
    duplicate.push(keys[1].clone());
    assert_eq!(
        fast_aggregate_verify(message, &partial, &duplicate).unwrap_err(),
        CryptoError::DuplicatePublicKey
    );
    assert_eq!(
        fast_aggregate_verify(message, &aggregate, &[]).unwrap_err(),
        CryptoError::EmptyAggregate
    );
}

#[test]
fn key_encodings_and_secrets() {
    let secret = BlsSecretKey::generate().unwrap();
    let public = secret.public_key();
    assert_eq!(
        BlsPublicKey::from_bytes(&public.to_bytes()).unwrap(),
        public
    );
    let restored = BlsSecretKey::from_bytes(&secret.to_bytes()).unwrap();
    assert_eq!(restored.public_key(), public);
    let sig = secret.sign(b"m");
    assert_eq!(BlsSignature::from_bytes(&sig.to_bytes()).unwrap(), sig);
    // Uncompressed encodings are not accepted (one canonical form only).
    assert!(BlsPublicKey::from_bytes(&[0u8; 96]).is_err());
    assert!(BlsSignature::from_bytes(&[0u8; 192]).is_err());
    // Debug never shows the secret.
    let debug = format!("{secret:?}");
    assert_eq!(debug, "BlsSecretKey(<redacted>)");
}
