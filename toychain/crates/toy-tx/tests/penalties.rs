//! Protection rules of level A (spec 8): every rule with a case where it
//! applies and a case where it does not. Penalties remove rights, never
//! balances: a penalized account can always transfer against a fee.

#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use common::*;
use toy_crypto::bls::BlsSecretKey;
use toy_crypto::secp256k1::SecretKey;
use toy_emission::Phase;
use toy_state::{Right, Violation, inactivity_violated, mass_creation_violated};
use toy_tx::evidence::{DoubleSignEvidence, Vote, VotePhase};
use toy_tx::ticket::sign_ticket;
use toy_tx::{Action, Block, TxError};
use toy_types::params::VALIDATOR_MIN_STAKE;
use toy_types::{Address, Amount, Hash};

fn bls(n: u8) -> BlsSecretKey {
    BlsSecretKey::from_bytes(&[n; 32]).unwrap()
}

/// A network with a registered validator `val` (BLS key `bls(1)`) and a
/// funded reporter.
fn with_validator() -> (Net, SecretKey, SecretKey) {
    let mut net = Net::new(Phase::Normal);
    let val = key(1);
    let reporter = key(2);
    net.fund(&val, toy(120));
    net.fund(&reporter, toy(10));
    net.txs(vec![net.sign(
        &val,
        &net.fields(&val),
        &Action::Bond(VALIDATOR_MIN_STAKE),
    )]);
    let key = bls(1);
    let register = Action::RegisterValidator {
        bls_key: key.public_key().to_bytes(),
        proof: key.prove_possession().to_bytes(),
    };
    net.txs(vec![net.sign(&val, &net.fields(&val), &register)]);
    assert!(net.state.can_sign(&val.address()));
    (net, val, reporter)
}

fn vote(height: u64, round: u32, phase: VotePhase, block: u8) -> Vote {
    Vote {
        height,
        round,
        phase,
        block_hash: Hash::from_bytes([block; 32]),
    }
}

fn evidence(offender: Address, signer: &BlsSecretKey, a: Vote, b: Vote) -> DoubleSignEvidence {
    DoubleSignEvidence {
        offender,
        first: a.sign(signer, CHAIN),
        second: b.sign(signer, CHAIN),
    }
}

fn report(net: &Net, reporter: &SecretKey, ev: DoubleSignEvidence) -> Vec<u8> {
    net.sign(
        reporter,
        &net.fields(reporter),
        &Action::ReportDoubleSign(ev),
    )
}

/// A paid transfer from `who` must always work, whatever penalty runs.
fn paid_transfer_works(net: &mut Net, who: &SecretKey) {
    let to = key(250).address();
    let r = net.txs(vec![net.transfer(who, &to, toy(1))]);
    assert_eq!(r.counted_tx, 1);
}

#[test]
fn double_signature_applies() {
    let (mut net, val, reporter) = with_validator();
    let free = net.free(&val.address());
    let ev = evidence(
        val.address(),
        &bls(1),
        vote(10, 0, VotePhase::Precommit, 1),
        vote(10, 0, VotePhase::Precommit, 2),
    );
    let r = net.txs(vec![report(&net, &reporter, ev)]);
    assert_eq!(r.penalties.len(), 1);
    let (who, outcome) = r.penalties[0];
    assert_eq!(who, val.address());
    assert_eq!(outcome.violation, Violation::DoubleSign);
    assert_eq!(outcome.epochs, Some(7));
    assert!(!net.state.can_sign(&val.address()));
    let account = net.state.account(&val.address()).unwrap();
    // Free balance untouched; the bonded stake is held, still owned.
    assert_eq!(account.free(), free);
    assert_eq!(account.stake(), Amount::ZERO);
    assert_eq!(account.held_total().unwrap(), VALIDATOR_MIN_STAKE);
    paid_transfer_works(&mut net, &val);
    // The same offence cannot be punished twice.
    let again = evidence(
        val.address(),
        &bls(1),
        vote(10, 0, VotePhase::Precommit, 3),
        vote(10, 0, VotePhase::Precommit, 4),
    );
    let err = tx_error(net.try_txs(vec![report(&net, &reporter, again)]));
    assert_eq!(err, TxError::EvidenceAlreadyReported);
    net.i0();
}

