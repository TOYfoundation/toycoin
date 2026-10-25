//! Non-monetary consensus data that the transaction rules (`toy-tx`)
//! keep between blocks: base fee, free quota, refund budget, ToyWork
//! difficulty, used tickets, pending lottery draws, the ticket history for
//! the jackpots, the randomness mix and the emission history.
//!
//! These values hold no TOY, so `toy-tx` may change them directly. They
//! are part of the state root, because every node must agree on them.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use toy_types::params::{BASE_FEE_MIN_PER_BYTE, QUOTA_MAX, TOYWORK_START_THRESHOLD};
use toy_types::{Address, Amount, Hash, Height};

/// Rule data carried from block to block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainData {
    /// Current base fee per transaction byte (EIP-1559 style).
    pub base_fee_per_byte: Amount,
    /// ToyWork difficulty: a ticket is valid if its work value is below.
    pub ticket_threshold: u128,
    /// Free quota per account in the current epoch.
    pub epoch_quota: u64,
    /// Refunds allowed in the current epoch (1/30 of the pool).
    pub refund_budget: Amount,
    /// Refunds already used in the current epoch.
    pub refund_used: Amount,
    /// Randomness of the last block of the previous epoch. Tickets of this
    /// epoch are bound to it, so they cannot be computed in advance.
    pub epoch_seed: Hash,
    /// Randomness of the last applied block.
    pub last_randomness: Hash,
    /// Bytes of all transactions in the current epoch.
    pub epoch_tx_bytes: u64,
    /// Blocks in the current epoch.
    pub epoch_blocks: u64,
    /// Valid tickets in the current epoch (difficulty adjustment).
    pub epoch_tickets: u64,
    /// Ids of all tickets used in the current epoch (no reuse).
    pub used_tickets: BTreeSet<Hash>,
    /// Valid lottery tickets `(ticket id, owner)` per committing block,
    /// waiting for the draw of the base pot two blocks later.
    pub pending_draws: BTreeMap<Height, Vec<(Hash, Address)>>,
    /// Ids of double signatures already punished.
    pub reported_evidence: BTreeSet<Hash>,
    /// Randomness mix `R_b = BLAKE3(R_(b−1) ‖ VRF output of block b)`
    /// (spec 9.3, decision 150).
    pub mix: Hash,
    /// Owners of the valid tickets of the last 60 blocks (jackpot Funke).
    pub recent_tickets: VecDeque<(Height, Vec<Address>)>,
    /// Owners of all valid tickets of the current epoch, in block order.
    /// Bound into the state root through [`Self::epoch_tickets_digest`].
    pub epoch_ticket_owners: Vec<Address>,
    /// Running BLAKE3 digest over [`Self::epoch_ticket_owners`].
    pub epoch_tickets_digest: Hash,
    /// Owners of all valid tickets of the previous epoch (jackpots Welle
    /// and Gezeiten).
    pub prev_epoch_ticket_owners: Vec<Address>,
    /// Digest of [`Self::prev_epoch_ticket_owners`].
    pub prev_epoch_tickets_digest: Hash,
    /// Height from which a milestone jackpot is due: Funke, Welle,
    /// Gezeiten (only Welle and Gezeiten have milestones).
    pub jackpot_due: [Option<Height>; 3],
    /// Emission minted in the current epoch so far.
    pub epoch_minted: Amount,
    /// Emission of the last up to 365 finished epochs (jackpot cap).
    pub minted_history: VecDeque<Amount>,
    /// Sum of all emissions after the previous block (milestones).
    pub minted_total: Amount,
    /// Full plan days at the previous block (milestones).
    pub plan_days: u64,
}

