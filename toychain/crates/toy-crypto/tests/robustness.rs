//! Randomized robustness tests for every function that reads data from
//! outside (decision 88). They run on stable Rust in every `cargo test`;
//! the same entry points are fuzzed with cargo-fuzz in CI (`fuzz/`).
//!
//! Each test prints its seed on failure, so a case can be replayed.
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use common::SplitMix64;
use toy_crypto::address::{parse_address, to_checksum_string};
use toy_crypto::bls::{BlsPublicKey, BlsSecretKey, BlsSignature, ProofOfPossession};
use toy_crypto::eip155::{ChainId, Eip155Signature, recover_eip155};
use toy_crypto::hash::keccak256;
use toy_crypto::hd::{DerivationPath, Mnemonic};
use toy_crypto::secp256k1::{PublicKey, RecoverableSignature, SecretKey};
use toy_crypto::vrf::{BlsVrf, BlsVrfProof, Vrf};

const SEED: u64 = 0x5eed_2026_1020_0002;

fn random_text(rng: &mut SplitMix64, max_len: u64) -> String {
    const ALPHABET: &[u8] = b"0123456789abcdefABCDEFxX '/m\t\n-";
    let len = rng.below(max_len + 1) as usize;
    (0..len)
        .map(|_| char::from(ALPHABET[rng.below(ALPHABET.len() as u64) as usize]))
        .collect()
}

#[test]
fn addresses_from_random_text() {
    let mut rng = SplitMix64::new(SEED);
    for _ in 0..20_000 {
        let text = random_text(&mut rng, 50);
        if let Ok(parsed) = parse_address(&text) {
            // Whatever is accepted round-trips to the same address.
            let canonical = to_checksum_string(&parsed.address);
            assert_eq!(parse_address(&canonical).unwrap().address, parsed.address);
        }
    }
    // Random well-formed addresses: checksum form always parses back.
    for _ in 0..2_000 {
        let address = toy_types::Address::from_bytes(rng.array());
        let text = to_checksum_string(&address);
        assert_eq!(
            parse_address(&text).unwrap().address,
            address,
            "seed {SEED}"
        );
        assert_eq!(
            parse_address(&text.to_lowercase()).unwrap().address,
            address
        );
    }
}

#[test]
fn phrases_from_random_words() {
    let mut rng = SplitMix64::new(SEED ^ 1);
    let words: Vec<String> = Mnemonic::parse(
        "legal winner thank year wave sausage worth useful legal winner thank yellow",
    )
    .unwrap()
    .phrase()
    .split(' ')
    .map(str::to_string)
    .collect();
    let mut accepted = 0;
    for _ in 0..5_000 {
        let count = [11u64, 12, 13, 24, 25][rng.below(5) as usize];
        let phrase: Vec<String> = (0..count)
            .map(|_| {
                if rng.below(50) == 0 {
                    random_text(&mut rng, 8)
                } else {
                    words[rng.below(words.len() as u64) as usize].clone()
                }
            })
            .collect();
        if let Ok(mnemonic) = Mnemonic::parse(&phrase.join(" ")) {
            accepted += 1;
            assert!(matches!(mnemonic.word_count(), 12 | 24));
        }
    }
    // A random 12-word phrase has a valid checksum with chance 1/16;
    // most are rejected, some pass. Both paths are exercised.
    assert!(accepted > 0 && accepted < 1_000, "accepted {accepted}");
}

#[test]
fn derivation_paths_from_random_text() {
    let mut rng = SplitMix64::new(SEED ^ 2);
    for _ in 0..20_000 {
        let text = format!("m/{}", random_text(&mut rng, 30));
        if let Ok(path) = DerivationPath::parse(&text) {
            assert_eq!(DerivationPath::parse(&path.to_string()).unwrap(), path);
        }
    }
}

#[test]
fn secp256k1_random_inputs() {
    let mut rng = SplitMix64::new(SEED ^ 3);
    // Random SEC1 bytes and random signatures never panic.
    for _ in 0..5_000 {
        let len = [33usize, 65, 0, 64, 66][rng.below(5) as usize];
        let mut bytes = rng.bytes(len);
        if let Some(first) = bytes.first_mut() {
            *first = [2u8, 3, 4, 0, 5][rng.below(5) as usize];
        }
        let _ = PublicKey::from_sec1_bytes(&bytes);

        let hash = keccak256(&rng.bytes(8));
        if let Ok(sig) =
            RecoverableSignature::from_parts(rng.array(), rng.array(), rng.below(2) == 1)
        {
            let _ = sig.recover(&hash);
        }
        let signature = Eip155Signature {
            v: rng.next_u64() >> rng.below(64),
            r: rng.array(),
            s: rng.array(),
        };
        let _ = recover_eip155(
            &hash,
            &signature,
            ChainId::new(1 + rng.below(1 << 40)).unwrap(),
        );
    }
}

#[test]
fn secp256k1_sign_recover_roundtrip() {
    let mut rng = SplitMix64::new(SEED ^ 4);
    for round in 0..500 {
        let key = SecretKey::generate().unwrap();
        let hash = keccak256(&rng.bytes(32));
        let sig = key.sign_prehash(&hash);
        // Always canonical (low S): from_parts accepts our own output.
        let again = RecoverableSignature::from_parts(*sig.r(), *sig.s(), sig.y_parity()).unwrap();
        assert_eq!(
            again.recover_address(&hash).unwrap(),
            key.address(),
            "round {round}"
        );
        // Restoring the key from bytes gives the same key.
        let restored = SecretKey::from_bytes(&key.to_bytes()).unwrap();
        assert_eq!(restored.address(), key.address());
    }
}

#[test]
fn bls_random_inputs_are_rejected() {
    let mut rng = SplitMix64::new(SEED ^ 5);
    let secret = BlsSecretKey::generate().unwrap();
    let public = secret.public_key();
    for i in 0..3_000 {
        // Random compressed encodings (flag bit set, so the decoder runs).
        let mut pk = rng.bytes(48);
        pk[0] |= 0x80;
        assert!(
            BlsPublicKey::from_bytes(&pk).is_err(),
            "seed {SEED} case {i}"
        );
        let mut sig = rng.bytes(96);
        sig[0] |= 0x80;
        assert!(
            BlsSignature::from_bytes(&sig).is_err(),
            "seed {SEED} case {i}"
        );
        assert!(ProofOfPossession::from_bytes(&sig).is_err());
        assert!(BlsVrfProof::from_bytes(&sig).is_err());
        // Random lengths.
        let len = rng.below(200) as usize;
        let junk = rng.bytes(len);
        let _ = BlsPublicKey::from_bytes(&junk);
        let _ = BlsSignature::from_bytes(&junk);
    }
    // Valid but unrelated signatures never verify.
    for _ in 0..50 {
        let other = BlsSecretKey::generate().unwrap();
        let input = rng.bytes(16);
        let (_, proof) = BlsVrf::prove(&other, &input);
        assert!(BlsVrf::verify(&public, &input, &proof).is_err());
        assert!(public.verify(&input, &other.sign(&input)).is_err());
    }
}
