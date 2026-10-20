//! Official Ethereum test vectors: key -> address, EIP-55 checksums,
//! EIP-155 chain-ID signatures, and phrase -> address end to end.
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

use common::{LegacyTx, decimal_to_be32, hex, hex_array, to_hex, vector_file};
use toy_crypto::CryptoError;
use toy_crypto::address::{ChecksumState, parse_address, to_checksum_string};
use toy_crypto::eip155::{ChainId, Eip155Signature, recover_eip155, sign_eip155, verify_eip155};
use toy_crypto::hash::keccak256;
use toy_crypto::hd::{DerivationPath, Mnemonic};
use toy_crypto::secp256k1::{RecoverableSignature, SecretKey};

/// ethereum/tests `BasicTests/keyaddrtest.json`: private key -> address.
///
/// The file also has a field `sig_of_emptystring`. Its signed message is
/// not documented, and it does not recover to the address over
/// Keccak-256(""), SHA-256(""), SHA3-256(""), the zero hash or the EIP-191
/// hash of "" (checked 2026-10-20). It is therefore not used as a signature
/// vector; signing is covered by the official EIP-155 example below. What
/// the field does show: the "horse" signature is in high-S form (pre
/// EIP-2), and ToyChain refuses it, and neither carries a chain ID.
#[test]
fn ethereum_tests_key_to_address() {
    let cases: serde_json::Value =
        serde_json::from_str(&vector_file("ethereum-keyaddrtest.json")).unwrap();
    let cases = cases.as_array().unwrap();
    assert_eq!(cases.len(), 2, "vector file changed");
    for case in cases {
        let key = SecretKey::from_bytes(&hex_array(case["key"].as_str().unwrap())).unwrap();
        let expected: [u8; 20] = hex_array(case["addr"].as_str().unwrap());
        assert_eq!(key.address().as_bytes(), &expected, "{}", case["seed"]);

        let sig = &case["sig_of_emptystring"];
        let v: u64 = sig["v"].as_str().unwrap().parse().unwrap();
        let r = decimal_to_be32(sig["r"].as_str().unwrap());
        let s = decimal_to_be32(sig["s"].as_str().unwrap());
        let canonical = RecoverableSignature::from_parts(r, s, v == 28);
        if case["seed"] == "horse" {
            assert_eq!(canonical.unwrap_err(), CryptoError::NonCanonicalSignature);
        }
        let legacy = Eip155Signature { v, r, s };
        assert_eq!(
            recover_eip155(&keccak256(b""), &legacy, ChainId::new(1).unwrap()).unwrap_err(),
            CryptoError::MissingChainId
        );
    }
}

/// The test cases listed in EIP-55 (ERC-55 document).
fn eip55_cases() -> Vec<String> {
    let text = vector_file("erc-55.md");
    let block = text
        .split("# Test Cases")
        .nth(1)
        .expect("test case section");
    let block = block.split("```").nth(1).expect("code block");
    block
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("0x"))
        .map(str::to_string)
        .collect()
}

#[test]
fn eip55_official_vectors() {
    let cases = eip55_cases();
    assert_eq!(cases.len(), 8, "vector file changed");
    for text in &cases {
        // Every listed case, including the "All caps" and "All Lower"
        // groups, is the exact EIP-55 form of its address.
        let parsed = parse_address(text).unwrap();
        assert_eq!(parsed.checksum, ChecksumState::Verified, "{text}");
        assert_eq!(&to_checksum_string(&parsed.address), text);

        // The same address in a case form that is not its EIP-55 form is
        // still valid, but counts as unchecked.
        let digits = &text[2..];
        for other in [digits.to_ascii_lowercase(), digits.to_ascii_uppercase()] {
            if other == digits || !other.chars().any(|c| c.is_ascii_alphabetic()) {
                continue;
            }
            let reparsed = parse_address(&format!("0x{other}")).unwrap();
            assert_eq!(reparsed.address, parsed.address, "{text}");
            assert_eq!(reparsed.checksum, ChecksumState::Absent, "{text}");
        }
    }
}

#[test]
fn eip55_detects_typos_and_bad_format() {
    let good = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed";
    // Flip the case of each letter in turn: every flip must be caught.
    for (i, c) in good.char_indices().skip(2) {
        if !c.is_ascii_alphabetic() {
            continue;
        }
        let flipped: String = good
            .char_indices()
            .map(|(j, d)| {
                if j == i {
                    if d.is_ascii_uppercase() {
                        d.to_ascii_lowercase()
                    } else {
                        d.to_ascii_uppercase()
                    }
                } else {
                    d
                }
            })
            .collect();
        assert_eq!(
            parse_address(&flipped).unwrap_err(),
            CryptoError::BadAddressChecksum,
            "{flipped}"
        );
    }
    for bad in [
        "",
        "0x",
        "5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
        "0X5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAe",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAedd",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAeg",
        " 0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeA\u{e9}",
    ] {
        assert_eq!(
            parse_address(bad).unwrap_err(),
            CryptoError::InvalidAddress,
            "{bad:?}"
        );
    }
}

// Values from the "Example" section of EIP-155 (eip-155.md).
const EIP155_SIGNING_DATA: &str =
    "0xec098504a817c800825208943535353535353535353535353535353535353535880de0b6b3a764000080018080";
const EIP155_SIGNING_HASH: &str =
    "0xdaf5a779ae972f972197303d7b574746c7ef83eadac0f2791ad23db92e4c8e53";
const EIP155_KEY: &str = "0x4646464646464646464646464646464646464646464646464646464646464646";
const EIP155_V: &str = "37";
const EIP155_R: &str =
    "18515461264373351373200002665853028612451056578545711640558177340181847433846";
