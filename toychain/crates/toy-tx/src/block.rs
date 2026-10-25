//! Executing a block: all or nothing (spec 7, 8, 9).
//!
//! [`execute_block`] works on a copy of the state and returns the new
//! state only if every step succeeds and rule I0 holds at the end. A block
//! with a single invalid transaction or ticket is invalid as a whole; the
//! proposer must leave such items out.
//!
//! Order of the steps:
//!
//! 1. Start the block; at an epoch change: staking rewards are credited,
//!    time locks end (state), free quota from utilization and water level
//!    of the previous epoch, the refund pool surplus goes to the staking
//!    pot, refund budget 1/30 of the pool, new ToyWork threshold, new
//!    ticket seed, the tickets of the epoch become the "previous epoch"
//!    tickets of the jackpots Welle and Gezeiten.
//! 2. Mint the block emission (from `toy-emission`), advance the
//!    randomness mix `R_H = BLAKE3(R_(H−1) ‖ VRF output)`.
//! 3. Check size (128 KB) and ticket count (32 at 60 seconds gap, at least
//!    8).
//! 4. Standalone tickets: valid ones join the lottery; a signed ticket
//!    whose work fails costs its owner the ticket right (penalty).
//! 5. Transactions in order: expiry, nonce, staking reward credit, then
//!    free quota, ToyWork ticket or paid fee (heavy users pay twice the
//!    base fee), then the action itself.
//! 6. Base pot draw for the tickets of block `H − 2`, then the jackpots
//!    (chance, force, milestones), all with `R_H`.
//! 7. Base fee and epoch counters, then rule I0 against the minted sum.

use std::collections::BTreeMap;

use toy_crypto::eip155::ChainId;
use toy_crypto::hash::blake3_derive;
use toy_crypto::work::ToyWorkHasher;
use toy_state::{
    LotteryPot, Mint, PenaltyOutcome, Right, State, Violation, mass_creation_violated,
};
use toy_types::params::{
    EPOCH_MIN_BLOCKS, JACKPOT_FUNKE_WINDOW_BLOCKS, JACKPOT_MAX_SHARE_OF_YEAR,
    JACKPOT_MILESTONE_HEIGHT, JACKPOT_MILESTONE_SUPPLY, JACKPOT_MILESTONE_TIDE_DAYS,
    JACKPOT_YEAR_EPOCHS, LOTTERY_DRAW_DELAY_BLOCKS, MAX_BLOCK_BYTES, MAX_EXPIRY_BLOCKS,
    MAX_FREE_BYTES_PER_BLOCK,
};
use toy_types::{Address, Amount, ArithmeticError, EpochNumber, Hash, Height, Ppm};

use crate::error::{BlockError, TxError};
use crate::fees::{
    account_quota, base_quota, charged_fee, is_heavy_use, next_base_fee, nominal_fee, quota,
    refund_budget, refund_pool_cap, required_fee, split_fee, water_level,
};
use crate::lottery::{
    Jackpot, Trigger, TriggerInputs, draw_base, draw_jackpot, milestone_delay, next_mix,
    next_threshold, ticket_limit, trigger,
};
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
    /// Randomness of the block (VRF output of the proposer, M4). It enters
    /// the randomness mix.
    pub randomness: Hash,
    /// Emission of this block (from `toy-emission`).
    pub mint: Mint,
    /// Sum of all emissions up to and including this block, computed
    /// independently by `toy-emission`. Rule I0 is checked against it.
    pub minted_total: Amount,
    /// Seconds since the previous block (ticket limit, spec 9.3).
    pub gap_seconds: u64,
    /// Full plan days at this block (`toy-emission`, milestone low tide).
    pub plan_days: u64,
    /// Tide `W(t)` of the epoch from the K table (`toy-emission`).
    pub tide: Ppm,
    /// Signature participation of the previous epoch (consensus, M4).
    pub participation: Ppm,
}

