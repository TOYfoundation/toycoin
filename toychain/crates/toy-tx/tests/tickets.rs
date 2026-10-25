//! ToyWork tickets and the base pot of the work lottery (spec 9.3,
//! decisions 106, 107, 119, 148 to 150).

#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use common::*;
use toy_crypto::eip155::ChainId;
use toy_emission::Phase;
use toy_state::{Right, Violation};
use toy_tx::lottery::{draw_base, next_mix};
use toy_tx::ticket::{TicketClaim, decode_ticket, sign_ticket};
use toy_tx::{Action, Block, BlockError, TxError};
use toy_types::params::{LOTTERY_MIN_PRIZE, TOYWORK_START_THRESHOLD};
use toy_types::{Amount, Hash};

fn tickets(list: Vec<Vec<u8>>) -> Block {
    Block {
        transactions: vec![],
        tickets: list,
    }
}

#[test]
fn a_device_without_account_wins_its_first_toy() {
    let mut net = Net::new(Phase::Normal);
    net.empty(3);
    net.easy_work();
    let device = key(30);
    assert!(net.state.account(&device.address()).is_none());
    let epoch = net.next_ctx().epoch;
    let committed = net.state.height() + 1;
    let r = net.block(tickets(vec![sign_ticket(&device, CHAIN, epoch, 7)]));
    assert_eq!(r.valid_tickets, 1);
    assert!(r.prizes.is_empty());
    // Block N + 1: no draw yet.
    let r = net.block(Block::default());
    assert!(r.prizes.is_empty());
    // Block N + 2: drawn with that block's randomness mix over the pot.
    let ctx = net.next_ctx();
    let pot = net
        .state
        .work_lottery()
        .checked_add(ctx.mint.work_base)
        .unwrap();
    let mix = next_mix(&net.state.chain().mix, &ctx.randomness);
    let id = TicketClaim {
        owner: device.address(),
        epoch,
        nonce: 7,
    }
    .id();
    let expected = draw_base(&[(id, device.address())], pot, &mix, committed, |_| false).unwrap();
    let r = net.block(Block::default());
    assert_eq!(r.prizes, expected);
    // A single ticket wins the whole pot; a new address needs 0.2 TOY.
    assert_eq!(r.prizes, vec![(device.address(), pot)]);
    assert!(r.prizes[0].1 >= LOTTERY_MIN_PRIZE);
    let account = net.state.account(&device.address()).unwrap();
    assert_eq!(account.deposit(), net.state.activation_deposit());
    assert_eq!(
        account.free(),
        r.prizes[0].1.checked_sub(account.deposit()).unwrap()
    );
    net.i0();
}

#[test]
fn pot_carries_over_when_nobody_plays() {
    let mut net = Net::new(Phase::Normal);
    let mut last = Amount::ZERO;
    for _ in 0..5 {
        net.empty(1);
        assert!(net.state.work_lottery() > last);
        last = net.state.work_lottery();
    }
}

#[test]
fn invalid_ticket_locks_the_tickets_of_its_owner_for_one_epoch() {
    let mut net = Net::new(Phase::Normal);
    net.empty(2);
    let device = key(31);
    // Nothing is below threshold 0: the work check fails.
    net.state.chain_mut().ticket_threshold = 0;
    let epoch = net.next_ctx().epoch;
    let r = net.block(tickets(vec![sign_ticket(&device, CHAIN, epoch, 1)]));
    assert_eq!(r.valid_tickets, 0);
    assert_eq!(r.penalties.len(), 1);
    let (who, outcome) = r.penalties[0];
    assert_eq!(who, device.address());
    assert_eq!(outcome.violation, Violation::InvalidTicket);
    assert_eq!(outcome.epochs, Some(1));
    assert!(net.state.is_locked(&device.address(), Right::Tickets));
    // Further tickets of this address are refused, also valid ones.
    net.easy_work();
    let again = tickets(vec![sign_ticket(&device, CHAIN, epoch, 2)]);
    assert_eq!(ticket_error(net.try_block(&again)), TxError::TicketsLocked);
    // Another device is not affected.
    net.block(tickets(vec![sign_ticket(&key(32), CHAIN, epoch, 1)]));
    // Rest of this epoch and the next full epoch: still locked.
    net.to_next_epoch();
    net.easy_work();
    let e1 = net.state.epoch();
    let in_e1 = tickets(vec![sign_ticket(&device, CHAIN, e1, 3)]);
    assert_eq!(ticket_error(net.try_block(&in_e1)), TxError::TicketsLocked);
    net.to_next_epoch();
    net.easy_work();
    let e2 = net.state.epoch();
    let r = net.block(tickets(vec![sign_ticket(&device, CHAIN, e2, 4)]));
    assert_eq!(r.valid_tickets, 1);
    net.i0();
}

