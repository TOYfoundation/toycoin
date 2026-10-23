//! Non-monetary consensus data that the transaction rules (`toy-tx`)
//! keep between blocks: base fee, free quota, refund budget, ToyWork
//! difficulty, used tickets and pending lottery draws.
//!
//! These values hold no TOY, so `toy-tx` may change them directly. They
//! are part of the state root, because every node must agree on them.

use std::collections::{BTreeMap, BTreeSet};

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
    /// Owners of the valid lottery tickets per committing block, waiting
    /// for the draw two blocks later.
    pub pending_draws: BTreeMap<Height, Vec<Address>>,
    /// Ids of double signatures already punished.
    pub reported_evidence: BTreeSet<Hash>,
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
        for (height, owners) in &self.pending_draws {
            out.extend_from_slice(&height.to_be_bytes());
            push_len(&mut out, owners.len());
            for owner in owners {
                out.extend_from_slice(owner.as_bytes());
            }
        }
        push_len(&mut out, self.reported_evidence.len());
        for id in &self.reported_evidence {
            out.extend_from_slice(id.as_bytes());
        }
        out
    }
}

fn push_len(out: &mut Vec<u8>, len: usize) {
    out.extend_from_slice(&u64::try_from(len).unwrap_or(u64::MAX).to_be_bytes());
}
