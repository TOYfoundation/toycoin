//! BLS keys, signatures and proofs of possession from the network.
#![no_main]

use libfuzzer_sys::fuzz_target;
use toy_crypto::bls::{BlsPublicKey, BlsSignature, ProofOfPossession};

fuzz_target!(|data: &[u8]| {
    let _ = BlsSignature::from_bytes(data);
    let _ = ProofOfPossession::from_bytes(data);
    if data.len() >= 48 + 96 {
        let (key, rest) = data.split_at(48);
        let (sig, message) = rest.split_at(96);
        if let (Ok(key), Ok(sig)) = (BlsPublicKey::from_bytes(key), BlsSignature::from_bytes(sig)) {
            let _ = key.verify(message, &sig);
            if let Ok(proof) = ProofOfPossession::from_bytes(&sig.to_bytes()) {
                let _ = key.verify_possession(&proof);
            }
        }
    } else {
        let _ = BlsPublicKey::from_bytes(data);
    }
});
