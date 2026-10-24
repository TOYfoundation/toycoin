//! Accounts through transactions: activation deposit, lowering it,
//! donations, closing with and without donation, and replay protection
//! after re-activation (spec 6.1, 7.1, decisions 73, 74, 76, 79).

#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use common::*;
use toy_emission::Phase;
use toy_state::{CloseBlocker, StateError};
use toy_tx::fees::split_fee;
use toy_tx::{Action, TxError};
use toy_types::Amount;
use toy_types::params::{ACTIVATION_DEPOSIT, ACTIVATION_DEPOSIT_FLOOR};

#[test]
fn first_incoming_amount_must_cover_the_deposit() {
    let mut net = Net::new(Phase::Normal);
    let alice = key(1);
    net.fund(&alice, toy(10));
    let bob = key(2).address();
    let below = ACTIVATION_DEPOSIT
        .checked_sub(Amount::from_units(1))
        .unwrap();
    let err = tx_error(net.try_txs(vec![net.transfer(&alice, &bob, below)]));
    assert!(
        matches!(
            err,
            TxError::State(StateError::BelowActivationDeposit { .. })
        ),
        "{err:?}"
    );
    assert!(net.state.account(&bob).is_none());
    net.txs(vec![net.transfer(&alice, &bob, ACTIVATION_DEPOSIT)]);
    let b = net.state.account(&bob).unwrap();
    assert_eq!((b.deposit(), b.free()), (ACTIVATION_DEPOSIT, Amount::ZERO));
    net.i0();
}

#[test]
fn lowering_the_deposit_frees_the_difference_with_the_next_own_transaction() {
    let mut net = Net::new(Phase::Normal);
    let alice = key(1);
    net.fund(&alice, toy(10));
    net.state
        .lower_activation_deposit(ACTIVATION_DEPOSIT_FLOOR)
        .unwrap();
    let free = net.free(&alice.address());
    let r = net.txs(vec![net.sign(
        &alice,
        &net.fields(&alice),
        &Action::Donate(Amount::ZERO),
    )]);
    let difference = ACTIVATION_DEPOSIT
        .checked_sub(ACTIVATION_DEPOSIT_FLOOR)
        .unwrap();
    assert_eq!(
        net.free(&alice.address()),
        free.checked_add(difference)
            .unwrap()
            .checked_sub(r.fees)
            .unwrap()
    );
    assert_eq!(
        net.state.account(&alice.address()).unwrap().deposit(),
        ACTIVATION_DEPOSIT_FLOOR
    );
    // New addresses need only the lower deposit now.
    net.txs(vec![net.transfer(
        &alice,
        &key(3).address(),
        ACTIVATION_DEPOSIT_FLOOR,
    )]);
    net.i0();
}

#[test]
fn donation_lands_exactly_in_the_refund_pool() {
    let mut net = Net::new(Phase::Normal);
    let alice = key(1);
    net.fund(&alice, toy(10));
    net.fund(&key(250), toy(1)); // the proposer has an account
    let pool = net.state.refund_pool();
    let ctx = net.next_ctx();
    let r = net.txs(vec![net.sign(
        &alice,
        &net.fields(&alice),
        &Action::Donate(toy(3)),
    )]);
    let fee_share = split_fee(r.fees).unwrap().refund_pool;
    assert_eq!(
        net.state.refund_pool(),
        pool.checked_add(toy(3))
            .unwrap()
            .checked_add(ctx.mint.refund_pool)
            .unwrap()
            .checked_add(fee_share)
            .unwrap()
    );
    net.i0();
    // Not reversible: no transaction kind takes TOY out of the pool. The
    // pool only shrinks by the surplus at an epoch start (decision 120),
    // and that goes to the lottery, never back to the donor.
    let alice_free = net.free(&alice.address());
    net.to_next_epoch();
    assert_eq!(net.free(&alice.address()), alice_free);
}

