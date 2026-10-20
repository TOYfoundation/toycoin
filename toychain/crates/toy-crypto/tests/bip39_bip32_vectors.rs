//! Official BIP-39 and BIP-32 test vectors.
//!
//! - BIP-39: `vectors.json` of the reference implementation
//!   (trezor/python-mnemonic), English list, passphrase "TREZOR".
//! - BIP-32: test vectors 1 to 4 from the BIP text (bip-0032.mediawiki).
//!
//! Sources and checksums: `tests/vectors/README.md`.
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use std::str::FromStr;

use common::{hex, vector_file};
use toy_crypto::CryptoError;
use toy_crypto::hd::{DerivationPath, Mnemonic, Seed, WordCount};

/// Private key bytes of an official `xprv` string.
///
/// Decoded twice: by the test's own Base58 decoder (no crate under test
/// involved) and by the `bip32` crate's parser. Both must agree, so a
/// bug in `bip32` cannot hide itself by being wrong in the same way
/// when deriving and when parsing the expected value.
fn xprv_secret(xprv: &str) -> [u8; 32] {
    let independent = common::xprv_secret_independent(xprv);
    let by_crate = bip32::XPrv::from_str(xprv)
        .expect("official xprv")
        .to_bytes();
    assert_eq!(independent, by_crate, "xprv decoding differs: {xprv}");
    independent
}

#[test]
fn bip39_official_vectors() {
    let file: serde_json::Value =
        serde_json::from_str(&vector_file("bip39-trezor-vectors.json")).unwrap();
    let english = file["english"].as_array().unwrap();
    assert_eq!(english.len(), 24, "vector file changed");

    let mut checked_supported = 0;
    let mut checked_rejected = 0;
    for case in english {
        let entropy = hex(case[0].as_str().unwrap());
        let phrase = case[1].as_str().unwrap();
        let seed = hex(case[2].as_str().unwrap());
        let xprv = case[3].as_str().unwrap();
        let words = phrase.split_whitespace().count();

        if words == 12 || words == 24 {
            // Entropy -> phrase.
            let from_entropy = Mnemonic::from_entropy(&entropy).unwrap();
            assert_eq!(from_entropy.phrase().as_str(), phrase);
            // Phrase -> seed (passphrase "TREZOR" as in the reference tests).
            let parsed = Mnemonic::parse(phrase).unwrap();
            assert_eq!(parsed.word_count(), words);
            let computed = parsed.to_seed("TREZOR");
            assert_eq!(computed.as_bytes(), seed.as_slice(), "seed of {phrase}");
            // Seed -> BIP-32 master key.
            let master = computed
                .derive(&DerivationPath::parse("m").unwrap())
                .unwrap();
            assert_eq!(
                *master.to_bytes(),
                xprv_secret(xprv),
                "master key of {phrase}"
            );
            checked_supported += 1;
        } else {
            // 15/18/21 words are valid BIP-39 but the spec allows only 12
            // or 24 (spec section 10). They must be rejected clearly.
            assert_eq!(
                Mnemonic::parse(phrase).unwrap_err(),
                CryptoError::UnsupportedWordCount(words)
            );
            assert_eq!(
                Mnemonic::from_entropy(&entropy).unwrap_err(),
                CryptoError::InvalidEntropyLength(entropy.len())
            );
            checked_rejected += 1;
        }
    }
    assert_eq!((checked_supported, checked_rejected), (16, 8));
}

#[test]
fn bip39_rejects_bad_phrases() {
    let valid = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    assert!(Mnemonic::parse(valid).is_ok());

    // Wrong last word: checksum fails.
    let bad_checksum = valid.replace("about", "abandon");
    assert_eq!(
        Mnemonic::parse(&bad_checksum).unwrap_err(),
        CryptoError::BadPhraseChecksum
    );
    // Unknown word: only its position is reported, never the word.
    let unknown = valid.replacen("abandon", "toycoin", 1);
    let error = Mnemonic::parse(&unknown).unwrap_err();
    assert_eq!(error, CryptoError::UnknownWord(0));
    assert!(!error.to_string().contains("toycoin"));
    // Wrong word counts.
    let eleven: Vec<&str> = valid.split_whitespace().take(11).collect();
    assert_eq!(
        Mnemonic::parse(&eleven.join(" ")).unwrap_err(),
        CryptoError::UnsupportedWordCount(11)
    );
    assert_eq!(
        Mnemonic::parse("").unwrap_err(),
        CryptoError::UnsupportedWordCount(0)
    );
    // Upper case is not in the word list.
    assert!(Mnemonic::parse(&valid.to_uppercase()).is_err());
}

