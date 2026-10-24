//! Protection rules of level A: penalty periods (spec section 8,
//! decision 67).
//!
//! A penalty only removes a **right** (role, proposal right, tickets, free
//! quota). It never touches a balance: nothing in this module knows about
//! amounts. The only rule that affects TOY is the slashing of voluntarily
//! bonded stake after a double signature (spec 5.4, milestone M4); until
//! then that stake is only *held* (still owned, still counted by rule I0).
//!
//! Escalation: every further violation of the same address doubles the
//! period, up to 64 epochs. After 30 epochs without a violation, counting
//! starts afresh.
//!
//! Timing (decision 121): a penalty of `N` epochs imposed in epoch `e`
//! lasts for the rest of epoch `e` and the `N` full epochs after it, so
//! the right is back from epoch `e + N + 1` on. This way a violation late
//! in an epoch is not cheaper than one early in the epoch.

use toy_types::EpochNumber;
use toy_types::params::{
    INACTIVITY_EPOCHS, MASS_CREATION_LIMIT, PENALTY_DOUBLE_SIGN_EPOCHS,
    PENALTY_INVALID_PROPOSAL_EPOCHS, PENALTY_INVALID_TICKET_EPOCHS, PENALTY_MASS_CREATION_EPOCHS,
    PENALTY_MAX_EPOCHS, PENALTY_RESET_EPOCHS,
};

/// A violation that the chain can prove from data alone (level A).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Violation {
    /// Two conflicting votes of the same key for the same height, round
    /// and phase.
    DoubleSign,
    /// A signed block that is invalid (evidence format follows with M4).
    InvalidProposal,
    /// A signed ToyWork ticket whose work check fails.
    InvalidTicket,
    /// More than 20 new accounts in one epoch from one sender.
    MassAccountCreation,
    /// Missing committee signatures in 3 consecutive epochs. This pauses
    /// the role until the node reports back; it has no period and does
    /// not count for escalation (decision 121).
    Inactivity,
}

impl Violation {
    /// All violations, in a fixed order.
    pub const ALL: [Self; 5] = [
        Self::DoubleSign,
        Self::InvalidProposal,
        Self::InvalidTicket,
        Self::MassAccountCreation,
        Self::Inactivity,
    ];

    /// The right this violation removes.
    #[must_use]
    pub const fn right(self) -> Right {
        match self {
            Self::DoubleSign | Self::Inactivity => Right::Role,
            Self::InvalidProposal => Right::Proposal,
            Self::InvalidTicket => Right::Tickets,
            Self::MassAccountCreation => Right::FreeQuota,
        }
    }

    /// Base period in epochs for a first violation (`None`: no period).
    #[must_use]
    pub const fn base_epochs(self) -> Option<u64> {
        match self {
            Self::DoubleSign => Some(PENALTY_DOUBLE_SIGN_EPOCHS),
            Self::InvalidProposal => Some(PENALTY_INVALID_PROPOSAL_EPOCHS),
            Self::InvalidTicket => Some(PENALTY_INVALID_TICKET_EPOCHS),
            Self::MassAccountCreation => Some(PENALTY_MASS_CREATION_EPOCHS),
            Self::Inactivity => None,
        }
    }

    const fn code(self) -> u8 {
        match self {
            Self::DoubleSign => 1,
            Self::InvalidProposal => 2,
            Self::InvalidTicket => 3,
            Self::MassAccountCreation => 4,
            Self::Inactivity => 5,
        }
    }
}

/// A right that a penalty can remove. Balances are never a right that can
/// be removed (constitution, spec 13 no. 8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Right {
    /// Signing as committee member or attester.
    Role,
    /// Proposing blocks.
    Proposal,
    /// Submitting ToyWork tickets (lottery and fee substitute).
    Tickets,
    /// The free transaction quota.
    FreeQuota,
}

impl Right {
    /// All rights, in a fixed order.
    pub const ALL: [Self; 4] = [Self::Role, Self::Proposal, Self::Tickets, Self::FreeQuota];

    const fn index(self) -> usize {
        match self {
            Self::Role => 0,
            Self::Proposal => 1,
            Self::Tickets => 2,
            Self::FreeQuota => 3,
        }
    }
}

