//! Accounts (spec sections 3, 4, 6.1).
//!
//! Every field is private. Balances change only through the methods of
//! [`crate::State`], which always move value between pots and never create
//! or destroy it (rule I0). Read access is open, so wallets, RPC and
//! ToyScan can show everything.

use toy_types::{Amount, ArithmeticError, EpochNumber, Height};

use crate::penalty::PenaltyRecord;

/// Length of a BLS12-381 public key in bytes (compressed G1 point).
pub const BLS_KEY_LEN: usize = 48;

/// Stake on its way back to the free balance (unbonding, 21 epochs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unbonding {
    /// Amount.
    pub amount: Amount,
    /// From this epoch on the amount is free again.
    pub release_epoch: EpochNumber,
}

/// Bonded start validator reward (spec 5.3: bonded for 365 epochs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RewardLock {
    /// Amount.
    pub amount: Amount,
    /// From this epoch on the amount counts as normal stake.
    pub unlock_epoch: EpochNumber,
}

/// Stake held after a double signature until the slashing is fixed
/// (spec 5.4: after 18 epochs; slashing itself follows in M4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeldStake {
    /// Amount.
    pub amount: Amount,
    /// Epoch in which the slashing is fixed and the rest is returned.
    pub release_epoch: EpochNumber,
}

/// Registered validator role (tier 4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidatorInfo {
    /// Compressed BLS12-381 public key, ownership proven at registration.
    pub bls_key: [u8; BLS_KEY_LEN],
}

/// Per-epoch counters of an account. They belong to one epoch and are
/// reset lazily when the account is first used in a new epoch.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EpochActivity {
    /// Epoch the counters belong to.
    pub epoch: EpochNumber,
    /// Own transactions in the epoch (all kinds).
    pub tx_count: u64,
    /// Free-quota transactions used in the epoch.
    pub free_used: u64,
    /// Work-backed transactions beyond the heavy-user threshold.
    pub work_backed_over: u64,
    /// New accounts activated by transfers from this account.
    pub new_accounts: u64,
    /// `true` once mass account creation was penalized in this epoch.
    pub mass_creation_flagged: bool,
}

impl EpochActivity {
    /// Counters for `epoch`: the stored ones if they belong to it,
    /// otherwise fresh zeros.
    #[must_use]
    pub fn for_epoch(&self, epoch: EpochNumber) -> Self {
        if self.epoch == epoch {
            *self
        } else {
            Self {
                epoch,
                ..Self::default()
            }
        }
    }
}

/// An active account.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Account {
    pub(crate) nonce: u64,
    pub(crate) free: Amount,
    pub(crate) deposit: Amount,
    pub(crate) created_height: Height,
    pub(crate) stake: Amount,
    pub(crate) unbonding: Vec<Unbonding>,
    pub(crate) reward_locks: Vec<RewardLock>,
    pub(crate) held: Vec<HeldStake>,
    pub(crate) validator: Option<ValidatorInfo>,
    pub(crate) activity: EpochActivity,
    pub(crate) penalties: PenaltyRecord,
}

impl Account {
    pub(crate) fn new(
        free: Amount,
        deposit: Amount,
        height: Height,
        nonce: u64,
        penalties: PenaltyRecord,
    ) -> Self {
        Self {
            nonce,
            free,
            deposit,
            created_height: height,
            stake: Amount::ZERO,
            unbonding: Vec::new(),
            reward_locks: Vec::new(),
            held: Vec::new(),
            validator: None,
            activity: EpochActivity::default(),
            penalties,
        }
    }

    /// Next expected transaction nonce.
    #[must_use]
    pub const fn nonce(&self) -> u64 {
        self.nonce
    }

    /// Freely spendable balance.
    #[must_use]
    pub const fn free(&self) -> Amount {
        self.free
    }

    /// Activation deposit (owned by the account, never slashed).
    #[must_use]
    pub const fn deposit(&self) -> Amount {
        self.deposit
    }

    /// Height of the block that created (activated) the account.
    #[must_use]
    pub const fn created_height(&self) -> Height {
        self.created_height
    }

    /// Own bonded stake (tiers 2 to 4).
    #[must_use]
    pub const fn stake(&self) -> Amount {
        self.stake
    }

    /// Stake in unbonding.
    #[must_use]
    pub fn unbonding(&self) -> &[Unbonding] {
        &self.unbonding
    }

