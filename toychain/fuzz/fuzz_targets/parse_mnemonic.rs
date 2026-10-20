//! Recovery phrase typed or pasted by a user.
#![no_main]

use libfuzzer_sys::fuzz_target;
use toy_crypto::hd::Mnemonic;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        if let Ok(mnemonic) = Mnemonic::parse(text) {
            assert!(matches!(mnemonic.word_count(), 12 | 24));
            let again = Mnemonic::parse(&mnemonic.phrase()).expect("re-parse");
            assert_eq!(again.word_count(), mnemonic.word_count());
        }
    }
});
