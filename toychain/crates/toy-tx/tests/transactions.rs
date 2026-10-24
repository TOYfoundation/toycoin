//! Transactions: both formats, signatures with chain ID, double spending,
//! overflow, expiry and memo (spec 7.1, 7.2, 10).

#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use common::*;
use toy_crypto::CryptoError;
use toy_crypto::eip155::ChainId;
use toy_emission::Phase;
use toy_state::StateError;
use toy_tx::ethereum::{sign_1559_transfer, sign_legacy_transfer};
use toy_tx::rlp;
use toy_tx::transaction::kind;
use toy_tx::{Action, Block, BlockError, TxError, TxFormat, decode_transaction, sign_transaction};
use toy_types::{Address, Amount};

fn funded() -> (Net, toy_crypto::secp256k1::SecretKey) {
    let mut net = Net::new(Phase::Normal);
    let alice = key(1);
    net.fund(&alice, toy(100));
    (net, alice)
}

#[test]
fn transfer_moves_exactly_the_amount_plus_fee() {
    let (mut net, alice) = funded();
    let bob = key(2).address();
    let before = net.free(&alice.address());
    let r = net.txs(vec![net.transfer(&alice, &bob, toy(5))]);
    assert_eq!(r.counted_tx, 1);
    assert_eq!(
        net.free(&bob),
        toy(5).checked_sub(net.state.activation_deposit()).unwrap()
    );
    assert_eq!(
        net.free(&alice.address()),
        before
            .checked_sub(toy(5))
            .unwrap()
            .checked_sub(r.fees)
            .unwrap()
    );
    net.i0();
}

#[test]
fn double_spend_with_the_same_nonce_is_rejected() {
    let (mut net, alice) = funded();
    let tx1 = net.transfer(&alice, &key(2).address(), toy(1));
    let tx2 = net.transfer(&alice, &key(3).address(), toy(1));
    let before = net.state.clone();
    let err = tx_error(net.try_txs(vec![tx1.clone(), tx2]));
    assert!(
        matches!(err, TxError::State(StateError::NonceMismatch { .. })),
        "{err:?}"
    );
    assert_eq!(net.state, before);
    // The first alone is fine; replaying it later is not.
    net.txs(vec![tx1.clone()]);
    let err = tx_error(net.try_txs(vec![tx1]));
    assert!(
        matches!(err, TxError::State(StateError::NonceMismatch { .. })),
        "{err:?}"
    );
}

#[test]
fn spending_more_than_the_free_balance_is_rejected() {
    let (mut net, alice) = funded();
    let free = net.free(&alice.address());
    // The whole free balance plus fee does not fit; the deposit is never
    // spendable.
    let tx = net.transfer(&alice, &key(2).address(), free);
    let err = tx_error(net.try_txs(vec![tx]));
    assert!(
        matches!(err, TxError::State(StateError::InsufficientFunds { .. })),
        "{err:?}"
    );
    // Two transfers that each fit but not together.
    let half = Amount::from_units(free.units() / 2 + 1);
    let a = net.transfer_n(&alice, 0, &key(2).address(), half);
    let b = net.transfer_n(&alice, 1, &key(3).address(), half);
    let err = tx_error(net.try_txs(vec![a, b]));
    assert!(
        matches!(err, TxError::State(StateError::InsufficientFunds { .. })),
        "{err:?}"
    );
    net.i0();
}

#[test]
fn overflowing_amounts_are_rejected() {
    let (mut net, alice) = funded();
    let tx = net.transfer(&alice, &key(2).address(), Amount::from_units(u128::MAX));
    let err = tx_error(net.try_txs(vec![tx]));
    assert!(
        matches!(err, TxError::State(StateError::InsufficientFunds { .. })),
        "{err:?}"
    );
    // A fee cap and tip at the maximum do not overflow: the charge is
    // the required fee plus tip, capped at the maximum.
    let mut fields = net.fields(&alice);
    fields.max_fee = Amount::from_units(u128::MAX);
    fields.max_tip = Amount::from_units(u128::MAX);
    let tx = net.sign(
        &alice,
        &fields,
        &Action::Transfer {
            to: key(2).address(),
            amount: toy(1),
        },
    );
    let err = tx_error(net.try_txs(vec![tx]));
    assert!(
        matches!(
            err,
            TxError::Arithmetic(_) | TxError::State(StateError::InsufficientFunds { .. })
        ),
        "{err:?}"
    );
    net.i0();
}