    /// Bonded start validator rewards.
    #[must_use]
    pub fn reward_locks(&self) -> &[RewardLock] {
        &self.reward_locks
    }

    /// Stake held after a double signature.
    #[must_use]
    pub fn held(&self) -> &[HeldStake] {
        &self.held
    }

    /// Validator registration, if any.
    #[must_use]
    pub const fn validator(&self) -> Option<&ValidatorInfo> {
        self.validator.as_ref()
    }

    /// Per-epoch counters as of `epoch`.
    #[must_use]
    pub fn activity(&self, epoch: EpochNumber) -> EpochActivity {
        self.activity.for_epoch(epoch)
    }

    /// Penalty status and history.
    #[must_use]
    pub const fn penalties(&self) -> &PenaltyRecord {
        &self.penalties
    }

    /// Sum of all unbonding entries.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] (not reachable below the supply cap).
    pub fn unbonding_total(&self) -> Result<Amount, ArithmeticError> {
        Amount::checked_sum(self.unbonding.iter().map(|u| u.amount))
    }

    /// Sum of all bonded start validator rewards.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] (not reachable below the supply cap).
    pub fn reward_locks_total(&self) -> Result<Amount, ArithmeticError> {
        Amount::checked_sum(self.reward_locks.iter().map(|r| r.amount))
    }

    /// Sum of all held stake.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] (not reachable below the supply cap).
    pub fn held_total(&self) -> Result<Amount, ArithmeticError> {
        Amount::checked_sum(self.held.iter().map(|h| h.amount))
    }

    /// Everything the account owns, in all its pots.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] (not reachable below the supply cap).
    pub fn total(&self) -> Result<Amount, ArithmeticError> {
        Amount::checked_sum([
            self.free,
            self.deposit,
            self.stake,
            self.unbonding_total()?,
            self.reward_locks_total()?,
            self.held_total()?,
        ])
    }

    /// Canonical encoding for the state root. Optional parts are present
    /// only when used, so a plain account stays small (spec 6: about 100
    /// to 200 bytes per account including the Merkle tree).
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut flags = 0u8;
        let sections: [(u8, bool); 6] = [
            (1, !self.stake.is_zero()),
            (2, !self.unbonding.is_empty()),
            (4, !self.reward_locks.is_empty()),
            (8, !self.held.is_empty()),
            (16, self.validator.is_some()),
            (32, self.activity != EpochActivity::default()),
        ];
        for (bit, present) in sections {
            if present {
                flags |= bit;
            }
        }
        let has_penalties = self.penalties != PenaltyRecord::default();
        if has_penalties {
            flags |= 64;
        }
        let mut out = Vec::with_capacity(64);
        out.push(flags);
        out.extend_from_slice(&self.nonce.to_be_bytes());
        out.extend_from_slice(&self.free.units().to_be_bytes());
        out.extend_from_slice(&self.deposit.units().to_be_bytes());
        out.extend_from_slice(&self.created_height.to_be_bytes());
        if !self.stake.is_zero() {
            out.extend_from_slice(&self.stake.units().to_be_bytes());
        }
        encode_entries(
            &mut out,
            self.unbonding.iter().map(|u| (u.amount, u.release_epoch)),
        );
        encode_entries(
            &mut out,
            self.reward_locks.iter().map(|r| (r.amount, r.unlock_epoch)),
        );
        encode_entries(
            &mut out,
            self.held.iter().map(|h| (h.amount, h.release_epoch)),
        );
        if let Some(v) = &self.validator {
            out.extend_from_slice(&v.bls_key);
        }
        if self.activity != EpochActivity::default() {
            let a = &self.activity;
            for value in [
                a.epoch,
                a.tx_count,
                a.free_used,
                a.work_backed_over,
                a.new_accounts,
            ] {
                out.extend_from_slice(&value.to_be_bytes());
            }
            out.push(u8::from(a.mass_creation_flagged));
        }
        if has_penalties {
            self.penalties.encode_into(&mut out);
        }
        out
    }
}

fn encode_entries(out: &mut Vec<u8>, entries: impl ExactSizeIterator<Item = (Amount, u64)>) {
    if entries.len() == 0 {
        return;
    }
    let len = u32::try_from(entries.len()).unwrap_or(u32::MAX);
    out.extend_from_slice(&len.to_be_bytes());
    for (amount, epoch) in entries {
        out.extend_from_slice(&amount.units().to_be_bytes());
        out.extend_from_slice(&epoch.to_be_bytes());
    }
}