/// A jackpot paid in a block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JackpotPayout {
    /// Which jackpot.
    pub jackpot: Jackpot,
    /// Why it fired.
    pub trigger: Trigger,
    /// Prizes per address.
    pub prizes: Vec<(Address, Amount)>,
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
    /// Prizes of the base pot paid in this block.
    pub prizes: Vec<(Address, Amount)>,
    /// Jackpots paid in this block.
    pub jackpots: Vec<JackpotPayout>,
    /// Refund pool surplus moved to the staking pot (epoch start).
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
    let mix = next_mix(&s.chain().mix, &ctx.randomness);
    {
        let chain = s.chain_mut();
        chain.mix = mix;
        chain.epoch_minted = chain.epoch_minted.checked_add(ctx.mint.total)?;
    }

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
    let limit = ticket_limit(ctx.gap_seconds);
    if u64::try_from(ticket_count).unwrap_or(u64::MAX) > limit {
        return Err(BlockError::TooManyTickets { limit });
    }

    let mut tickets = Vec::new();
    for (index, raw) in block.tickets.iter().enumerate() {
        apply_ticket(&mut s, ctx, raw, hasher, &mut tickets, &mut r)
            .map_err(|error| BlockError::Ticket { index, error })?;
    }

    let mut exec = TxRun::default();
    for (index, tx) in txs.iter().enumerate() {
        apply_transaction(&mut s, ctx, tx, hasher, &mut exec, &mut r)
            .map_err(|error| BlockError::Transaction { index, error })?;
    }

    record_tickets(&mut s, ctx.height, tickets);
    if let Some(committed) = ctx.height.checked_sub(LOTTERY_DRAW_DELAY_BLOCKS)
        && let Some(drawn) = s.chain_mut().pending_draws.remove(&committed)
    {
        let pot = s.lottery_pot(LotteryPot::Base);
        let prizes = draw_base(&drawn, pot, &mix, committed, |a| s.account(a).is_some())?;
        s.pay_lottery_prizes(LotteryPot::Base, &prizes)?;
        r.prizes = prizes;
    }
    run_jackpots(&mut s, ctx, &mix, &mut r)?;

    let chain = s.chain_mut();
    chain.base_fee_per_byte = next_base_fee(chain.base_fee_per_byte, r.tx_bytes)?;
    chain.epoch_tx_bytes = chain.epoch_tx_bytes.saturating_add(r.tx_bytes);
    chain.epoch_blocks = chain.epoch_blocks.saturating_add(1);
    chain.epoch_tickets = chain.epoch_tickets.saturating_add(r.valid_tickets);
    chain.last_randomness = ctx.randomness;
    chain.minted_total = ctx.minted_total;
    chain.plan_days = ctx.plan_days;

    s.check_i0(ctx.minted_total).map_err(BlockError::I0)?;
    r.state_root = s.root();
    Ok((s, r))
}

const TICKET_DIGEST_CONTEXT: &str = "ToyChain 2026-10 epoch ticket list v1";

/// Remembers the valid standalone tickets of block `height` for the base
/// pot (two blocks later), the jackpot Funke (60 blocks) and the
/// jackpots of the next epoch.
fn record_tickets(s: &mut State, height: Height, tickets: Vec<(Hash, Address)>) {
    let chain = s.chain_mut();
    if !tickets.is_empty() {
        let owners: Vec<Address> = tickets.iter().map(|(_, owner)| *owner).collect();
        let mut data = Vec::with_capacity(owners.len().saturating_mul(20).saturating_add(40));
        data.extend_from_slice(chain.epoch_tickets_digest.as_bytes());
        data.extend_from_slice(&height.to_be_bytes());
        for owner in &owners {
            data.extend_from_slice(owner.as_bytes());
        }
        chain.epoch_tickets_digest = blake3_derive(TICKET_DIGEST_CONTEXT, &data);
        chain.epoch_ticket_owners.extend_from_slice(&owners);
        chain.recent_tickets.push_back((height, owners));
        chain.pending_draws.insert(height, tickets);
    }
    // The next block draws Funke from heights `height − 60` and above.
    let keep_from = height.saturating_sub(JACKPOT_FUNKE_WINDOW_BLOCKS);
    while chain
        .recent_tickets
        .front()
        .is_some_and(|(h, _)| *h < keep_from)
    {
        chain.recent_tickets.pop_front();
    }
}