#[test]
fn expiry_rules() {
    let (mut net, alice) = funded();
    let next = net.state.height() + 1;
    let with_expiry = |net: &Net, expiry| {
        let mut f = net.fields(&alice);
        f.expiry = expiry;
        net.sign(
            &alice,
            &f,
            &Action::Transfer {
                to: key(2).address(),
                amount: toy(1),
            },
        )
    };
    let err = tx_error(net.try_txs(vec![with_expiry(&net, next - 1)]));
    assert!(matches!(err, TxError::Expired { .. }), "{err:?}");
    let err = tx_error(net.try_txs(vec![with_expiry(&net, next + 1_441)]));
    assert_eq!(err, TxError::ExpiryTooFar);
    // Both limits themselves are valid.
    net.txs(vec![with_expiry(&net, next + 1_440)]);
    let next = net.state.height() + 1;
    net.txs(vec![with_expiry(&net, next)]);
}

#[test]
fn memo_up_to_32_bytes() {
    let (mut net, alice) = funded();
    let mut f = net.fields(&alice);
    f.memo = vec![b'x'; 33];
    let raw = net.sign(&alice, &f, &Action::Donate(toy(1)));
    assert_eq!(decode_transaction(&raw, CHAIN), Err(TxError::MemoTooLong));
    f.memo = vec![b'x'; 32];
    let raw = net.sign(&alice, &f, &Action::Donate(toy(1)));
    let tx = decode_transaction(&raw, CHAIN).unwrap();
    assert_eq!(tx.memo.len(), 32);
    net.txs(vec![raw]);
}

#[test]
fn signatures_for_other_chains_are_rejected() {
    let (mut net, alice) = funded();
    let fields = net.fields(&alice);
    let action = Action::Transfer {
        to: key(2).address(),
        amount: toy(1),
    };
    for other in [
        ChainId::TOYCHAIN_MAINNET,
        ChainId::TOYCHAIN_TESTNET,
        ChainId::new(1).unwrap(),
    ] {
        let raw = sign_transaction(&alice, other, &fields, &action);
        assert_eq!(decode_transaction(&raw, CHAIN), Err(TxError::WrongChainId));
        let legacy = sign_legacy_transfer(
            &alice,
            other,
            fields.nonce,
            1,
            21_000,
            &key(2).address(),
            toy(1),
        );
        assert!(matches!(
            decode_transaction(&legacy, CHAIN),
            Err(TxError::Crypto(CryptoError::WrongChainId { .. }))
        ));
        let typed = sign_1559_transfer(
            &alice,
            other,
            fields.nonce,
            1,
            1,
            21_000,
            &key(2).address(),
            toy(1),
        );
        assert_eq!(
            decode_transaction(&typed, CHAIN),
            Err(TxError::WrongChainId)
        );
    }
    let raw = sign_transaction(&alice, CHAIN, &fields, &action);
    net.txs(vec![raw]);
}

#[test]
fn a_signature_never_counts_for_another_kind() {
    let (net, alice) = funded();
    let fields = net.fields(&alice);
    let bond = sign_transaction(&alice, CHAIN, &fields, &Action::Bond(toy(1)));
    assert_eq!(
        decode_transaction(&bond, CHAIN).unwrap().sender,
        alice.address()
    );
    // Same fields, other type byte: the signature recovers someone else.
    for other in [kind::UNBOND, kind::DONATE] {
        let mut forged = bond.clone();
        forged[0] = other;
        let tx = decode_transaction(&forged, CHAIN).unwrap();
        assert_ne!(tx.sender, alice.address());
    }
}