/// Result of recording a violation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PenaltyOutcome {
    /// The violation.
    pub violation: Violation,
    /// Period in epochs (`None` for a pause without period).
    pub epochs: Option<u64>,
    /// Last epoch in which the right is removed (`None` for a pause).
    pub locked_through: Option<EpochNumber>,
    /// Strike number after this violation (1 for a first violation).
    pub strike: u32,
}

/// Penalty status and history of one address.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PenaltyRecord {
    /// Violations since the last reset (escalation counter).
    strikes: u32,
    /// Epoch of the last counted violation.
    last_violation: Option<EpochNumber>,
    /// Per right: last epoch in which it is removed.
    locked_through: [Option<EpochNumber>; 4],
    /// Role paused for inactivity until the node reports back.
    role_paused: bool,
}

impl PenaltyRecord {
    /// Records a violation in `epoch` and returns the imposed penalty.
    ///
    /// Periods never shorten an existing lock: if a longer lock is already
    /// running for the same right, it stays.
    pub fn record(&mut self, violation: Violation, epoch: EpochNumber) -> PenaltyOutcome {
        let Some(base) = violation.base_epochs() else {
            self.role_paused = true;
            return PenaltyOutcome {
                violation,
                epochs: None,
                locked_through: None,
                strike: self.strikes,
            };
        };
        if let Some(last) = self.last_violation
            && epoch.saturating_sub(last) >= PENALTY_RESET_EPOCHS
        {
            self.strikes = 0;
        }
        self.strikes = self.strikes.saturating_add(1);
        self.last_violation = Some(epoch);
        let doublings = self.strikes.saturating_sub(1);
        let factor = 1u64.checked_shl(doublings).unwrap_or(u64::MAX);
        let epochs = base.saturating_mul(factor).min(PENALTY_MAX_EPOCHS);
        let until = epoch.saturating_add(epochs);
        let slot = self
            .locked_through
            .get_mut(violation.right().index())
            .map(|s| {
                let new = s.map_or(until, |old| old.max(until));
                *s = Some(new);
                new
            });
        PenaltyOutcome {
            violation,
            epochs: Some(epochs),
            locked_through: slot,
            strike: self.strikes,
        }
    }

    /// `true` if `right` is removed in `epoch`.
    #[must_use]
    pub fn is_locked(&self, right: Right, epoch: EpochNumber) -> bool {
        let locked = self
            .locked_through
            .get(right.index())
            .copied()
            .flatten()
            .is_some_and(|until| epoch <= until);
        locked || (right == Right::Role && self.role_paused)
    }

    /// Last epoch in which `right` is removed by a period (ignores pauses).
    #[must_use]
    pub fn locked_through(&self, right: Right) -> Option<EpochNumber> {
        self.locked_through.get(right.index()).copied().flatten()
    }

    /// `true` if the role is paused for inactivity.
    #[must_use]
    pub const fn role_paused(&self) -> bool {
        self.role_paused
    }

    /// Ends an inactivity pause (the node reported back). Periods keep
    /// running.
    pub fn resume_role(&mut self) {
        self.role_paused = false;
    }

    /// Current strike count (escalation).
    #[must_use]
    pub const fn strikes(&self) -> u32 {
        self.strikes
    }

    /// Epoch of the last counted violation.
    #[must_use]
    pub const fn last_violation(&self) -> Option<EpochNumber> {
        self.last_violation
    }

    /// `true` if there is anything worth keeping after the account closes.
    #[must_use]
    pub fn has_history(&self) -> bool {
        self.strikes > 0 || self.role_paused || self.locked_through.iter().any(Option::is_some)
    }

    /// Appends the canonical encoding (for the state root).
    pub(crate) fn encode_into(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.strikes.to_be_bytes());
        encode_opt_u64(out, self.last_violation);
        for slot in self.locked_through {
            encode_opt_u64(out, slot);
        }
        out.push(u8::from(self.role_paused));
    }
}

fn encode_opt_u64(out: &mut Vec<u8>, value: Option<u64>) {
    match value {
        Some(v) => {
            out.push(1);
            out.extend_from_slice(&v.to_be_bytes());
        }
        None => out.push(0),
    }
}

/// Inactivity rule (spec 8): a committee member that missed its
/// signatures in this many consecutive epochs is paused. The consensus
/// (M4) counts the epochs; the rule itself is fixed here.
#[must_use]
pub const fn inactivity_violated(consecutive_missing_epochs: u64) -> bool {
    consecutive_missing_epochs >= INACTIVITY_EPOCHS
}

