//! Replay protection: a signature made for one chain is never accepted on
//! another chain (M2 completion criterion "Signaturen anderer Chain-IDs
//! werden abgelehnt", spec section 10).
//!
//! The scenario uses the transaction from the official EIP-155 example.
//! ToyChain's chain IDs are fixed by decision 116 (mainnet 7630713,
//! testnet 7630714, devnet 7630715); random IDs are tested as well.
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use common::{LegacyTx, SECP256K1_N, SplitMix64, hex_array, sub_be32};
use toy_crypto::CryptoError;
use toy_crypto::eip155::{ChainId, Eip155Signature, recover_eip155, sign_eip155, verify_eip155};
use toy_crypto::hash::keccak256;
use toy_crypto::secp256k1::SecretKey;
use toy_types::Hash;

/// ToyChain mainnet chain ID (decision 116).
const TOYCHAIN: u64 = ChainId::TOYCHAIN_MAINNET.get();
const ETHEREUM: u64 = 1;

fn chain(id: u64) -> ChainId {
    ChainId::new(id).unwrap()
}

/// Signing hash of the EIP-155 example transaction on `chain_id`. Every
/// verifier computes this with its own chain ID.
fn signing_hash(chain_id: ChainId) -> Hash {
    keccak256(&LegacyTx::eip155_example().eip155_signing_data(chain_id.get()))
}

fn victim() -> SecretKey {
    // The key of the official EIP-155 example.
    SecretKey::from_bytes(&hex_array(
        "4646464646464646464646464646464646464646464646464646464646464646",
    ))
    .unwrap()
}

#[test]
fn same_chain_is_accepted() {
    let key = victim();
    for id in [ETHEREUM, TOYCHAIN] {
        let hash = signing_hash(chain(id));
        let signature = sign_eip155(&key, chain(id), &hash);
        verify_eip155(&hash, &signature, chain(id), &key.address()).unwrap();
    }
}

#[test]
fn ethereum_signature_is_rejected_on_toychain() {
    let key = victim();
    // Signed for Ethereum mainnet (exactly the EIP-155 example).
    let ethereum_sig = sign_eip155(&key, chain(ETHEREUM), &signing_hash(chain(ETHEREUM)));
    assert_eq!(ethereum_sig.v, 37);

    // Replayed unchanged on ToyChain: rejected by the chain ID in v.
    let toy = chain(TOYCHAIN);
    assert_eq!(
        recover_eip155(&signing_hash(toy), &ethereum_sig, toy).unwrap_err(),
        CryptoError::WrongChainId {
            expected: toy,
            found: chain(ETHEREUM)
        }
    );
}

#[test]
fn toychain_signature_is_rejected_on_ethereum() {
    let key = victim();
    let toy = chain(TOYCHAIN);
    let toy_sig = sign_eip155(&key, toy, &signing_hash(toy));
    let eth = chain(ETHEREUM);
    assert_eq!(
        recover_eip155(&signing_hash(eth), &toy_sig, eth).unwrap_err(),
        CryptoError::WrongChainId {
            expected: eth,
            found: toy
        }
    );
}

#[test]
fn toychain_networks_reject_each_other() {
    // Mainnet, testnet and devnet must never accept each other's
    // signatures (decision 40): free testnet coins must not move real
    // mainnet coins, even with the same recovery phrase.
    let key = victim();
    let networks = [
        ChainId::TOYCHAIN_MAINNET,
        ChainId::TOYCHAIN_TESTNET,
        ChainId::TOYCHAIN_DEVNET,
    ];
    for signed_on in networks {
        let signature = sign_eip155(&key, signed_on, &signing_hash(signed_on));
        for checked_on in networks {
            let result = recover_eip155(&signing_hash(checked_on), &signature, checked_on);
            if signed_on == checked_on {
                assert_eq!(result.unwrap(), key.address());
            } else {
                assert_eq!(
                    result.unwrap_err(),
                    CryptoError::WrongChainId {
                        expected: checked_on,
                        found: signed_on
                    }
                );
            }
        }
    }
}

#[test]
fn rewriting_v_does_not_help_the_attacker() {
    // The attacker rewrites v so that it encodes ToyChain. The signature
    // then passes the v check, but the signed data contained chain ID 1,
    // while the ToyChain verifier hashes data with its own chain ID. The
    // recovered "signer" is an unrelated address, never the victim.
    let key = victim();
    let ethereum_sig = sign_eip155(&key, chain(ETHEREUM), &signing_hash(chain(ETHEREUM)));
    let toy = chain(TOYCHAIN);
    let parity = ethereum_sig.v == 38;
    let forged = Eip155Signature {
        v: toy.v(parity),
        ..ethereum_sig
    };
    let toy_hash = signing_hash(toy);
    if let Ok(signer) = recover_eip155(&toy_hash, &forged, toy) {
        assert_ne!(signer, key.address());
    }
    assert!(verify_eip155(&toy_hash, &forged, toy, &key.address()).is_err());
}

#[test]
fn signature_without_chain_id_is_rejected() {
    let key = victim();
    // A pre-EIP-155 signature (v = 27/28) over the chain-1 data.
    let hash = signing_hash(chain(ETHEREUM));
    let plain = key.sign_prehash(&hash);
    let legacy = Eip155Signature {
        v: 27 + u64::from(plain.y_parity()),
        r: *plain.r(),
        s: *plain.s(),
    };
    for id in [ETHEREUM, TOYCHAIN] {
        assert_eq!(
            recover_eip155(&hash, &legacy, chain(id)).unwrap_err(),
            CryptoError::MissingChainId
        );
    }
}

#[test]
fn high_s_twin_is_rejected() {
    // ECDSA allows a second valid signature (r, n - s) with flipped parity.
    // Accepting it would make transaction IDs malleable (EIP-2).
    let key = victim();
    let id = chain(TOYCHAIN);
    let hash = signing_hash(id);
    let signature = sign_eip155(&key, id, &hash);
    let parity = (signature.v - 35) % 2 == 1;
    let twin = Eip155Signature {
        v: id.v(!parity),
        r: signature.r,
        s: sub_be32(&SECP256K1_N, &signature.s),
    };
    assert_eq!(
        recover_eip155(&hash, &twin, id).unwrap_err(),
        CryptoError::NonCanonicalSignature
    );
}

#[test]
fn random_keys_and_chain_pairs() {
    let mut rng = SplitMix64::new(0x00c0_ffee_2026_1020);
    for round in 0..200 {
        let key = SecretKey::generate().unwrap();
        let own = chain(1 + rng.below(ChainId::MAX));
        let mut other = chain(1 + rng.below(1_000_000));
        if other == own {
            other = chain(if own.get() == 1 { 2 } else { 1 });
        }
        let hash = signing_hash(own);
        let signature = sign_eip155(&key, own, &hash);
        verify_eip155(&hash, &signature, own, &key.address())
            .unwrap_or_else(|e| panic!("round {round}: own chain rejected: {e}"));
        let other_hash = signing_hash(other);
        assert_eq!(
            recover_eip155(&other_hash, &signature, other).unwrap_err(),
            CryptoError::WrongChainId {
                expected: other,
                found: own
            },
            "round {round}"
        );
        // Even with v rewritten, the victim is never the recovered signer.
        let forged = Eip155Signature {
            v: other.v(signature.v.is_multiple_of(2)),
            ..signature
        };
        assert!(
            verify_eip155(&other_hash, &forged, other, &key.address()).is_err(),
            "round {round}"
        );
    }
}
