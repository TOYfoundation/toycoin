//! The single list of all pots that hold TOY (rule I0, spec section 9).
//!
//! Rule I0: in every block, the sum of all pots equals the sum of all
//! emissions. Nothing is ever burned (decision 74), so every TOY that was
//! minted is in exactly one pot.
//!
//! **Every new kind of transaction that keeps TOY somewhere new must add
//! its pot here** (spec 9) and come with a test for I0. [`Pot::ALL`] and
//! the exhaustive `match` in `State::pot_total` make the compiler insist
//! on it. Pots of later milestones are already listed and are zero until
//! their milestone fills them.

use core::fmt;

use toy_types::{Amount, ArithmeticError};

/// A place where TOY can be.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pot {
    /// Free balances of all accounts.
    Free,
    /// Activation deposits (spec 6.1).
    ActivationDeposit,
    /// Own bonded stake (tiers 2 to 4).
    OwnStake,
    /// Delegated stake (spec 7.4, M4). Always zero before M4.
    DelegatedStake,
    /// Stake in unbonding (21 epochs).
    Unbonding,
    /// Stake held after a double signature until slashing is fixed.
    HeldStake,
    /// Bonded rewards of the start validators (365 epochs).
    StartValidatorRewards,
    /// The refund pool that pays for free transactions.
    RefundPool,
    /// Pot of the work lottery (ToyWork tickets).
    WorkLottery,
    /// Pot of the fee lottery among active participants (paid out in M4).
    FeeLottery,
    /// Consensus share after the bootstrap phase, waiting for distribution
    /// by the consensus layer (decision 112, M4).
    ConsensusRewards,
    /// Pending vault withdrawals (spec 7.3, M12). Always zero before M12.
    VaultWithdrawals,
    /// Hash time locks (spec 7.1, M12). Always zero before M12.
    HashTimeLocks,
    /// Escrow amounts (spec 7.4, M12). Always zero before M12.
    Escrow,
    /// Balances of dormant accounts in the dormancy snapshots (spec 6.2,
    /// M5). Always zero before M5.
    DormantSnapshots,
}

impl Pot {
    /// Every pot, in a fixed order.
    pub const ALL: [Self; 15] = [
        Self::Free,
        Self::ActivationDeposit,
        Self::OwnStake,
        Self::DelegatedStake,
        Self::Unbonding,
        Self::HeldStake,
        Self::StartValidatorRewards,
        Self::RefundPool,
        Self::WorkLottery,
        Self::FeeLottery,
        Self::ConsensusRewards,
        Self::VaultWithdrawals,
        Self::HashTimeLocks,
        Self::Escrow,
        Self::DormantSnapshots,
    ];
}

/// Amounts of all pots at one point in time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PotTotals {
    entries: Vec<(Pot, Amount)>,
}

impl PotTotals {
    pub(crate) const fn new(entries: Vec<(Pot, Amount)>) -> Self {
        Self { entries }
    }

    /// Amount in `pot`.
    #[must_use]
    pub fn get(&self, pot: Pot) -> Amount {
        self.entries
            .iter()
            .find(|(p, _)| *p == pot)
            .map_or(Amount::ZERO, |(_, a)| *a)
    }

    /// All pots with their amounts, in the order of [`Pot::ALL`].
    #[must_use]
    pub fn entries(&self) -> &[(Pot, Amount)] {
        &self.entries
    }

    /// Sum over all pots.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] (not reachable below the supply cap).
    pub fn sum(&self) -> Result<Amount, ArithmeticError> {
        Amount::checked_sum(self.entries.iter().map(|(_, a)| *a))
    }
}

impl fmt::Display for PotTotals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (pot, amount) in &self.entries {
            writeln!(f, "{pot:?}: {amount}")?;
        }
        Ok(())
    }
}