#[test]
fn ethereum_legacy_and_1559_transfers_execute() {
    let (mut net, alice) = funded();
    let bob = key(2).address();
    let nonce = net.nonce(&alice.address());
    let base = net.state.chain().base_fee_per_byte.units();
    // Legacy: pays everything it offers (gas limit × gas price).
    let price = base * 200 / 21_000 + 1;
    let legacy = sign_legacy_transfer(&alice, CHAIN, nonce, price, 21_000, &bob, toy(3));
    let tx = decode_transaction(&legacy, CHAIN).unwrap();
    assert!(matches!(tx.format, TxFormat::Ethereum(_)));
    let before = net.free(&alice.address());
    let r = net.txs(vec![legacy]);
    assert_eq!(r.fees.units(), price * 21_000);
    assert_eq!(
        net.free(&alice.address()),
        before
            .checked_sub(toy(3))
            .unwrap()
            .checked_sub(r.fees)
            .unwrap()
    );
    // EIP-1559: pays the required fee plus at most the priority fee.
    let base = net.state.chain().base_fee_per_byte.units();
    let typed = sign_1559_transfer(&alice, CHAIN, nonce + 1, 1, base, 21_000, &bob, toy(2));
    let size = typed.len() as u128;
    let r = net.txs(vec![typed]);
    assert_eq!(r.fees.units(), base * size + 21_000);
    assert_eq!(
        net.free(&bob),
        toy(5).checked_sub(net.state.activation_deposit()).unwrap()
    );
    net.i0();
}

#[test]
fn ethereum_contract_calls_and_creation_are_refused() {
    let (net, alice) = funded();
    let nonce = net.nonce(&alice.address());
    // A legacy transaction with call data, signed correctly.
    let build = |to: &[u8], data: &[u8]| {
        let mut fields = Vec::new();
        rlp::encode_u64(&mut fields, nonce);
        rlp::encode_u64(&mut fields, 1);
        rlp::encode_u64(&mut fields, 50_000);
        rlp::encode_bytes(&mut fields, to);
        rlp::encode_u64(&mut fields, 0);
        rlp::encode_bytes(&mut fields, data);
        let mut signing = fields.clone();
        rlp::encode_u64(&mut signing, CHAIN.get());
        rlp::encode_u64(&mut signing, 0);
        rlp::encode_u64(&mut signing, 0);
        let sig = toy_crypto::eip155::sign_eip155(
            &alice,
            CHAIN,
            &toy_crypto::hash::keccak256(&rlp::encode_list(&signing)),
        );
        rlp::encode_u64(&mut fields, sig.v);
        rlp::encode_bytes(&mut fields, &sig.r);
        rlp::encode_bytes(&mut fields, &sig.s);
        rlp::encode_list(&fields)
    };
    let call = build(key(2).address().as_bytes(), &[0xa9, 0x05, 0x9c, 0xbb]);
    assert_eq!(decode_transaction(&call, CHAIN), Err(TxError::NotATransfer));
    let create = build(&[], &[0x60, 0x00]);
    assert_eq!(
        decode_transaction(&create, CHAIN),
        Err(TxError::NotATransfer)
    );
    // Gas limit below 21,000 is refused like on Ethereum.
    let low = sign_legacy_transfer(&alice, CHAIN, nonce, 1, 20_999, &key(2).address(), toy(1));
    assert_eq!(
        decode_transaction(&low, CHAIN),
        Err(TxError::GasLimitTooLow)
    );
}

#[test]
fn unknown_sender_and_oversized_block_are_rejected() {
    let (mut net, _) = funded();
    let stranger = key(9);
    let raw = net.sign(&stranger, &net.fields(&stranger), &Action::Donate(toy(1)));
    let err = tx_error(net.try_txs(vec![raw]));
    assert!(
        matches!(err, TxError::State(StateError::UnknownAccount(_))),
        "{err:?}"
    );
    let huge = Block {
        transactions: vec![vec![0x70; 131_073]],
        tickets: vec![],
    };
    assert_eq!(net.try_block(&huge), Err(BlockError::TooLarge));
}

#[test]
fn malformed_bytes_never_panic() {
    let (net, alice) = funded();
    let raw = net.sign(
        &alice,
        &net.fields(&alice),
        &Action::Transfer {
            to: Address::from_bytes([1; 20]),
            amount: toy(1),
        },
    );
    // Every truncation and every single-byte change is either refused or
    // decodes to a different, still well-formed transaction.
    for cut in 0..raw.len() {
        let _ = decode_transaction(&raw[..cut], CHAIN);
    }
    for i in 0..raw.len() {
        for flip in [0x01u8, 0x80, 0xff] {
            let mut bad = raw.clone();
            bad[i] ^= flip;
            if let Ok(tx) = decode_transaction(&bad, CHAIN) {
                assert!(tx.sender != alice.address() || bad == raw);
            }
        }
    }
}