#[test]
fn closing_with_and_without_donation() {
    let mut net = Net::new(Phase::Normal);
    let alice = key(1);
    let bob = key(2);
    let carol = key(3);
    net.fund(&alice, toy(10));
    net.fund(&bob, toy(10));
    net.fund(&carol, toy(1));

    // With bonded stake an account cannot be closed.
    net.txs(vec![net.sign(
        &alice,
        &net.fields(&alice),
        &Action::Bond(toy(1)),
    )]);
    let close = Action::CloseAccount {
        target: Some(carol.address()),
        donation: Amount::ZERO,
    };
    let err = tx_error(net.try_txs(vec![net.sign(&alice, &net.fields(&alice), &close)]));
    assert_eq!(
        err,
        TxError::State(StateError::CannotClose(CloseBlocker::Stake))
    );

    // Bob closes with a partial donation.
    let pool = net.state.refund_pool();
    let carol_free = net.free(&carol.address());
    let bob_total = net
        .free(&bob.address())
        .checked_add(ACTIVATION_DEPOSIT)
        .unwrap();
    let ctx = net.next_ctx();
    let close = Action::CloseAccount {
        target: Some(carol.address()),
        donation: toy(4),
    };
    let r = net.txs(vec![net.sign(&bob, &net.fields(&bob), &close)]);
    assert!(net.state.account(&bob.address()).is_none());
    let rest = bob_total
        .checked_sub(r.fees)
        .unwrap()
        .checked_sub(toy(4))
        .unwrap();
    assert_eq!(
        net.free(&carol.address()),
        carol_free.checked_add(rest).unwrap()
    );
    let fee_to_pool = split_fee(r.fees).unwrap();
    // The proposer has no account here: its share goes to the pool too.
    let pool_gain = toy(4)
        .checked_add(fee_to_pool.refund_pool)
        .unwrap()
        .checked_add(fee_to_pool.proposer)
        .unwrap()
        .checked_add(ctx.mint.refund_pool)
        .unwrap();
    assert_eq!(
        net.state.refund_pool(),
        pool.checked_add(pool_gain).unwrap()
    );
    net.i0();

    // Carol donates everything: no target needed.
    let total = net
        .free(&carol.address())
        .checked_add(ACTIVATION_DEPOSIT)
        .unwrap();
    let mut fields = net.fields(&carol);
    let fee = Amount::from_units(1_000_000_000_000_000);
    fields.max_fee = fee;
    fields.max_tip = fee;
    let everything = Action::CloseAccount {
        target: None,
        donation: total.checked_sub(fee).unwrap(),
    };
    net.txs(vec![net.sign(&carol, &fields, &everything)]);
    assert!(net.state.account(&carol.address()).is_none());
    net.i0();
}

#[test]
fn an_old_transaction_stays_invalid_after_reactivation() {
    let mut net = Net::new(Phase::Normal);
    let alice = key(1);
    let bob = key(2);
    net.fund(&alice, toy(10));
    net.fund(&bob, toy(1));
    // Alice signs a transfer but it is never included.
    let old = net.transfer(&alice, &bob.address(), toy(1));
    // She closes her account to Bob...
    let close = Action::CloseAccount {
        target: Some(bob.address()),
        donation: Amount::ZERO,
    };
    net.txs(vec![net.sign(&alice, &net.fields(&alice), &close)]);
    // ...and later receives TOY again: a new account with a new nonce base.
    net.empty(3);
    net.txs(vec![net.transfer(&bob, &alice.address(), toy(2))]);
    let new_nonce = net.nonce(&alice.address());
    assert_eq!(new_nonce, net.state.height() << 16);
    // The old signed transaction can never be booked.
    let err = tx_error(net.try_txs(vec![old]));
    assert!(
        matches!(err, TxError::State(StateError::NonceMismatch { .. })),
        "{err:?}"
    );
    net.i0();
}
