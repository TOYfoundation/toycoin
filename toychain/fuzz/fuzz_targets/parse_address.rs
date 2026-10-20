//! Address text from outside (wallet input, RPC, QR codes).
#![no_main]

use libfuzzer_sys::fuzz_target;
use toy_crypto::address::{parse_address, to_checksum_string};

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        if let Ok(parsed) = parse_address(text) {
            let canonical = to_checksum_string(&parsed.address);
            assert_eq!(parse_address(&canonical).map(|p| p.address), Ok(parsed.address));
        }
    }
});
