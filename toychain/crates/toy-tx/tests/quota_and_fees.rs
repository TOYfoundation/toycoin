//! Free quota, refund pool and fees (spec 7.2, decisions 106, 117, 118,
//! 120).

#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use common::*;
use toy_crypto::secp256k1::SecretKey;
use toy_emission::Phase;
use toy_state::Violation;
use toy_tx::fees::{progressive_factor, refund_pool_cap, split_fee};
use toy_tx::{Action, TxError, decode_transaction};
use toy_types::Amount;
use toy_types::params::{BASE_FEE_MIN_PER_BYTE, MAX_BLOCK_BYTES};

/// Network in epoch 1 with `alice` funded in epoch 0 (old enough for the
/// free quota) and a funded proposer.
fn in_epoch_one() -> (Net, SecretKey) {
    let mut net = Net::new(Phase::Normal);
    let alice = key(1);
    net.fund(&alice, toy(1_000));
    let proposer = key(250);
    net.fund(&proposer, toy(1));
    // Receiver of small amounts in the tests.
    net.fund(&key(2), toy(1));
    net.to_next_epoch();
    assert_eq!(net.state.epoch(), 1);
    (net, alice)
}

#[test]
fn quota_follows_the_utilization_of_the_previous_epoch() {
    let capacity = 1_440 * MAX_BLOCK_BYTES;
    for (used, expected) in [
        (0, 100),
        (capacity / 4, 75),
        (capacity / 2, 50),
        (capacity * 9 / 10, 10),
        (capacity, 10),
    ] {
        let mut net = Net::new(Phase::Normal);
        net.empty(5);
        net.state.chain_mut().epoch_tx_bytes = used;
        net.to_next_epoch();
        assert_eq!(net.state.chain().epoch_quota, expected, "used {used}");
    }
}

#[test]
fn free_transactions_up_to_the_quota_then_refused() {
    let mut net = Net::new(Phase::Normal);
    let alice = key(1);
    net.fund(&alice, toy(10));
    net.fund(&key(2), toy(1));
    // 90 % utilization in epoch 0: quota 10 in epoch 1.
    net.state.chain_mut().epoch_tx_bytes = 1_440 * MAX_BLOCK_BYTES * 9 / 10;
    net.to_next_epoch();
    assert_eq!(net.state.chain().epoch_quota, 10);
    let bob = key(2).address();
    let ten: Vec<_> = (0..10)
        .map(|i| net.free_transfer_n(&alice, i, &bob, toy(0)))
        .collect();
    let r = net.txs(ten);
    assert_eq!((r.free_tx, r.counted_tx), (10, 0));
    let eleventh = net.free_transfer_n(&alice, 0, &bob, toy(0));
    assert_eq!(
        tx_error(net.try_txs(vec![eleventh])),
        TxError::QuotaExhausted
    );
    // Paying still works.
    net.txs(vec![net.transfer(&alice, &bob, toy(1))]);
    // The next epoch brings a new quota.
    net.to_next_epoch();
    net.txs(vec![net.free_transfer_n(&alice, 0, &bob, toy(0))]);
    net.i0();
}

#[test]
fn free_transactions_need_an_account_age_of_100_blocks() {
    let (mut net, alice) = in_epoch_one();
    let young = key(3);
    net.txs(vec![net.transfer(&alice, &young.address(), toy(2))]);
    let tx = net.free_transfer_n(&young, 0, &alice.address(), toy(0));
    assert_eq!(tx_error(net.try_txs(vec![tx])), TxError::AccountTooYoung);
    net.empty(98);
    let tx = net.free_transfer_n(&young, 0, &alice.address(), toy(0));
    assert_eq!(
        tx_error(net.try_txs(vec![tx.clone()])),
        TxError::AccountTooYoung
    );
    net.empty(1);
    net.txs(vec![tx]);
}

#[test]
fn free_transactions_move_no_fee_and_use_the_refund_budget() {
    let (mut net, alice) = in_epoch_one();
    let bob = key(2).address();
    let pool = net.state.refund_pool();
    let lottery = net.state.fee_lottery();
    let free = net.free(&alice.address());
    let ctx = net.next_ctx();
    let tx = net.free_transfer_n(&alice, 0, &bob, toy(1));
    let size = tx.len() as u128;
    let r = net.txs(vec![tx]);
    assert_eq!(r.free_tx, 1);
    assert_eq!(r.fees, Amount::ZERO);
    assert_eq!(
        net.free(&alice.address()),
        free.checked_sub(toy(1)).unwrap()
    );
    // Refunded fees are not redistributed (decision 106): pool and fee
    // lottery change only by the block emission.
    assert_eq!(
        net.state.refund_pool(),
        pool.checked_add(ctx.mint.refund_pool).unwrap()
    );
    assert_eq!(net.state.fee_lottery(), lottery);
    let nominal = net.state.chain().refund_used;
    assert_eq!(nominal.units(), BASE_FEE_MIN_PER_BYTE.units() * size);
    assert_eq!(r.refunded, nominal);
    net.i0();
}

