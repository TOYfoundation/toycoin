//! Transactions from the network in both formats (ToyChain and Ethereum):
//! decoding never panics and is deterministic.
#![no_main]

use libfuzzer_sys::fuzz_target;
use toy_crypto::eip155::ChainId;
use toy_tx::decode_transaction;

fuzz_target!(|data: &[u8]| {
    let chain = ChainId::TOYCHAIN_DEVNET;
    if let Ok(tx) = decode_transaction(data, chain) {
        assert_eq!(tx.size, data.len() as u64);
        assert_eq!(decode_transaction(data, chain).ok(), Some(tx));
    }
});