#[test]
fn double_signature_does_not_apply() {
    let (mut net, val, reporter) = with_validator();
    let not_conflicting = [
        // A new round after a timeout is allowed (decision 106).
        (
            vote(10, 0, VotePhase::Prevote, 1),
            vote(10, 1, VotePhase::Prevote, 2),
        ),
        // Prevote and precommit of the same round are two phases.
        (
            vote(10, 0, VotePhase::Prevote, 1),
            vote(10, 0, VotePhase::Precommit, 2),
        ),
        // The same vote twice is no conflict.
        (
            vote(10, 0, VotePhase::Prevote, 1),
            vote(10, 0, VotePhase::Prevote, 1),
        ),
        // Different heights.
        (
            vote(10, 0, VotePhase::Prevote, 1),
            vote(11, 0, VotePhase::Prevote, 2),
        ),
    ];
    for (a, b) in not_conflicting {
        let ev = evidence(val.address(), &bls(1), a, b);
        let err = tx_error(net.try_txs(vec![report(&net, &reporter, ev)]));
        assert_eq!(err, TxError::InvalidEvidence, "{a:?} / {b:?}");
    }
    // Votes signed by another key than the registered one.
    let ev = evidence(
        val.address(),
        &bls(9),
        vote(10, 0, VotePhase::Prevote, 1),
        vote(10, 0, VotePhase::Prevote, 2),
    );
    assert_eq!(
        tx_error(net.try_txs(vec![report(&net, &reporter, ev)])),
        TxError::InvalidEvidence
    );
    // An address without validator key cannot be accused.
    let ev = evidence(
        reporter.address(),
        &bls(1),
        vote(10, 0, VotePhase::Prevote, 1),
        vote(10, 0, VotePhase::Prevote, 2),
    );
    assert_eq!(
        tx_error(net.try_txs(vec![report(&net, &reporter, ev)])),
        TxError::InvalidEvidence
    );
    assert!(net.state.can_sign(&val.address()));
    assert!(net.state.penalties(&val.address()).unwrap().strikes() == 0);
}

#[test]
fn registration_needs_a_valid_proof_of_possession() {
    let mut net = Net::new(Phase::Normal);
    let val = key(1);
    net.fund(&val, toy(120));
    net.txs(vec![net.sign(
        &val,
        &net.fields(&val),
        &Action::Bond(VALIDATOR_MIN_STAKE),
    )]);
    let register = Action::RegisterValidator {
        bls_key: bls(1).public_key().to_bytes(),
        // Proof made with another key.
        proof: bls(2).prove_possession().to_bytes(),
    };
    let err = tx_error(net.try_txs(vec![net.sign(&val, &net.fields(&val), &register)]));
    assert!(matches!(err, TxError::Crypto(_)), "{err:?}");
}

#[test]
fn invalid_proposal_rule() {
    // The evidence format (a signed invalid block) follows with the block
    // format in M4; the penalty itself is fixed now.
    let (mut net, val, _) = with_validator();
    assert!(net.state.can_propose(&val.address()));
    let out = net
        .state
        .apply_violation(&val.address(), Violation::InvalidProposal)
        .unwrap();
    assert_eq!(out.epochs, Some(7));
    // Applies: no more proposals. Does not apply: signing stays allowed.
    assert!(!net.state.can_propose(&val.address()));
    assert!(net.state.can_sign(&val.address()));
    paid_transfer_works(&mut net, &val);
}

#[test]
fn invalid_ticket_rule() {
    let mut net = Net::new(Phase::Normal);
    let honest = key(40);
    let cheat = key(41);
    net.easy_work();
    let epoch = net.next_ctx().epoch;
    // Does not apply: valid work.
    let r = net.block(Block {
        transactions: vec![],
        tickets: vec![sign_ticket(&honest, CHAIN, epoch, 1)],
    });
    assert!(r.penalties.is_empty());
    // Applies: work above the threshold.
    net.state.chain_mut().ticket_threshold = 0;
    let r = net.block(Block {
        transactions: vec![],
        tickets: vec![sign_ticket(&cheat, CHAIN, epoch, 1)],
    });
    assert_eq!(r.penalties[0].1.violation, Violation::InvalidTicket);
    assert!(net.state.is_locked(&cheat.address(), Right::Tickets));
    assert!(!net.state.is_locked(&honest.address(), Right::Tickets));
}