#[test]
fn refund_budget_limits_free_transactions() {
    let (mut net, alice) = in_epoch_one();
    // The budget is 1/30 of the pool at the epoch start.
    assert_eq!(
        net.state.chain().refund_budget,
        refund_pool_cap(BASE_FEE_MIN_PER_BYTE, 1_440)
            .unwrap()
            .checked_div(30)
            .unwrap()
    );
    let bob = key(2).address();
    let probe = net.free_transfer_n(&alice, 0, &bob, toy(0));
    let one = BASE_FEE_MIN_PER_BYTE
        .checked_mul(probe.len() as u128 + 2)
        .unwrap();
    net.state.chain_mut().refund_budget = one.checked_mul(2).unwrap();
    net.txs(vec![
        net.free_transfer_n(&alice, 0, &bob, toy(0)),
        net.free_transfer_n(&alice, 1, &bob, toy(0)),
    ]);
    let third = net.free_transfer_n(&alice, 0, &bob, toy(0));
    assert_eq!(
        tx_error(net.try_txs(vec![third])),
        TxError::RefundBudgetExhausted
    );
}

#[test]
fn free_transactions_stand_behind_paid_ones() {
    let (mut net, alice) = in_epoch_one();
    let carol = key(4);
    net.txs(vec![net.transfer(&alice, &carol.address(), toy(5))]);
    net.empty(100);
    let bob = key(2).address();
    let free = net.free_transfer_n(&carol, 0, &bob, toy(0));
    let paid = net.transfer(&alice, &bob, toy(1));
    assert_eq!(
        tx_error(net.try_txs(vec![free.clone(), paid.clone()])),
        TxError::PaidAfterFree
    );
    let r = net.txs(vec![paid, free]);
    assert_eq!((r.counted_tx, r.free_tx), (1, 1));
}

#[test]
fn free_transactions_use_at_most_half_a_block() {
    let mut net = Net::new(Phase::Normal);
    let alice = key(1);
    net.fund(&alice, toy(100));
    let senders: Vec<_> = (10..16).map(key).collect();
    let funding: Vec<_> = senders
        .iter()
        .enumerate()
        .map(|(i, s)| net.transfer_n(&alice, i as u64, &s.address(), toy(1)))
        .collect();
    net.txs(funding);
    net.to_next_epoch();
    let quota = net.state.chain().epoch_quota;
    assert!(quota >= 99);
    // Free transfers with a 32-byte memo until more than 64 KB.
    let mut txs = Vec::new();
    let mut bytes = 0;
    'outer: for s in &senders {
        for i in 0..quota {
            let mut f = net.fields(s);
            f.nonce += i;
            f.max_fee = Amount::ZERO;
            f.memo = vec![7; 32];
            let tx = net.sign(
                s,
                &f,
                &Action::Transfer {
                    to: alice.address(),
                    amount: Amount::ZERO,
                },
            );
            bytes += tx.len() as u64;
            txs.push(tx);
            if bytes > MAX_BLOCK_BYTES / 2 {
                break 'outer;
            }
        }
    }
    assert_eq!(
        tx_error(net.try_txs(txs.clone())),
        TxError::FreeBytesExceeded
    );
    txs.pop();
    let r = net.txs(txs);
    assert!(r.tx_bytes <= MAX_BLOCK_BYTES / 2);
    assert!(r.free_tx > 400);
    net.i0();
}

#[test]
fn penalized_quota_still_allows_paid_transfers() {
    let (mut net, alice) = in_epoch_one();
    net.state
        .apply_violation(&alice.address(), Violation::MassAccountCreation)
        .unwrap();
    let bob = key(2).address();
    let free = net.free_transfer_n(&alice, 0, &bob, toy(1));
    assert_eq!(tx_error(net.try_txs(vec![free])), TxError::QuotaLocked);
    let before = net.free(&bob);
    let r = net.txs(vec![net.transfer(&alice, &bob, toy(1))]);
    assert_eq!(r.counted_tx, 1);
    assert!(net.free(&bob) > before);
    net.i0();
}

#[test]
fn progressive_fee_doubles_every_10_transactions_and_is_split_exactly() {
    let (mut net, alice) = in_epoch_one();
    let bob = key(2).address();
    let base = net.state.chain().base_fee_per_byte;
    let txs: Vec<_> = (0..25)
        .map(|i| net.transfer_n(&alice, i, &bob, Amount::from_units(1)))
        .collect();
    let mut expected = Amount::ZERO;
    let (mut lottery, mut proposer) = (Amount::ZERO, Amount::ZERO);
    for (i, raw) in txs.iter().enumerate() {
        let size = decode_transaction(raw, CHAIN).unwrap().size;
        let fee = base
            .checked_mul(u128::from(size))
            .unwrap()
            .checked_mul(progressive_factor(i as u64 + 1))
            .unwrap();
        expected = expected.checked_add(fee).unwrap();
        let split = split_fee(fee).unwrap();
        lottery = lottery.checked_add(split.fee_lottery).unwrap();
        proposer = proposer.checked_add(split.proposer).unwrap();
    }
    let before_lottery = net.state.fee_lottery();
    let before_proposer = net.free(&net.proposer);
    let r = net.txs(txs);
    assert_eq!(r.fees, expected);
    assert_eq!(
        net.state.fee_lottery(),
        before_lottery.checked_add(lottery).unwrap()
    );
    assert_eq!(
        net.free(&net.proposer),
        before_proposer.checked_add(proposer).unwrap()
    );
    // The 21st to 25th transactions paid four times the base.
    assert_eq!(progressive_factor(21), 4);
    net.i0();
}