/// Sets the due height of milestone jackpots (spec 9.3): every millionth
/// block height and every low tide (from day 1,825, every 1,825 plan
/// days) start a Gezeiten jackpot, every full 10 million TOY of supply a
/// Welle jackpot, at a random block within the following epoch.
fn schedule_milestones(s: &mut State, ctx: &BlockContext, mix: &Hash) -> Result<(), BlockError> {
    let chain = s.chain();
    let supply_step = JACKPOT_MILESTONE_SUPPLY.units();
    let before = chain
        .minted_total
        .units()
        .checked_div(supply_step)
        .unwrap_or(0);
    let after = ctx
        .minted_total
        .units()
        .checked_div(supply_step)
        .unwrap_or(0);
    let tide_days = JACKPOT_MILESTONE_TIDE_DAYS;
    let tide_before = chain.plan_days.checked_div(tide_days).unwrap_or(0);
    let tide_after = ctx.plan_days.checked_div(tide_days).unwrap_or(0);
    let height_milestone = ctx
        .height
        .checked_rem(JACKPOT_MILESTONE_HEIGHT)
        .is_some_and(|r| r == 0);
    let mut due = Vec::new();
    if after > before {
        due.push(Jackpot::Welle);
    }
    if height_milestone || (tide_after > tide_before && tide_after >= 1) {
        due.push(Jackpot::Gezeiten);
    }
    for jackpot in due {
        let delay = milestone_delay(jackpot, mix, ctx.height, EPOCH_MIN_BLOCKS);
        let at = ctx
            .height
            .checked_add(delay)
            .ok_or(ArithmeticError::Overflow)?;
        let slot = s
            .chain_mut()
            .jackpot_due
            .get_mut(jackpot.index())
            .ok_or(ArithmeticError::Overflow)?;
        if slot.is_none() {
            *slot = Some(at);
        }
    }
    Ok(())
}

/// Trigger and draw of the three jackpots with the mix of this block.
fn run_jackpots(
    s: &mut State,
    ctx: &BlockContext,
    mix: &Hash,
    r: &mut BlockReceipt,
) -> Result<(), BlockError> {
    schedule_milestones(s, ctx, mix)?;
    let year = Amount::checked_sum(s.chain().minted_history.iter().copied())?
        .checked_add(s.chain().epoch_minted)?;
    let cap = year.mul_ppm(JACKPOT_MAX_SHARE_OF_YEAR)?;
    for (jackpot, (pot_id, inflow)) in Jackpot::ALL
        .into_iter()
        .zip(LotteryPot::JACKPOTS.into_iter().zip(ctx.mint.jackpots))
    {
        let pot = s.lottery_pot(pot_id);
        let due = s
            .chain()
            .jackpot_due
            .get(jackpot.index())
            .copied()
            .flatten();
        let fired = if due.is_some_and(|at| ctx.height >= at) && !pot.is_zero() {
            if let Some(slot) = s.chain_mut().jackpot_due.get_mut(jackpot.index()) {
                *slot = None;
            }
            Some(Trigger::Milestone)
        } else {
            let inputs = TriggerInputs {
                pot,
                inflow,
                tide: ctx.tide,
                participation: ctx.participation,
            };
            trigger(jackpot, &inputs, mix, ctx.height)?
        };
        let Some(why) = fired else { continue };
        let owners: Vec<Address> = match jackpot {
            Jackpot::Funke => {
                let newest = ctx.height.saturating_sub(LOTTERY_DRAW_DELAY_BLOCKS);
                s.chain()
                    .recent_tickets
                    .iter()
                    .filter(|(h, _)| *h <= newest)
                    .flat_map(|(_, owners)| owners.iter().copied())
                    .collect()
            }
            Jackpot::Welle | Jackpot::Gezeiten => s.chain().prev_epoch_ticket_owners.clone(),
        };
        let prizes = draw_jackpot(jackpot, &owners, pot, cap, mix, ctx.height, |a| {
            s.account(a).is_some()
        })?;
        if prizes.is_empty() {
            continue;
        }
        s.pay_lottery_prizes(pot_id, &prizes)?;
        r.jackpots.push(JackpotPayout {
            jackpot,
            trigger: why,
            prizes,
        });
    }
    Ok(())
}

