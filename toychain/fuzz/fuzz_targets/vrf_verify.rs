//! VRF proofs from block proposals and lottery draws.
#![no_main]

use std::sync::OnceLock;

use libfuzzer_sys::fuzz_target;
use toy_crypto::bls::{BlsPublicKey, BlsSecretKey};
use toy_crypto::vrf::{BlsVrf, BlsVrfProof, Vrf};

fn public_key() -> &'static BlsPublicKey {
    static KEY: OnceLock<BlsPublicKey> = OnceLock::new();
    KEY.get_or_init(|| {
        BlsSecretKey::from_bytes(&[7u8; 32])
            .expect("fixed test key")
            .public_key()
    })
}

fuzz_target!(|data: &[u8]| {
    let (proof, input) = data.split_at(data.len().min(96));
    if let Ok(proof) = BlsVrfProof::from_bytes(proof) {
        // Random proofs for a key whose secret the fuzzer does not use.
        assert!(BlsVrf::verify(public_key(), input, &proof).is_err());
    }
});