#[test]
fn fee_cap_and_tip() {
    let (mut net, alice) = in_epoch_one();
    let bob = key(2).address();
    let mut f = net.fields(&alice);
    f.max_fee = Amount::from_units(1);
    let low = net.sign(
        &alice,
        &f,
        &Action::Transfer {
            to: bob,
            amount: toy(1),
        },
    );
    assert!(matches!(
        tx_error(net.try_txs(vec![low])),
        TxError::FeeTooLow { .. }
    ));
    // A tip is charged on top of the required fee, within the cap.
    let mut f = net.fields(&alice);
    f.max_tip = Amount::from_units(12_345);
    let tx = net.sign(
        &alice,
        &f,
        &Action::Transfer {
            to: bob,
            amount: toy(1),
        },
    );
    let size = u128::from(decode_transaction(&tx, CHAIN).unwrap().size);
    let base = net.state.chain().base_fee_per_byte.units();
    let r = net.txs(vec![tx]);
    assert_eq!(r.fees.units(), base * size + 12_345);
}

#[test]
fn base_fee_moves_after_each_block() {
    let mut net = Net::new(Phase::Normal);
    net.empty(2);
    assert_eq!(net.state.chain().base_fee_per_byte, BASE_FEE_MIN_PER_BYTE);
    net.state.chain_mut().base_fee_per_byte = Amount::from_units(8_000_000_000_000);
    net.empty(1);
    // An empty block lowers the base fee by 1/8.
    assert_eq!(
        net.state.chain().base_fee_per_byte,
        Amount::from_units(7_000_000_000_000)
    );
}

#[test]
fn refund_pool_surplus_moves_to_the_work_lottery() {
    let mut net = Net::new(Phase::Normal);
    // Run to the last block of epoch 0.
    while net.next_ctx().epoch == 0 {
        net.empty(1);
    }
    let pool = net.state.refund_pool();
    let cap = refund_pool_cap(net.state.chain().base_fee_per_byte, 1_440).unwrap();
    assert!(pool > cap, "epoch 0 fills the pool with 10 % of 50,500 TOY");
    let lottery = net.state.work_lottery();
    let ctx = net.next_ctx();
    let r = net.block(toy_tx::Block::default());
    assert!(r.new_epoch);
    assert_eq!(r.pool_surplus, pool.checked_sub(cap).unwrap());
    assert_eq!(
        net.state.refund_pool(),
        cap.checked_add(ctx.mint.refund_pool).unwrap()
    );
    let prizes = Amount::checked_sum(r.prizes.iter().map(|(_, p)| *p)).unwrap();
    assert_eq!(
        net.state.work_lottery(),
        lottery
            .checked_add(r.pool_surplus)
            .unwrap()
            .checked_add(ctx.mint.work_lottery)
            .unwrap()
            .checked_sub(prizes)
            .unwrap()
    );
    assert_eq!(
        net.state.chain().refund_budget,
        cap.checked_div(30).unwrap()
    );
    net.i0();
}

#[test]
fn heavy_users_cover_half_of_the_excess_with_work() {
    let (mut net, alice) = in_epoch_one();
    let bob = key(2).address();
    // Pretend alice already sent 1,000 transactions in this epoch.
    net.state
        .update_activity(&alice.address(), |a| {
            a.tx_count = 1_000;
            Ok(())
        })
        .unwrap();
    // The 1,001st without work: refused.
    let paid = net.transfer(&alice, &bob, toy(1));
    assert_eq!(
        tx_error(net.try_txs(vec![paid])),
        TxError::HeavyUserNeedsWork
    );
    // With a ticket: fine, and free of charge.
    net.easy_work();
    let mut f = net.fields(&alice);
    f.max_fee = Amount::ZERO;
    f.work_nonce = Some(1);
    let worked = net.sign(
        &alice,
        &f,
        &Action::Transfer {
            to: bob,
            amount: toy(1),
        },
    );
    let r = net.txs(vec![worked]);
    assert_eq!((r.counted_tx, r.fees), (1, Amount::ZERO));
    // 1,002nd paid: excess 2, covered 1: half is covered, fine.
    net.txs(vec![net.transfer(&alice, &bob, toy(1))]);
    // 1,003rd paid: excess 3, covered 1: refused.
    let paid = net.transfer(&alice, &bob, toy(1));
    assert_eq!(
        tx_error(net.try_txs(vec![paid])),
        TxError::HeavyUserNeedsWork
    );
}
