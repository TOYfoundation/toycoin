//! ToyWork tickets from the network: decoding never panics. (The Argon2id
//! work check is not fuzzed here: it is a fixed library call on a hash.)
#![no_main]

use libfuzzer_sys::fuzz_target;
use toy_crypto::eip155::ChainId;
use toy_tx::ticket::decode_ticket;

fuzz_target!(|data: &[u8]| {
    let _ = decode_ticket(data, ChainId::TOYCHAIN_DEVNET);
});