#[test]
fn bip39_generate_roundtrip() {
    for (count, words) in [(WordCount::Twelve, 12), (WordCount::TwentyFour, 24)] {
        let a = Mnemonic::generate(count).unwrap();
        let b = Mnemonic::generate(count).unwrap();
        assert_eq!(a.word_count(), words);
        assert_ne!(a.phrase().as_str(), b.phrase().as_str(), "randomness");
        let reparsed = Mnemonic::parse(&a.phrase()).unwrap();
        assert_eq!(reparsed.to_seed("").as_bytes(), a.to_seed("").as_bytes());
        // A different passphrase gives an unrelated seed.
        assert_ne!(a.to_seed("").as_bytes(), a.to_seed("x").as_bytes());
    }
}

/// One BIP-32 test chain: seed, path, expected extended private key.
struct Bip32Case {
    seed: Vec<u8>,
    path: String,
    xprv: String,
}

/// Reads test vectors 1 to 4 from the official BIP text.
fn bip32_cases() -> Vec<Bip32Case> {
    let text = vector_file("bip-0032.mediawiki");
    let mut cases = Vec::new();
    let mut seed: Option<Vec<u8>> = None;
    let mut path: Option<String> = None;
    for line in text.lines() {
        if line.starts_with("===Test vector 5") {
            break; // Vector 5 lists invalid serialized keys (not derived).
        }
        if let Some(rest) = line.strip_prefix("Seed (hex): ") {
            seed = Some(hex(rest.trim()));
        } else if let Some(rest) = line.strip_prefix("* Chain ") {
            path = Some(rest.trim().replace("<sub>H</sub>", "'"));
        } else if let Some(rest) = line.strip_prefix("** ext prv: ") {
            cases.push(Bip32Case {
                seed: seed.clone().expect("seed before chain"),
                path: path.take().expect("chain before ext prv"),
                xprv: rest.trim().to_string(),
            });
        }
    }
    cases
}

#[test]
fn bip32_official_vectors() {
    let cases = bip32_cases();
    // Vector 1: 6 chains, vector 2: 6, vector 3: 2, vector 4: 3.
    assert_eq!(cases.len(), 17, "parser or vector file changed");
    for case in &cases {
        let seed = Seed::from_bytes(&case.seed).unwrap();
        let path = DerivationPath::parse(&case.path).unwrap();
        let key = seed.derive(&path).unwrap();
        assert_eq!(
            *key.to_bytes(),
            xprv_secret(&case.xprv),
            "path {}",
            case.path
        );
    }
}

#[test]
fn derivation_paths() {
    let path = DerivationPath::ethereum(0).unwrap();
    assert_eq!(path.to_string(), "m/44'/60'/0'/0/0");
    assert_eq!(path.depth(), 5);
    assert_eq!(
        DerivationPath::ethereum(DerivationPath::MAX_INDEX)
            .unwrap()
            .to_string(),
        "m/44'/60'/0'/0/2147483647"
    );
    assert_eq!(
        DerivationPath::ethereum(DerivationPath::MAX_INDEX + 1).unwrap_err(),
        CryptoError::InvalidDerivationPath
    );
    for bad in [
        "",
        "x/0",
        "m/",
        "m/0''",
        "m/a",
        "m/2147483648",
        "m/-1",
        "44'/60'",
    ] {
        assert!(DerivationPath::parse(bad).is_err(), "accepted {bad:?}");
    }
    for bad_len in [0, 15, 33, 63, 65] {
        assert_eq!(
            Seed::from_bytes(&vec![0; bad_len]).unwrap_err(),
            CryptoError::InvalidSeedLength(bad_len)
        );
    }
}

#[test]
fn secrets_are_not_printed() {
    let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let mnemonic = Mnemonic::parse(phrase).unwrap();
    let seed = mnemonic.to_seed("TREZOR");
    let key = seed.derive(&DerivationPath::ethereum(0).unwrap()).unwrap();
    let debug = format!("{mnemonic:?} {seed:?} {key:?}");
    assert!(!debug.contains("abandon"), "{debug}");
    assert!(debug.contains("redacted"), "{debug}");
    let seed_hex = common::to_hex(seed.as_bytes());
    let key_hex = common::to_hex(&*key.to_bytes());
    assert!(!debug.contains(&seed_hex[..16]) && !debug.contains(&key_hex[..16]));
}
