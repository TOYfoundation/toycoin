//! Behaviour of the BLS-based VRF.
//!
//! There are no official vectors for this construction (it is not
//! RFC 9381). The regression vector below pins the current output; the
//! proof itself is a standard BLS signature, so its correctness rests on
//! the official BLS vectors (`src/bls/tests.rs`).
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use common::hex_array;
use toy_crypto::CryptoError;
use toy_crypto::bls::BlsSecretKey;
use toy_crypto::vrf::{BlsVrf, BlsVrfProof, Vrf};

#[test]
fn prove_and_verify() {
    let secret = BlsSecretKey::generate().unwrap();
    let public = secret.public_key();
    let (output, proof) = BlsVrf::prove(&secret, b"epoch 7, block 42");
    assert_eq!(
        BlsVrf::verify(&public, b"epoch 7, block 42", &proof).unwrap(),
        output
    );

    // Deterministic: the owner cannot pick between several outputs.
    let (again, proof_again) = BlsVrf::prove(&secret, b"epoch 7, block 42");
    assert_eq!((again, &proof_again), (output, &proof));

    // Another input gives another output.
    let (other, _) = BlsVrf::prove(&secret, b"epoch 7, block 43");
    assert_ne!(other, output);
}

#[test]
fn wrong_key_input_or_proof_is_rejected() {
    let secret = BlsSecretKey::generate().unwrap();
    let stranger = BlsSecretKey::generate().unwrap();
    let (_, proof) = BlsVrf::prove(&secret, b"input");

    assert_eq!(
        BlsVrf::verify(&stranger.public_key(), b"input", &proof).unwrap_err(),
        CryptoError::VerificationFailed
    );
    assert!(BlsVrf::verify(&secret.public_key(), b"other input", &proof).is_err());
    // The stranger's proof for the same input does not count for the owner.
    let (_, stranger_proof) = BlsVrf::prove(&stranger, b"input");
    assert!(BlsVrf::verify(&secret.public_key(), b"input", &stranger_proof).is_err());
}

#[test]
fn vote_signatures_and_vrf_proofs_are_not_interchangeable() {
    let secret = BlsSecretKey::generate().unwrap();
    let public = secret.public_key();
    // An ordinary signature (vote) over the same bytes is no VRF proof.
    let vote = secret.sign(b"input");
    let as_proof = BlsVrfProof::from_bytes(&vote.to_bytes()).unwrap();
    assert!(BlsVrf::verify(&public, b"input", &as_proof).is_err());
    // A VRF proof is no valid vote.
    let (_, proof) = BlsVrf::prove(&secret, b"input");
    let as_vote = toy_crypto::bls::BlsSignature::from_bytes(&proof.to_bytes()).unwrap();
    assert!(public.verify(b"input", &as_vote).is_err());
}

#[test]
fn outputs_look_uniform() {
    // Coarse check: each of the 256 output bits is set in roughly half of
    // 400 outputs (expected 200, standard deviation 10; bound 6 sigma).
    let secret = BlsSecretKey::generate().unwrap();
    let mut counts = [0u32; 256];
    for i in 0u32..400 {
        let (output, _) = BlsVrf::prove(&secret, &i.to_be_bytes());
        for (bit, count) in counts.iter_mut().enumerate() {
            if output.as_bytes()[bit / 8] >> (bit % 8) & 1 == 1 {
                *count += 1;
            }
        }
    }
    for (bit, count) in counts.iter().enumerate() {
        assert!((140..=260).contains(count), "bit {bit}: {count}");
    }
}

#[test]
fn regression_vector() {
    // Fixed key and input; pins the construction (DST, output hash). If
    // this changes, every VRF output on the network changes: that needs a
    // new VRF version and a decision, never a silent update.
    let secret = BlsSecretKey::from_bytes(&hex_array(
        "47b8192d77bf871b62e87859d653922725724a5c031afeabc60bcef5ff665138",
    ))
    .unwrap();
    let (output, proof) = BlsVrf::prove(&secret, b"ToyChain VRF regression v1");
    assert_eq!(
        BlsVrf::verify(&secret.public_key(), b"ToyChain VRF regression v1", &proof).unwrap(),
        output
    );
    assert_eq!(output.to_string(), REGRESSION_OUTPUT);
}

const REGRESSION_OUTPUT: &str =
    "0xeb720a23b9cb0e2608a9042d3621a57d6606b7306480532746e8bd42db42808a";