fn start_epoch(s: &mut State, r: &mut BlockReceipt) -> Result<(), BlockError> {
    let previous = s.chain().clone();
    let cap = refund_pool_cap(previous.base_fee_per_byte, previous.epoch_blocks)?;
    let surplus = s.refund_pool().checked_sub(cap).unwrap_or(Amount::ZERO);
    if !surplus.is_zero() {
        s.move_refund_surplus_to_staking(surplus)?;
    }
    r.pool_surplus = surplus;
    let budget = refund_budget(s.refund_pool())?;
    // Water level from the free transactions of the previous epoch.
    let last_epoch = s.epoch().saturating_sub(1);
    let counts: Vec<u64> = s
        .accounts()
        .map(|(_, a)| a.activity(last_epoch).free_used)
        .filter(|c| *c > 0)
        .collect();
    let level = water_level(&counts, previous.epoch_blocks);
    let chain = s.chain_mut();
    chain.epoch_quota = quota(
        base_quota(previous.epoch_tx_bytes, previous.epoch_blocks),
        level,
    );
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
    // Tickets of the finished epoch feed Welle and Gezeiten.
    chain.prev_epoch_ticket_owners = core::mem::take(&mut chain.epoch_ticket_owners);
    chain.prev_epoch_tickets_digest =
        core::mem::replace(&mut chain.epoch_tickets_digest, Hash::from_bytes([0; 32]));
    // Emission of the last 365 epochs (with the current one) caps the
    // jackpots.
    chain.minted_history.push_back(previous.epoch_minted);
    while chain.minted_history.len() >= JACKPOT_YEAR_EPOCHS {
        chain.minted_history.pop_front();
    }
    chain.epoch_minted = Amount::ZERO;
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
    tickets: &mut Vec<(Hash, Address)>,
    r: &mut BlockReceipt,
) -> Result<(), TxError> {
    let ticket = decode_ticket(raw, ctx.chain)?;
    let claim = ticket.claim;
    if claim_ticket(s, ctx, &claim, hasher)? {
        tickets.push((claim.id(), claim.owner));
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
    // Staking rewards are credited with every own transaction (spec 9.4).
    s.settle_rewards(&sender)?;
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
    // From the 1,000th transaction of the epoch: paid only, twice the base
    // fee (decision 146).
    let heavy = is_heavy_use(activity.tx_count);
    if heavy && payment != Payment::Paid {
        return Err(TxError::HeavyUserMustPay);
    }

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
        }
        Payment::Free => {
            if s.is_locked(&sender, Right::FreeQuota) {
                return Err(TxError::QuotaLocked);
            }
            let allowed = account_quota(s.chain().epoch_quota, ctx.height.saturating_sub(created));
            if allowed == 0 {
                return Err(TxError::AccountTooYoung);
            }
            let chain = s.chain();
            if activity.free_used >= allowed {
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
            let required = required_fee(s.chain().base_fee_per_byte, tx.size, k_in_block, heavy)?;
            let fee = charged_fee(tx.max_fee, tx.max_tip, required)?;
            s.pay_fee(&sender, &split_fee(fee)?, &ctx.proposer)?;
            r.fees = r.fees.checked_add(fee)?;
            r.counted_tx = r.counted_tx.saturating_add(1);
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