impl ChainData {
    /// Values at genesis.
    #[must_use]
    pub fn genesis(genesis_randomness: Hash) -> Self {
        Self {
            base_fee_per_byte: BASE_FEE_MIN_PER_BYTE,
            ticket_threshold: TOYWORK_START_THRESHOLD,
            // Utilization before the first epoch is zero.
            epoch_quota: QUOTA_MAX,
            // The pool is empty at genesis: no refunds in epoch 0.
            refund_budget: Amount::ZERO,
            refund_used: Amount::ZERO,
            epoch_seed: genesis_randomness,
            last_randomness: genesis_randomness,
            epoch_tx_bytes: 0,
            epoch_blocks: 0,
            epoch_tickets: 0,
            used_tickets: BTreeSet::new(),
            pending_draws: BTreeMap::new(),
            reported_evidence: BTreeSet::new(),
            mix: genesis_randomness,
            recent_tickets: VecDeque::new(),
            epoch_ticket_owners: Vec::new(),
            epoch_tickets_digest: Hash::from_bytes([0; 32]),
            prev_epoch_ticket_owners: Vec::new(),
            prev_epoch_tickets_digest: Hash::from_bytes([0; 32]),
            jackpot_due: [None; 3],
            epoch_minted: Amount::ZERO,
            minted_history: VecDeque::new(),
            minted_total: Amount::ZERO,
            plan_days: 0,
        }
    }

    /// Canonical encoding for the state root.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.base_fee_per_byte.units().to_be_bytes());
        out.extend_from_slice(&self.ticket_threshold.to_be_bytes());
        out.extend_from_slice(&self.epoch_quota.to_be_bytes());
        out.extend_from_slice(&self.refund_budget.units().to_be_bytes());
        out.extend_from_slice(&self.refund_used.units().to_be_bytes());
        out.extend_from_slice(self.epoch_seed.as_bytes());
        out.extend_from_slice(self.last_randomness.as_bytes());
        for v in [self.epoch_tx_bytes, self.epoch_blocks, self.epoch_tickets] {
            out.extend_from_slice(&v.to_be_bytes());
        }
        push_len(&mut out, self.used_tickets.len());
        for id in &self.used_tickets {
            out.extend_from_slice(id.as_bytes());
        }
        push_len(&mut out, self.pending_draws.len());
        for (height, tickets) in &self.pending_draws {
            out.extend_from_slice(&height.to_be_bytes());
            push_len(&mut out, tickets.len());
            for (id, owner) in tickets {
                out.extend_from_slice(id.as_bytes());
                out.extend_from_slice(owner.as_bytes());
            }
        }
        push_len(&mut out, self.reported_evidence.len());
        for id in &self.reported_evidence {
            out.extend_from_slice(id.as_bytes());
        }
        out.extend_from_slice(self.mix.as_bytes());
        push_len(&mut out, self.recent_tickets.len());
        for (height, owners) in &self.recent_tickets {
            out.extend_from_slice(&height.to_be_bytes());
            push_len(&mut out, owners.len());
            for owner in owners {
                out.extend_from_slice(owner.as_bytes());
            }
        }
        // The ticket lists of the epochs are bound by their digests and
        // lengths; encoding them in full every block would be too costly.
        push_len(&mut out, self.epoch_ticket_owners.len());
        out.extend_from_slice(self.epoch_tickets_digest.as_bytes());
        push_len(&mut out, self.prev_epoch_ticket_owners.len());
        out.extend_from_slice(self.prev_epoch_tickets_digest.as_bytes());
        for due in self.jackpot_due {
            out.extend_from_slice(&due.map_or(u64::MAX, |h| h).to_be_bytes());
        }
        out.extend_from_slice(&self.epoch_minted.units().to_be_bytes());
        push_len(&mut out, self.minted_history.len());
        for amount in &self.minted_history {
            out.extend_from_slice(&amount.units().to_be_bytes());
        }
        out.extend_from_slice(&self.minted_total.units().to_be_bytes());
        out.extend_from_slice(&self.plan_days.to_be_bytes());
        out
    }
}

fn push_len(out: &mut Vec<u8>, len: usize) {
    out.extend_from_slice(&u64::try_from(len).unwrap_or(u64::MAX).to_be_bytes());
}