#[test]
fn a_valid_ticket_is_never_penalized() {
    let mut net = Net::new(Phase::Normal);
    net.easy_work();
    let epoch = net.next_ctx().epoch;
    let r = net.block(tickets(vec![sign_ticket(&key(33), CHAIN, epoch, 1)]));
    assert!(r.penalties.is_empty());
    assert!(net.state.penalties(&key(33).address()).is_none());
}

#[test]
fn reused_tickets_are_refused() {
    let mut net = Net::new(Phase::Normal);
    net.easy_work();
    let epoch = net.next_ctx().epoch;
    let t = sign_ticket(&key(34), CHAIN, epoch, 5);
    let twice = tickets(vec![t.clone(), t.clone()]);
    assert_eq!(ticket_error(net.try_block(&twice)), TxError::TicketReused);
    net.block(tickets(vec![t.clone()]));
    assert_eq!(
        ticket_error(net.try_block(&tickets(vec![t]))),
        TxError::TicketReused
    );
    // Replaying is no violation of the owner (anyone can replay).
    assert!(net.state.penalties(&key(34).address()).is_none());
}

#[test]
fn tickets_for_another_epoch_or_chain_are_refused_without_penalty() {
    let mut net = Net::new(Phase::Normal);
    net.state.chain_mut().ticket_threshold = 0;
    let epoch = net.next_ctx().epoch;
    let wrong_epoch = tickets(vec![sign_ticket(&key(35), CHAIN, epoch + 1, 1)]);
    assert_eq!(
        ticket_error(net.try_block(&wrong_epoch)),
        TxError::TicketWrongEpoch
    );
    let other_chain = tickets(vec![sign_ticket(
        &key(35),
        ChainId::TOYCHAIN_TESTNET,
        epoch,
        1,
    )]);
    assert_eq!(
        ticket_error(net.try_block(&other_chain)),
        TxError::WrongChainId
    );
    assert!(net.state.penalties(&key(35).address()).is_none());
}

#[test]
fn at_most_32_tickets_per_block_including_attached_ones() {
    let mut net = Net::new(Phase::Normal);
    let alice = key(1);
    net.fund(&alice, toy(10));
    net.easy_work();
    let epoch = net.next_ctx().epoch;
    let many: Vec<_> = (0..33)
        .map(|i| sign_ticket(&key(40 + i), CHAIN, epoch, 1))
        .collect();
    assert_eq!(
        net.try_block(&tickets(many.clone())),
        Err(BlockError::TooManyTickets { limit: 32 })
    );
    // 31 standalone plus one attached to a transaction: 32, fine.
    let mut f = net.fields(&alice);
    f.max_fee = Amount::ZERO;
    f.work_nonce = Some(99);
    let worked = net.sign(&alice, &f, &Action::Donate(Amount::from_units(1)));
    let block = Block {
        transactions: vec![worked.clone()],
        tickets: many[..32].to_vec(),
    };
    assert_eq!(
        net.try_block(&block),
        Err(BlockError::TooManyTickets { limit: 32 })
    );
    let block = Block {
        transactions: vec![worked],
        tickets: many[..31].to_vec(),
    };
    let r = net.block(block);
    assert_eq!(r.valid_tickets, 32);
}

