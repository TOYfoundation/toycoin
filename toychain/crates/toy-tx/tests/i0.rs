//! Rule I0: the sum of all pots equals the sum of all emissions after
//! every block (spec 9). A block that breaks it is invalid.

#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use common::*;
use toy_emission::Phase;
use toy_state::{ConsensusMint, Pot, StateError};
use toy_tx::ticket::sign_ticket;
use toy_tx::{Action, Block, BlockError};
use toy_types::{Address, Amount};

#[test]
fn genesis_and_empty_blocks_keep_i0() {
    let mut net = Net::new(Phase::Bootstrap);
    net.i0();
    assert_eq!(net.minted(), Amount::ZERO);
    for _ in 0..50 {
        net.empty(1);
        net.i0();
    }
    assert!(net.minted() > Amount::ZERO);
}

/// Two epochs of random activity: transfers (also to new addresses),
/// free transactions, donations, stake, closing, valid and invalid
/// tickets. After every block the pots must add up to the minted sum,
/// and the refund pool may only shrink by the surplus moved to the
/// lottery at an epoch start.
#[test]
fn i0_holds_after_every_block_with_mixed_activity() {
    let mut net = Net::new(Phase::Bootstrap);
    let users: Vec<_> = (1..=8).map(key).collect();
    net.fund(&users[0], toy(20_000));
    for u in &users[1..] {
        let tx = net.transfer(&users[0], &u.address(), toy(1_000));
        net.txs(vec![tx]);
    }
    let mut rng = Rng(42);
    let mut ticket_nonce = 0u64;
    let start_epoch = net.state.epoch();
    let mut checked_blocks = 0;
    let (mut free_tx, mut paid_tx, mut penalties, mut prizes, mut surplus) = (0, 0, 0, 0, 0);
    while net.state.epoch() < start_epoch + 2 {
        let mut candidates = Vec::new();
        for _ in 0..rng.below(4) {
            let from = &users[rng.below(8) as usize];
            let Some(account) = net.state.account(&from.address()) else {
                continue;
            };
            let free = account.free();
            let mut fields = net.fields(from);
            let action = match rng.below(10) {
                0..=3 => {
                    let to = if rng.below(3) == 0 {
                        let mut b = [0u8; 20];
                        b[..8].copy_from_slice(&rng.next().to_be_bytes());
                        Address::from_bytes(b)
                    } else {
                        users[rng.below(8) as usize].address()
                    };
                    let amount =
                        Amount::from_units(u128::from(rng.next()) % (free.units() / 4 + 1));
                    Action::Transfer {
                        to,
                        amount: amount.max(toy(1)),
                    }
                }
                4 => {
                    fields.max_fee = Amount::ZERO;
                    Action::Transfer {
                        to: users[rng.below(8) as usize].address(),
                        amount: Amount::from_units(1_000),
                    }
                }
                5 => Action::Donate(Amount::from_units(u128::from(rng.below(1_000_000)))),
                6 => Action::Bond(toy(rng.below(5))),
                7 => Action::Unbond(toy(rng.below(3))),
                8 => Action::CloseAccount {
                    target: Some(users[0].address()),
                    donation: Amount::from_units(u128::from(rng.below(1_000))),
                },
                _ => Action::Transfer {
                    to: users[0].address(),
                    amount: Amount::from_units(1),
                },
            };
            candidates.push(net.sign(from, &fields, &action));
        }
        let mut tickets = Vec::new();
        // Tickets cost an Argon2id hash per check, so only now and then.
        if rng.below(20) == 0 {
            if rng.below(2) == 0 {
                net.easy_work();
            }
            let epoch = net.next_ctx().epoch;
            ticket_nonce += 1;
            tickets.push(sign_ticket(
                &users[rng.below(8) as usize],
                CHAIN,
                epoch,
                ticket_nonce,
            ));
        }
        let pool_before = net.state.refund_pool();
        let (receipt, _) = net.greedy(candidates, tickets);
        net.i0();
        let pool_after = net.state.refund_pool();
        if pool_after < pool_before {
            assert!(receipt.new_epoch);
            assert!(pool_before.checked_sub(pool_after).unwrap() <= receipt.pool_surplus);
        }
        checked_blocks += 1;
        free_tx += receipt.free_tx;
        paid_tx += receipt.counted_tx;
        penalties += receipt.penalties.len();
        prizes += receipt.prizes.len();
        surplus += usize::from(!receipt.pool_surplus.is_zero());
    }
    // The random walk really reached every path.
    assert!(
        free_tx > 0 && paid_tx > 0 && penalties > 0 && prizes > 0 && surplus > 0,
        "free {free_tx}, paid {paid_tx}, penalties {penalties}, prizes {prizes}, surplus {surplus}"
    );
    assert!(checked_blocks > 1_440);
    // Every pot that M3 fills was used, the later ones stay zero.
    let totals = net.state.pot_totals().unwrap();
    for pot in [
        Pot::Free,
        Pot::ActivationDeposit,
        Pot::RefundPool,
        Pot::WorkLottery,
    ] {
        assert!(totals.get(pot) > Amount::ZERO, "{pot:?}");
    }
    for pot in [
        Pot::DelegatedStake,
        Pot::VaultWithdrawals,
        Pot::HashTimeLocks,
        Pot::Escrow,
        Pot::DormantSnapshots,
    ] {
        assert_eq!(totals.get(pot), Amount::ZERO, "{pot:?}");
    }
}

