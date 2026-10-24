//! RLP from the network: decoding never panics, and every accepted input
//! is canonical (re-encoding gives the same bytes, no malleability).
#![no_main]

use libfuzzer_sys::fuzz_target;
use toy_tx::rlp;

fuzz_target!(|data: &[u8]| {
    if let Ok(item) = rlp::decode(data) {
        let mut again = Vec::new();
        rlp::encode_item(&mut again, &item);
        assert_eq!(again, data, "accepted RLP must be canonical");
    }
});