const EIP155_S: &str =
    "46948507304638947509940763649030358759909902576025900602547168820602576006531";
const EIP155_SIGNED_TX: &str = "0xf86c098504a817c800825208943535353535353535353535353535353535353535880de0b6b3a76400008025a028ef61340bd939bc2195fe537567866003e1a15d3c71ff63e1590620aa636276a067cbe9d8997f761aecb703304b3800ccf555c9f3dc64214b297fb1966a3b6d83";

#[test]
fn eip155_official_example() {
    // The constants above are copied from the EIP; prove it.
    let eip = vector_file("eip-155.md");
    for value in [
        EIP155_SIGNING_DATA,
        EIP155_SIGNING_HASH,
        EIP155_KEY,
        EIP155_R,
        EIP155_S,
        EIP155_SIGNED_TX,
    ] {
        assert!(eip.contains(value), "{value} not in eip-155.md");
    }
    assert!(eip.contains(&format!("({EIP155_V}, {EIP155_R}, {EIP155_S})")));

    // Our RLP test helper reproduces the official signing data.
    let signing_data = LegacyTx::eip155_example().eip155_signing_data(1);
    assert_eq!(signing_data, hex(EIP155_SIGNING_DATA));
    let hash = keccak256(&signing_data);
    assert_eq!(hash.to_string(), EIP155_SIGNING_HASH);

    let key = SecretKey::from_bytes(&hex_array(EIP155_KEY)).unwrap();
    let chain = ChainId::new(1).unwrap();
    let signature = sign_eip155(&key, chain, &hash);
    assert_eq!(signature.v, 37);
    assert_eq!(signature.r, decimal_to_be32(EIP155_R));
    assert_eq!(signature.s, decimal_to_be32(EIP155_S));
    // v, r and s sit at the end of the official signed transaction.
    let tail = format!("25a0{}a0{}", to_hex(&signature.r), to_hex(&signature.s));
    assert!(EIP155_SIGNED_TX.ends_with(&tail));

    assert_eq!(
        recover_eip155(&hash, &signature, chain).unwrap(),
        key.address()
    );
    verify_eip155(&hash, &signature, chain, &key.address()).unwrap();
}

/// Phrase -> address, end to end. Not from a standard document but from
/// the test suites of two major Ethereum wallets/libraries:
/// - alloy (`signer-local/src/mnemonic.rs`, cases "taken from
///   MyCryptoWallet"), path m/44'/60'/0'/0/{index}, with passphrases;
/// - MetaMask HD keyring (`keyring-eth-hd/src/hd-keyring.test.ts`),
///   default path m/44'/60'/0'/0/{0,1}.
#[test]
fn phrase_to_address_wallet_vectors() {
    let alloy_mycrypto = [
        (
            "work man father plunge mystery proud hollow address reunion sauce theory bonus",
            0u32,
            "TREZOR123",
            "0x431a00DA1D54c281AeF638A73121B3D153e0b0F6",
        ),
        (
            "inject danger program federal spice bitter term garbage coyote breeze thought funny",
            1,
            "LEDGER321",
            "0x231a3D0a05d13FAf93078C779FeeD3752ea1350C",
        ),
        (
            "fire evolve buddy tenant talent favorite ankle stem regret myth dream fresh",
            2,
            "",
            "0x1D86AD5eBb2380dAdEAF52f61f4F428C485460E9",
        ),
        (
            "thumb soda tape crunch maple fresh imitate cancel order blind denial giraffe",
            3,
            "",
            "0xFB78b25f69A8e941036fEE2A5EeAf349D81D4ccc",
        ),
    ];
    for (phrase, index, passphrase, expected) in alloy_mycrypto {
        let seed = Mnemonic::parse(phrase).unwrap().to_seed(passphrase);
        let key = seed
            .derive(&DerivationPath::ethereum(index).unwrap())
            .unwrap();
        assert_eq!(to_checksum_string(&key.address()), expected, "{phrase}");
    }

    let metamask = "finish oppose decorate face calm tragic certain desk hour urge dinosaur mango";
    let seed = Mnemonic::parse(metamask).unwrap().to_seed("");
    for (index, expected) in [
        (0, "0x1c96099350f13d558464ec79b9be4445aa0ef579"),
        (1, "0x1b00aed43a693f3a957f9feb5cc08afa031e37a0"),
    ] {
        let key = seed
            .derive(&DerivationPath::ethereum(index).unwrap())
            .unwrap();
        assert_eq!(key.address().to_string(), expected);
        assert_eq!(parse_address(expected).unwrap().address, key.address());
    }
}

#[test]
fn public_key_encodings() {
    let key = SecretKey::from_bytes(&hex_array(EIP155_KEY)).unwrap();
    let public = key.public_key();
    let uncompressed = public.to_uncompressed();
    let compressed = public.to_compressed();
    assert_eq!(uncompressed[0], 0x04);
    assert!(compressed[0] == 0x02 || compressed[0] == 0x03);
    assert_eq!(
        toy_crypto::secp256k1::PublicKey::from_sec1_bytes(&uncompressed).unwrap(),
        public
    );
    assert_eq!(
        toy_crypto::secp256k1::PublicKey::from_sec1_bytes(&compressed).unwrap(),
        public
    );
    for bad in [[0u8; 32], [0xff; 32], common::SECP256K1_N] {
        assert_eq!(
            SecretKey::from_bytes(&bad).unwrap_err(),
            CryptoError::InvalidSecretKey
        );
    }
}
