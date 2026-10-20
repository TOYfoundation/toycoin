//! Signatures from transactions received over the network.
#![no_main]

use libfuzzer_sys::fuzz_target;
use toy_crypto::eip155::{ChainId, Eip155Signature, recover_eip155};
use toy_types::Hash;

fn take<const N: usize>(data: &[u8], at: usize) -> [u8; N] {
    let mut out = [0u8; N];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = data.get(at + i).copied().unwrap_or(0);
    }
    out
}

fuzz_target!(|data: &[u8]| {
    let hash = Hash::from_bytes(take(data, 0));
    let signature = Eip155Signature {
        r: take(data, 32),
        s: take(data, 64),
        v: u64::from_be_bytes(take(data, 96)),
    };
    let chain = u64::from_be_bytes(take(data, 104));
    if let Ok(chain) = ChainId::new(chain) {
        let _ = recover_eip155(&hash, &signature, chain);
    }
});