#[test]
fn mass_account_creation_rule() {
    let mut net = Net::new(Phase::Normal);
    let alice = key(1);
    net.fund(&alice, toy(100));
    net.fund(&key(250), toy(1));
    let deposit = net.state.activation_deposit();
    let new_address = |i: u8| {
        let mut b = [0x55u8; 20];
        b[19] = i;
        Address::from_bytes(b)
    };
    // 20 new accounts: does not apply.
    for i in 0..20u8 {
        net.txs(vec![net.transfer(&alice, &new_address(i), deposit)]);
    }
    assert!(!mass_creation_violated(20));
    assert!(net.state.penalties(&alice.address()).unwrap().strikes() == 0);
    // Transfers to existing accounts never count.
    for i in 0..5u8 {
        net.txs(vec![net.transfer(&alice, &new_address(i), toy(1))]);
    }
    assert!(!net.state.is_locked(&alice.address(), Right::FreeQuota));
    // The 21st new account: applies once, quota locked for 7 epochs.
    let r = net.txs(vec![net.transfer(&alice, &new_address(20), deposit)]);
    assert_eq!(r.penalties.len(), 1);
    assert_eq!(r.penalties[0].1.violation, Violation::MassAccountCreation);
    assert_eq!(r.penalties[0].1.epochs, Some(7));
    assert!(net.state.is_locked(&alice.address(), Right::FreeQuota));
    // A 22nd in the same epoch is not punished again.
    let r = net.txs(vec![net.transfer(&alice, &new_address(21), deposit)]);
    assert!(r.penalties.is_empty());
    paid_transfer_works(&mut net, &alice);
    // Counting starts afresh in the next epoch.
    net.to_next_epoch();
    let r = net.txs(vec![net.transfer(&alice, &new_address(100), deposit)]);
    assert!(r.penalties.is_empty());
    net.i0();
}

#[test]
fn inactivity_rule() {
    assert!(!inactivity_violated(2), "two missed epochs: does not apply");
    assert!(inactivity_violated(3), "three missed epochs: applies");
    let (mut net, val, _) = with_validator();
    let out = net
        .state
        .apply_violation(&val.address(), Violation::Inactivity)
        .unwrap();
    assert_eq!(out.epochs, None, "a pause, no period");
    assert!(!net.state.can_sign(&val.address()));
    paid_transfer_works(&mut net, &val);
    // The node reports back by registering its key again.
    let k = bls(1);
    let register = Action::RegisterValidator {
        bls_key: k.public_key().to_bytes(),
        proof: k.prove_possession().to_bytes(),
    };
    net.txs(vec![net.sign(&val, &net.fields(&val), &register)]);
    assert!(net.state.can_sign(&val.address()));
}

#[test]
fn escalation_doubles_and_resets_after_30_clean_epochs() {
    let mut net = Net::new(Phase::Normal);
    let cheat = key(42);
    let mut nonce = 0;
    let mut cheat_once = |net: &mut Net| {
        net.state.chain_mut().ticket_threshold = 0;
        let epoch = net.next_ctx().epoch;
        nonce += 1;
        let r = net.block(Block {
            transactions: vec![],
            tickets: vec![sign_ticket(&cheat, CHAIN, epoch, nonce)],
        });
        r.penalties[0].1
    };
    assert_eq!(cheat_once(&mut net).epochs, Some(1));
    // Wait until the lock is over, then cheat again: twice as long.
    net.to_next_epoch();
    net.to_next_epoch();
    let second = cheat_once(&mut net);
    assert_eq!((second.epochs, second.strike), (Some(2), 2));
    // 30 epochs without violation: counting starts afresh.
    for _ in 0..30 {
        net.to_next_epoch();
    }
    let fresh = cheat_once(&mut net);
    assert_eq!((fresh.epochs, fresh.strike), (Some(1), 1));
}

#[test]
fn no_penalty_ever_takes_a_balance() {
    let (mut net, val, _) = with_validator();
    for v in Violation::ALL {
        let free = net.free(&val.address());
        let deposit = net.state.account(&val.address()).unwrap().deposit();
        net.state.apply_violation(&val.address(), v).unwrap();
        let account = net.state.account(&val.address()).unwrap();
        assert_eq!(account.free(), free, "{v:?}");
        assert_eq!(account.deposit(), deposit, "{v:?}");
        paid_transfer_works(&mut net, &val);
        net.i0();
    }
}
