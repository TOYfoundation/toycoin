//! Emission per epoch (spec 9 and 9.0).

use toy_types::params::{K_SCALE, MAX_SUPPLY};
use toy_types::{Amount, ArithmeticError, EpochNumber, Ppm};

use crate::bootstrap;
use crate::error::EmissionError;
use crate::factor::{self, EpochMeasurements};
use crate::ktable;
use crate::plan::{PlanCurve, plan_step};

/// Emission of an epoch for a remaining amount, the rate `k` from the K
/// table (in 10^-12) and the factor `m`:
/// `floor(floor(remaining × k ÷ 10^6) × m ÷ 10^12)` (spec 9.1). The largest
/// intermediate value is about 10^35 and fits `u128`.
///
/// # Errors
/// [`ArithmeticError::Overflow`] if an intermediate product does not fit.
pub fn emission_for(remaining: Amount, k: u64, factor: Ppm) -> Result<Amount, ArithmeticError> {
    // k is in 10^-12 and m in 10^-6: divide by 10^6 first, then by 10^12.
    remaining
        .mul_div_floor(u128::from(k), u128::from(Ppm::SCALE))?
        .mul_div_floor(u128::from(factor.get()), K_SCALE)
}

/// Who receives the consensus share of an epoch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Bootstrap phase without stake (spec 5.3): the start validators.
    Bootstrap,
    /// After the handover: stakers of tiers 2 to 4 (spec 9, decision 112).
    Normal,
}

/// Input to start an epoch, taken at its first block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EpochStart {
    /// Number of the epoch to start.
    pub epoch: EpochNumber,
    /// Full plan days up to the last block of the previous epoch.
    pub plan_days: u64,
    /// Measurements of the previous epoch (`None` for epoch 0).
    pub measurements: Option<EpochMeasurements>,
    /// Phase of this epoch, decided by the consensus layer.
    pub phase: Phase,
}

/// The fixed emission of one epoch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EpochEmission {
    /// Epoch number.
    pub epoch: EpochNumber,
    /// Total emission `E` of the epoch.
    pub emission: Amount,
    /// Factor `m` used.
    pub factor: Ppm,
    /// Full plan days `t` at the start of the epoch.
    pub plan_days: u64,
    /// Phase of the epoch.
    pub phase: Phase,
    /// Start wage: share of each block emission for the start validators
    /// after the cap (zero outside the bootstrap phase).
    pub start_validator_share: Ppm,
    /// Rate `k(t)` from the K table (10^-12 per epoch).
    pub k: u64,
    /// Work share `w(t)` from the K table.
    pub work_share: Ppm,
    /// Tide `W(t)` from the K table.
    pub tide: Ppm,
}

/// Consensus state of the emission. Only [`EmissionState::start_epoch`]
/// changes it, and only by adding to the issued sum: no operation lowers
/// it, so slashing or donations can never raise the remaining amount.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmissionState {
    next_epoch: EpochNumber,
    issued: Amount,
    factor: Ppm,
    last_plan_days: u64,
    reward_plan_mark: Amount,
    handover_done: bool,
    plan: PlanCurve,
}

impl Default for EmissionState {
    fn default() -> Self {
        Self::genesis()
    }
}

impl EmissionState {
    /// State at genesis: nothing issued, m = 1.0 (total fair launch).
    #[must_use]
    pub const fn genesis() -> Self {
        Self {
            next_epoch: 0,
            issued: Amount::ZERO,
            factor: Ppm::ONE,
            last_plan_days: 0,
            reward_plan_mark: Amount::ZERO,
            handover_done: false,
            plan: PlanCurve::new(),
        }
    }

    /// Sum of all epoch emissions fixed so far, the running epoch included.
    /// At the end of every epoch it equals the minted sum.
    #[must_use]
    pub const fn issued(&self) -> Amount {
        self.issued
    }

    /// `10^26 - issued`.
    #[must_use]
    pub fn remaining(&self) -> Amount {
        MAX_SUPPLY.checked_sub(self.issued).unwrap_or(Amount::ZERO)
    }

    /// Factor `m` of the latest epoch.
    #[must_use]
    pub const fn factor(&self) -> Ppm {
        self.factor
    }

    /// Number of the next epoch to start.
    #[must_use]
    pub const fn next_epoch(&self) -> EpochNumber {
        self.next_epoch
    }

    /// `true` once an epoch ran in the normal phase.
    #[must_use]
    pub const fn handover_done(&self) -> bool {
        self.handover_done
    }

    /// Fixes the emission of the next epoch. The state changes only if the
    /// call succeeds.
    ///
    /// # Errors
    /// See [`EmissionError`]; in particular epochs must come in order,
    /// plan time must not go backwards, epochs after 0 need measurements,
    /// and the bootstrap phase cannot come back after the handover.
    pub fn start_epoch(&mut self, start: &EpochStart) -> Result<EpochEmission, EmissionError> {
        if start.epoch != self.next_epoch {
            return Err(EmissionError::UnexpectedEpoch {
                expected: self.next_epoch,
                got: start.epoch,
            });
        }
        if start.plan_days < self.last_plan_days {
            return Err(EmissionError::PlanTimeWentBackwards);
        }
        if self.handover_done && start.phase == Phase::Bootstrap {
            return Err(EmissionError::PhaseRegression);
        }
        let mut plan = self.plan.clone();
        let plan_now = plan.at(start.plan_days)?;
        let factor = if start.epoch == 0 {
            Ppm::ONE
        } else {
            let measurements = start
                .measurements
                .ok_or(EmissionError::MissingMeasurements)?;
            factor::next_factor(self.factor, &measurements, self.issued, plan_now)?.factor
        };
        let remaining = MAX_SUPPLY
            .checked_sub(self.issued)
            .map_err(|_| EmissionError::SupplyCapExceeded)?;
        let row = ktable::at(start.plan_days);
        let emission = emission_for(remaining, row.k, factor)?;
        if emission > remaining {
            return Err(EmissionError::SupplyCapExceeded);
        }
        let issued = self.issued.checked_add(emission)?;
        if issued > MAX_SUPPLY {
            return Err(EmissionError::SupplyCapExceeded);
        }

        let plan_next = plan_step(plan_now, start.plan_days)?;
        let plan_delta = plan_next
            .checked_sub(self.reward_plan_mark)
            .map_err(|_| EmissionError::Inconsistent)?;
        let start_validator_share = match start.phase {
            Phase::Bootstrap => bootstrap::effective_share(start.plan_days, emission, plan_delta)?,
            Phase::Normal => Ppm::ZERO,
        };

        self.next_epoch = self
            .next_epoch
            .checked_add(1)
            .ok_or(ArithmeticError::Overflow)?;
        self.issued = issued;
        self.factor = factor;
        self.last_plan_days = start.plan_days;
        self.reward_plan_mark = plan_next;
        self.plan = plan;
        if start.phase == Phase::Normal {
            self.handover_done = true;
        }
        Ok(EpochEmission {
            epoch: start.epoch,
            emission,
            factor,
            plan_days: start.plan_days,
            phase: start.phase,
            start_validator_share,
            k: row.k,
            work_share: Ppm::new(u64::from(row.w)),
            tide: Ppm::new(u64::from(row.tide)),
        })
    }
}
