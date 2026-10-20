//! Executing a block: all or nothing (spec 7, 8, 9).
//!
//! [`execute_block`] works on a copy of the state and returns the new
//! state only if every step succeeds and rule I0 holds at the end. A block
//! with a single invalid transaction or ticket is invalid as a whole; the
//! proposer must leave such items out.
//!
//! Order of the steps:
//!
//! 1. Start the block; at an epoch change: unbonded stake becomes free
//!    (state), free quota from the utilization of the previous epoch, the
//!    refund pool surplus goes to the work lottery, refund budget 1/30 of
//!    the pool, new ToyWork threshold, new ticket seed.
//! 2. Mint the block emission (from `toy-emission`).
//! 3. Check size (128 KB) and ticket count (16).
//! 4. Standalone tickets: valid ones join the lottery; a signed ticket
//!    whose work fails costs its owner the ticket right (penalty).
//! 5. Transactions in order: expiry, nonce, then free quota, ToyWork
//!    ticket or paid fee, heavy-user rule, then the action itself.
//! 6. Lottery draw for the tickets of block `height − 2`.
//! 7. Base fee and epoch counters, then rule I0 against the minted sum.

use std::collections::BTreeMap;

use toy_crypto::eip155::ChainId;
use toy_crypto::work::ToyWorkHasher;
use toy_state::{Mint, PenaltyOutcome, Right, State, Violation, mass_creation_violated};
use toy_types::params::{
    HEAVY_USER_THRESHOLD, LOTTERY_DRAW_DELAY_BLOCKS, MAX_BLOCK_BYTES, MAX_EXPIRY_BLOCKS,
    MAX_FREE_BYTES_PER_BLOCK, QUOTA_MIN_ACCOUNT_AGE_BLOCKS, TICKETS_MAX_PER_BLOCK,
};
use toy_types::{Address, Amount, ArithmeticError, EpochNumber, Hash, Height};

use crate::error::{BlockError, TxError};
use crate::fees::{
    charged_fee, next_base_fee, nominal_fee, quota, refund_budget, refund_pool_cap, required_fee,
    split_fee,
};
use crate::lottery::{draw, next_threshold};
use crate::ticket::{TicketClaim, decode_ticket};
use crate::transaction::{Action, Transaction, decode_transaction, verify_possession};

/// What the block executor needs to know besides the block contents.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockContext {
    /// Height of the block.
    pub height: Height,
    /// Epoch of the block (from the epoch clock, `toy-emission`).
    pub epoch: EpochNumber,
    /// Chain ID of this network.
    pub chain: ChainId,
    /// Proposer, receives 20 % of paid fees.
    pub proposer: Address,
    /// Randomness of the block (VRF output of the proposer, M4).
    pub randomness: Hash,
    /// Emission of this block (from `toy-emission`).
    pub mint: Mint,
    /// Sum of all emissions up to and including this block, computed
    /// independently by `toy-emission`. Rule I0 is checked against it.
    pub minted_total: Amount,
}

/// The contents of a block.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Block {
    /// Encoded transactions, in order.
    pub transactions: Vec<Vec<u8>>,
    /// Encoded standalone ToyWork tickets.
    pub tickets: Vec<Vec<u8>>,
}

/// Summary of an executed block.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BlockReceipt {
    /// Paid or work-backed transactions (count for the factor `m`).
    pub counted_tx: u64,
    /// Free-quota transactions (never count for `m`).
    pub free_tx: u64,
    /// Bytes of all transactions.
    pub tx_bytes: u64,
    /// Sum of all paid fees.
    pub fees: Amount,
    /// Sum of the nominal fees covered by the refund pool.
    pub refunded: Amount,
    /// Valid tickets (standalone and attached).
    pub valid_tickets: u64,
    /// Penalties imposed in this block.
    pub penalties: Vec<(Address, PenaltyOutcome)>,
    /// Lottery prizes paid in this block.
    pub prizes: Vec<(Address, Amount)>,
    /// Refund pool surplus moved to the work lottery (epoch start).
    pub pool_surplus: Amount,
    /// `true` if the block started a new epoch.
    pub new_epoch: bool,
    /// State root after the block.
    pub state_root: Hash,
}