#[test]
fn faster_blocks_carry_proportionally_fewer_tickets() {
    let mut net = Net::new(Phase::Normal);
    net.easy_work();
    let epoch = net.next_ctx().epoch;
    let nine: Vec<_> = (0..9)
        .map(|i| sign_ticket(&key(40 + i), CHAIN, epoch, 1))
        .collect();
    // 15 seconds after the previous block: 32 × 15 ÷ 60 = 8.
    let r = net.try_block_with(&tickets(nine.clone()), |ctx| ctx.gap_seconds = 15);
    assert_eq!(r, Err(BlockError::TooManyTickets { limit: 8 }));
    // 30 seconds: 16.
    let r = net
        .try_block_with(&tickets(nine), |ctx| ctx.gap_seconds = 30)
        .unwrap();
    assert_eq!(r.valid_tickets, 9);
}

#[test]
fn many_tickets_share_the_base_pot_by_luck_levels() {
    let mut net = Net::new(Phase::Normal);
    net.empty(50);
    net.easy_work();
    let epoch = net.next_ctx().epoch;
    let devices: Vec<_> = (60..76).map(key).collect();
    let list: Vec<_> = devices
        .iter()
        .map(|d| sign_ticket(d, CHAIN, epoch, 1))
        .collect();
    net.block(tickets(list));
    net.empty(1);
    let pot = net.state.work_lottery();
    let ctx = net.next_ctx();
    let r = net.block(Block::default());
    // Half to all 16 tickets win; prizes differ by luck level.
    assert!((8..=16).contains(&r.prizes.len()), "{}", r.prizes.len());
    let paid = Amount::checked_sum(r.prizes.iter().map(|(_, p)| *p)).unwrap();
    let available = pot.checked_add(ctx.mint.work_base).unwrap();
    assert!(paid <= available);
    assert!(paid > available.checked_sub(Amount::from_units(100)).unwrap());
    let mut sizes: Vec<Amount> = r.prizes.iter().map(|(_, p)| *p).collect();
    sizes.sort();
    sizes.dedup();
    assert!(sizes.len() > 1, "luck levels give different prizes");
    for (winner, _) in &r.prizes {
        assert!(devices.iter().any(|d| d.address() == *winner));
    }
    net.i0();
}

#[test]
fn a_ticket_as_fee_substitute_makes_the_transaction_free() {
    let mut net = Net::new(Phase::Normal);
    let alice = key(1);
    net.fund(&alice, toy(10));
    net.fund(&key(2), toy(1));
    net.easy_work();
    let mut f = net.fields(&alice);
    f.max_fee = Amount::ZERO;
    f.work_nonce = Some(3);
    let tx = net.sign(
        &alice,
        &f,
        &Action::Transfer {
            to: key(2).address(),
            amount: toy(1),
        },
    );
    let before = net.free(&alice.address());
    let r = net.txs(vec![tx]);
    assert_eq!((r.counted_tx, r.free_tx, r.fees), (1, 0, Amount::ZERO));
    assert_eq!(
        net.free(&alice.address()),
        before.checked_sub(toy(1)).unwrap()
    );
    // The same ticket cannot be used again, neither attached nor alone.
    let mut f = net.fields(&alice);
    f.max_fee = Amount::ZERO;
    f.work_nonce = Some(3);
    let again = net.sign(&alice, &f, &Action::Donate(Amount::from_units(1)));
    assert_eq!(tx_error(net.try_txs(vec![again])), TxError::TicketReused);
    let epoch = net.next_ctx().epoch;
    let alone = tickets(vec![sign_ticket(&alice, CHAIN, epoch, 3)]);
    assert_eq!(ticket_error(net.try_block(&alone)), TxError::TicketReused);
    // Invalid work in an attached ticket: the block is invalid.
    net.state.chain_mut().ticket_threshold = 0;
    let mut f = net.fields(&alice);
    f.max_fee = Amount::ZERO;
    f.work_nonce = Some(4);
    let bad = net.sign(&alice, &f, &Action::Donate(Amount::from_units(1)));
    assert_eq!(tx_error(net.try_txs(vec![bad])), TxError::TicketWorkInvalid);
    net.i0();
}