#[test]
fn block_that_mints_more_than_the_emission_is_rejected() {
    let mut net = Net::new(Phase::Normal);
    net.empty(3);
    let before = net.state.clone();
    // The proposer claims one unit more for the refund pool than the
    // emission engine computed. Parts and total still agree, so only rule
    // I0 can catch it.
    let result = net.try_block_with(&Block::default(), |ctx| {
        ctx.mint.refund_pool = ctx
            .mint
            .refund_pool
            .checked_add(Amount::from_units(1))
            .unwrap();
        ctx.mint.total = ctx.mint.total.checked_add(Amount::from_units(1)).unwrap();
    });
    assert!(
        matches!(result, Err(BlockError::I0(StateError::I0Violated { .. }))),
        "{result:?}"
    );
    assert_eq!(net.state, before, "a rejected block changes nothing");
    // The honest block is fine.
    net.empty(1);
    net.i0();
}

#[test]
fn block_that_mints_less_or_inconsistent_is_rejected() {
    let mut net = Net::new(Phase::Normal);
    net.empty(2);
    let less = net.try_block_with(&Block::default(), |ctx| {
        ctx.mint.work_lottery = ctx
            .mint
            .work_lottery
            .checked_sub(Amount::from_units(1))
            .unwrap();
        ctx.mint.total = ctx.mint.total.checked_sub(Amount::from_units(1)).unwrap();
    });
    assert!(matches!(less, Err(BlockError::I0(_))), "{less:?}");
    let inconsistent = net.try_block_with(&Block::default(), |ctx| {
        ctx.mint.consensus = ConsensusMint::Stakers(Amount::ZERO);
    });
    assert!(
        matches!(
            inconsistent,
            Err(BlockError::State(StateError::PartsMismatch))
        ),
        "{inconsistent:?}"
    );
    let wrong_total = net.try_block_with(&Block::default(), |ctx| {
        ctx.minted_total = ctx.minted_total.checked_add(Amount::from_units(1)).unwrap();
    });
    assert!(
        matches!(wrong_total, Err(BlockError::I0(_))),
        "{wrong_total:?}"
    );
}

#[test]
fn bootstrap_rewards_are_bonded_and_counted() {
    let mut net = Net::new(Phase::Bootstrap);
    net.empty(10);
    net.i0();
    let validators = net.state.start_validators().to_vec();
    assert_eq!(validators.len(), 21);
    let first = net.state.account(&validators[0]).unwrap();
    assert_eq!(first.free(), Amount::ZERO, "reward is bonded, not free");
    assert!(first.reward_locks_total().unwrap() > Amount::ZERO);
    // All start validators receive exactly the same.
    for v in &validators {
        let a = net.state.account(v).unwrap();
        assert_eq!(
            a.reward_locks_total().unwrap(),
            first.reward_locks_total().unwrap()
        );
    }
    assert!(net.state.pot_total(Pot::StartValidatorRewards).unwrap() > Amount::ZERO);
}
