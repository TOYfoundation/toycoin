//! Derivation path from wallet settings or import files.
#![no_main]

use libfuzzer_sys::fuzz_target;
use toy_crypto::hd::DerivationPath;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        if let Ok(path) = DerivationPath::parse(text) {
            assert_eq!(DerivationPath::parse(&path.to_string()), Ok(path));
        }
    }
});