#[test]
fn tickets_are_bound_to_owner_epoch_seed_and_chain() {
    let base = TicketClaim {
        owner: key(1).address(),
        epoch: 5,
        nonce: 9,
    };
    let seed = Hash::from_bytes([1; 32]);
    let input = base.work_input(CHAIN, &seed);
    let other_owner = TicketClaim {
        owner: key(2).address(),
        ..base
    };
    let other_epoch = TicketClaim { epoch: 6, ..base };
    assert_ne!(other_owner.work_input(CHAIN, &seed), input);
    assert_ne!(other_epoch.work_input(CHAIN, &seed), input);
    assert_ne!(base.work_input(CHAIN, &Hash::from_bytes([2; 32])), input);
    assert_ne!(base.work_input(ChainId::TOYCHAIN_TESTNET, &seed), input);
    // The signature names the owner: a ticket cannot be reassigned.
    let raw = sign_ticket(&key(1), CHAIN, 5, 9);
    assert_eq!(decode_ticket(&raw, CHAIN).unwrap().claim, base);
    let mut forged = raw.clone();
    let last = forged.len() - 1;
    forged[last] ^= 1;
    if let Ok(t) = decode_ticket(&forged, CHAIN) {
        assert_ne!(t.claim.owner, base.owner);
    }
}

#[test]
fn difficulty_adapts_once_per_epoch() {
    let mut net = Net::new(Phase::Normal);
    assert_eq!(net.state.chain().ticket_threshold, TOYWORK_START_THRESHOLD);
    // No tickets at all in epoch 0 (target 16 per block): four times
    // easier in epoch 1.
    net.to_next_epoch();
    assert_eq!(
        net.state.chain().ticket_threshold,
        TOYWORK_START_THRESHOLD * 4
    );
    // The seed of epoch 1 is the randomness of the last block of epoch 0.
    let height = net.state.height() - 1;
    assert_eq!(
        net.state.chain().epoch_seed,
        toy_crypto::hash::blake3(&height.to_be_bytes())
    );
}

/// Prizes of block N + 2 for 16 tickets of block N, with optional changes
/// to the proposer of block N and the VRF output of block N + 2.
fn draw_after(other_proposer: bool, other_vrf: bool) -> Vec<(toy_types::Address, Amount)> {
    let mut net = Net::new(Phase::Normal);
    net.empty(20);
    net.easy_work();
    let epoch = net.next_ctx().epoch;
    let list: Vec<_> = (60..76)
        .map(|i| sign_ticket(&key(i), CHAIN, epoch, 1))
        .collect();
    net.try_block_with(&tickets(list), |ctx| {
        if other_proposer {
            ctx.proposer = key(251).address();
        }
    })
    .unwrap();
    net.empty(1);
    let r = net
        .try_block_with(&Block::default(), |ctx| {
            if other_vrf {
                ctx.randomness = Hash::from_bytes([0xee; 32]);
            }
        })
        .unwrap();
    r.prizes
}

/// Spec 9.3, decision 150: the draw for block N uses `R_(N+2)`. Whoever
/// builds block N cannot steer it, the VRF output of block N + 2 decides.
#[test]
fn the_draw_depends_on_the_randomness_two_blocks_later() {
    let normal = draw_after(false, false);
    assert!(!normal.is_empty());
    assert_eq!(draw_after(true, false), normal, "proposer of block N");
    assert_ne!(draw_after(false, true), normal, "VRF of block N + 2");
}