/// Executes `block` on a copy of `state`.
///
/// # Errors
/// [`BlockError`] if anything in the block is invalid; the given state is
/// never changed.
pub fn execute_block(
    state: &State,
    ctx: &BlockContext,
    block: &Block,
    hasher: &mut ToyWorkHasher,
) -> Result<(State, BlockReceipt), BlockError> {
    let mut s = state.clone();
    let mut r = BlockReceipt {
        new_epoch: ctx.epoch != s.epoch(),
        ..BlockReceipt::default()
    };
    s.begin_block(ctx.height, ctx.epoch)?;
    if r.new_epoch {
        start_epoch(&mut s, &mut r)?;
    }
    s.mint_block(&ctx.mint)?;

    let mut total_bytes = 0u64;
    for raw in block.transactions.iter().chain(&block.tickets) {
        let len = u64::try_from(raw.len()).map_err(|_| BlockError::TooLarge)?;
        total_bytes = total_bytes.checked_add(len).ok_or(BlockError::TooLarge)?;
    }
    if total_bytes > MAX_BLOCK_BYTES {
        return Err(BlockError::TooLarge);
    }

    let txs = block
        .transactions
        .iter()
        .enumerate()
        .map(|(index, raw)| {
            decode_transaction(raw, ctx.chain)
                .map_err(|error| BlockError::Transaction { index, error })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let attached = txs.iter().filter(|t| t.work_nonce.is_some()).count();
    let ticket_count = block.tickets.len().saturating_add(attached);
    if u64::try_from(ticket_count).unwrap_or(u64::MAX) > TICKETS_MAX_PER_BLOCK {
        return Err(BlockError::TooManyTickets);
    }

    let mut owners = Vec::new();
    for (index, raw) in block.tickets.iter().enumerate() {
        apply_ticket(&mut s, ctx, raw, hasher, &mut owners, &mut r)
            .map_err(|error| BlockError::Ticket { index, error })?;
    }

    let mut exec = TxRun::default();
    for (index, tx) in txs.iter().enumerate() {
        apply_transaction(&mut s, ctx, tx, hasher, &mut exec, &mut r)
            .map_err(|error| BlockError::Transaction { index, error })?;
    }

    if !owners.is_empty() {
        s.chain_mut().pending_draws.insert(ctx.height, owners);
    }
    if let Some(committed) = ctx.height.checked_sub(LOTTERY_DRAW_DELAY_BLOCKS)
        && let Some(drawn) = s.chain_mut().pending_draws.remove(&committed)
    {
        let prizes = draw(&drawn, s.work_lottery(), &ctx.randomness, committed)?;
        s.pay_lottery_prizes(&prizes)?;
        r.prizes = prizes;
    }

    let chain = s.chain_mut();
    chain.base_fee_per_byte = next_base_fee(chain.base_fee_per_byte, r.tx_bytes)?;
    chain.epoch_tx_bytes = chain.epoch_tx_bytes.saturating_add(r.tx_bytes);
    chain.epoch_blocks = chain.epoch_blocks.saturating_add(1);
    chain.epoch_tickets = chain.epoch_tickets.saturating_add(r.valid_tickets);
    chain.last_randomness = ctx.randomness;

    s.check_i0(ctx.minted_total).map_err(BlockError::I0)?;
    r.state_root = s.root();
    Ok((s, r))
}

fn start_epoch(s: &mut State, r: &mut BlockReceipt) -> Result<(), BlockError> {
    let previous = s.chain().clone();
    let cap = refund_pool_cap(previous.base_fee_per_byte, previous.epoch_blocks)?;
    let surplus = s.refund_pool().checked_sub(cap).unwrap_or(Amount::ZERO);
    if !surplus.is_zero() {
        s.move_refund_surplus_to_lottery(surplus)?;
    }
    r.pool_surplus = surplus;
    let budget = refund_budget(s.refund_pool())?;
    let chain = s.chain_mut();
    chain.epoch_quota = quota(previous.epoch_tx_bytes, previous.epoch_blocks);
    chain.refund_budget = budget;
    chain.refund_used = Amount::ZERO;
    chain.ticket_threshold = next_threshold(
        previous.ticket_threshold,
        previous.epoch_tickets,
        previous.epoch_blocks,
    );
    chain.epoch_seed = previous.last_randomness;
    chain.epoch_tx_bytes = 0;
    chain.epoch_blocks = 0;
    chain.epoch_tickets = 0;
    chain.used_tickets.clear();
    Ok(())
}

/// Checks that `owner` may use the ticket and marks it used. Returns
/// whether its work is valid.
fn claim_ticket(
    s: &mut State,
    ctx: &BlockContext,
    claim: &TicketClaim,
    hasher: &mut ToyWorkHasher,
) -> Result<bool, TxError> {
    if claim.epoch != ctx.epoch {
        return Err(TxError::TicketWrongEpoch);
    }
    if s.is_locked(&claim.owner, Right::Tickets) {
        return Err(TxError::TicketsLocked);
    }
    let id = claim.id();
    let chain = s.chain_mut();
    if !chain.used_tickets.insert(id) {
        return Err(TxError::TicketReused);
    }
    let seed = chain.epoch_seed;
    let threshold = chain.ticket_threshold;
    claim.work_is_valid(hasher, ctx.chain, &seed, threshold)
}

fn apply_ticket(
    s: &mut State,
    ctx: &BlockContext,
    raw: &[u8],
    hasher: &mut ToyWorkHasher,
    owners: &mut Vec<Address>,
    r: &mut BlockReceipt,
) -> Result<(), TxError> {
    let ticket = decode_ticket(raw, ctx.chain)?;
    let claim = ticket.claim;
    if claim_ticket(s, ctx, &claim, hasher)? {
        owners.push(claim.owner);
        r.valid_tickets = r.valid_tickets.saturating_add(1);
    } else {
        // Signed by its owner, recomputable by everyone: a provable
        // violation (spec 8). Only the ticket right is removed.
        let outcome = s.apply_violation(&claim.owner, Violation::InvalidTicket)?;
        r.penalties.push((claim.owner, outcome));
    }
    Ok(())
}

/// Running data of the transactions of one block.
#[derive(Default)]
struct TxRun {
    per_sender: BTreeMap<Address, u64>,
    free_started: bool,
    free_bytes: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Payment {
    Paid,
    Work(u64),
    Free,
}

fn apply_transaction(
    s: &mut State,
    ctx: &BlockContext,
    tx: &Transaction,
    hasher: &mut ToyWorkHasher,
    run: &mut TxRun,
    r: &mut BlockReceipt,
) -> Result<(), TxError> {
    if let Some(expiry) = tx.expiry {
        if expiry < ctx.height {
            return Err(TxError::Expired {
                expiry,
                height: ctx.height,
            });
        }
        if expiry > ctx.height.saturating_add(MAX_EXPIRY_BLOCKS) {
            return Err(TxError::ExpiryTooFar);
        }
    }
    let sender = tx.sender;
    let created = s
        .account(&sender)
        .ok_or(toy_state::StateError::UnknownAccount(sender))?
        .created_height();
    s.release_excess_deposit(&sender)?;
    s.use_nonce(&sender, tx.nonce)?;
    let k = run.per_sender.entry(sender).or_insert(0);
    *k = k.checked_add(1).ok_or(ArithmeticError::Overflow)?;
    let k_in_block = *k;

    let payment = match tx.work_nonce {
        Some(n) => Payment::Work(n),
        None if tx.max_fee.is_zero() => Payment::Free,
        None => Payment::Paid,
    };
    if payment == Payment::Free {
        run.free_started = true;
    } else if run.free_started {
        return Err(TxError::PaidAfterFree);
    }

    let activity = s.update_activity(&sender, |a| {
        a.tx_count = a.tx_count.checked_add(1).ok_or(ArithmeticError::Overflow)?;
        Ok(())
    })?;
    let over = activity.tx_count.saturating_sub(HEAVY_USER_THRESHOLD);

    match payment {
        Payment::Work(nonce) => {
            let claim = TicketClaim {
                owner: sender,
                epoch: ctx.epoch,
                nonce,
            };
            if !claim_ticket(s, ctx, &claim, hasher)? {
                return Err(TxError::TicketWorkInvalid);
            }
            r.valid_tickets = r.valid_tickets.saturating_add(1);
            r.counted_tx = r.counted_tx.saturating_add(1);
            if over > 0 {
                s.update_activity(&sender, |a| {
                    a.work_backed_over = a
                        .work_backed_over
                        .checked_add(1)
                        .ok_or(ArithmeticError::Overflow)?;
                    Ok(())
                })?;
            }
        }
        Payment::Free => {
            if s.is_locked(&sender, Right::FreeQuota) {
                return Err(TxError::QuotaLocked);
            }
            if ctx.height.saturating_sub(created) < QUOTA_MIN_ACCOUNT_AGE_BLOCKS {
                return Err(TxError::AccountTooYoung);
            }
            let chain = s.chain();
            if activity.free_used >= chain.epoch_quota {
                return Err(TxError::QuotaExhausted);
            }
            let nominal = nominal_fee(chain.base_fee_per_byte, tx.size)?;
            let used = chain.refund_used.checked_add(nominal)?;
            if used > chain.refund_budget {
                return Err(TxError::RefundBudgetExhausted);
            }
            let free_bytes = run
                .free_bytes
                .checked_add(tx.size)
                .ok_or(ArithmeticError::Overflow)?;
            if free_bytes > MAX_FREE_BYTES_PER_BLOCK {
                return Err(TxError::FreeBytesExceeded);
            }
            run.free_bytes = free_bytes;
            s.chain_mut().refund_used = used;
            s.update_activity(&sender, |a| {
                a.free_used = a
                    .free_used
                    .checked_add(1)
                    .ok_or(ArithmeticError::Overflow)?;
                Ok(())
            })?;
            r.free_tx = r.free_tx.saturating_add(1);
            r.refunded = r.refunded.checked_add(nominal)?;
        }
        Payment::Paid => {
            let required = required_fee(s.chain().base_fee_per_byte, tx.size, k_in_block)?;
            let fee = charged_fee(tx.max_fee, tx.max_tip, required)?;
            s.pay_fee(&sender, &split_fee(fee)?, &ctx.proposer)?;
            r.fees = r.fees.checked_add(fee)?;
            r.counted_tx = r.counted_tx.saturating_add(1);
        }
    }

    if over > 0 {
        let backed = s
            .account(&sender)
            .map_or(0, |a| a.activity(ctx.epoch).work_backed_over);
        if backed.saturating_mul(2) < over {
            return Err(TxError::HeavyUserNeedsWork);
        }
    }

    r.tx_bytes = r.tx_bytes.saturating_add(tx.size);
    apply_action(s, ctx, &sender, &tx.action, r)
}

fn apply_action(
    s: &mut State,
    ctx: &BlockContext,
    sender: &Address,
    action: &Action,
    r: &mut BlockReceipt,
) -> Result<(), TxError> {
    match action {
        Action::Transfer { to, amount } => {
            let outcome = s.transfer(sender, to, *amount)?;
            if outcome.activated {
                let activity = s.update_activity(sender, |a| {
                    a.new_accounts = a
                        .new_accounts
                        .checked_add(1)
                        .ok_or(ArithmeticError::Overflow)?;
                    Ok(())
                })?;
                if mass_creation_violated(activity.new_accounts) && !activity.mass_creation_flagged
                {
                    s.update_activity(sender, |a| {
                        a.mass_creation_flagged = true;
                        Ok(())
                    })?;
                    let outcome = s.apply_violation(sender, Violation::MassAccountCreation)?;
                    r.penalties.push((*sender, outcome));
                }
            }
        }
        Action::Bond(amount) => s.bond(sender, *amount)?,
        Action::Unbond(amount) => s.unbond(sender, *amount)?,
        Action::Donate(amount) => s.donate(sender, *amount)?,
        Action::RegisterValidator { bls_key, proof } => {
            verify_possession(bls_key, proof)?;
            s.register_validator(sender, *bls_key)?;
        }
        Action::DeregisterValidator => s.deregister_validator(sender)?,
        Action::CloseAccount { target, donation } => {
            s.close_account(sender, target.as_ref(), *donation)?;
        }
        Action::ReportDoubleSign(evidence) => {
            let key = s
                .account(&evidence.offender)
                .and_then(|a| a.validator().map(|v| v.bls_key))
                .ok_or(TxError::InvalidEvidence)?;
            evidence.verify(ctx.chain, &key)?;
            if !s.chain_mut().reported_evidence.insert(evidence.id()) {
                return Err(TxError::EvidenceAlreadyReported);
            }
            let outcome = s.apply_violation(&evidence.offender, Violation::DoubleSign)?;
            r.penalties.push((evidence.offender, outcome));
        }
    }
    Ok(())
}