/// Mass account creation rule (spec 8): more than 20 new accounts from
/// one sender in one epoch.
#[must_use]
pub const fn mass_creation_violated(new_accounts_in_epoch: u64) -> bool {
    new_accounts_in_epoch > MASS_CREATION_LIMIT
}

/// Encodes a violation kind as one byte (evidence ids, events).
#[must_use]
pub const fn violation_code(violation: Violation) -> u8 {
    violation.code()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_violation_uses_base_period() {
        for v in Violation::ALL {
            let mut rec = PenaltyRecord::default();
            let out = rec.record(v, 10);
            match v.base_epochs() {
                Some(base) => {
                    assert_eq!(out.epochs, Some(base));
                    assert_eq!(out.locked_through, Some(10 + base));
                    assert!(rec.is_locked(v.right(), 10));
                    assert!(rec.is_locked(v.right(), 10 + base));
                    assert!(!rec.is_locked(v.right(), 11 + base));
                }
                None => {
                    assert!(rec.role_paused());
                    assert!(rec.is_locked(Right::Role, 10_000));
                }
            }
            // Other rights are untouched.
            for r in Right::ALL {
                if r != v.right() {
                    assert!(!rec.is_locked(r, 10), "{v:?} locked {r:?}");
                }
            }
        }
    }

    #[test]
    fn escalation_doubles_up_to_64() {
        let mut rec = PenaltyRecord::default();
        let periods: Vec<u64> = (0..7)
            .map(|i| rec.record(Violation::DoubleSign, 100 + i).epochs.unwrap())
            .collect();
        assert_eq!(periods, [7, 14, 28, 56, 64, 64, 64]);

        let mut rec = PenaltyRecord::default();
        let periods: Vec<u64> = (0..8)
            .map(|i| rec.record(Violation::InvalidTicket, i).epochs.unwrap())
            .collect();
        assert_eq!(periods, [1, 2, 4, 8, 16, 32, 64, 64]);
    }

    #[test]
    fn escalation_counts_across_kinds() {
        let mut rec = PenaltyRecord::default();
        rec.record(Violation::MassAccountCreation, 5);
        let second = rec.record(Violation::InvalidTicket, 6);
        assert_eq!(second.epochs, Some(2));
        assert_eq!(second.strike, 2);
    }

    #[test]
    fn reset_after_30_clean_epochs() {
        let mut rec = PenaltyRecord::default();
        rec.record(Violation::InvalidTicket, 0);
        rec.record(Violation::InvalidTicket, 5);
        // 29 epochs later: still counting (third strike, 4 epochs).
        assert_eq!(rec.record(Violation::InvalidTicket, 34).epochs, Some(4));
        // 30 epochs after the last one: starts afresh.
        let out = rec.record(Violation::InvalidTicket, 64);
        assert_eq!(out.epochs, Some(1));
        assert_eq!(out.strike, 1);
    }

    #[test]
    fn longer_lock_is_never_shortened() {
        let mut rec = PenaltyRecord::default();
        for e in 0..5 {
            rec.record(Violation::MassAccountCreation, e);
        }
        let long = rec.locked_through(Right::FreeQuota).unwrap();
        // After 30 clean epochs a new first violation gives 7 epochs, but
        // an older lock that still runs longer is kept.
        assert!(long >= 30 + 7);
        let mut fresh = rec.clone();
        fresh.record(Violation::MassAccountCreation, 34);
        assert_eq!(
            fresh.locked_through(Right::FreeQuota),
            Some(long.max(34 + 7))
        );
    }

    #[test]
    fn rule_thresholds() {
        assert!(!inactivity_violated(2));
        assert!(inactivity_violated(3));
        assert!(!mass_creation_violated(20));
        assert!(mass_creation_violated(21));
    }

    #[test]
    fn inactivity_pause_does_not_escalate_and_can_be_resumed() {
        let mut rec = PenaltyRecord::default();
        rec.record(Violation::Inactivity, 3);
        assert_eq!(rec.strikes(), 0);
        assert!(rec.is_locked(Right::Role, 3));
        rec.resume_role();
        assert!(!rec.is_locked(Right::Role, 3));
        assert!(!rec.has_history());
    }
}
