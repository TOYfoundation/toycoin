//! Emission factor `m` (spec 9.1, decision 81).
//!
//! All values are integers in millionths. Divisions round towards negative
//! infinity (`div_euclid`), exactly like `//` in the reference simulation
//! `docs/anhang/m-simulation.py`. Intermediate values are `i128`.

use toy_types::params::{FACTOR_MAX, FACTOR_MAX_STEP, FACTOR_MIN, USAGE_CAPACITY_PER_BLOCK};
use toy_types::{Amount, ArithmeticError, Ppm};

use crate::error::EmissionError;

const SCALE: i128 = 1_000_000;

/// Measurements of one finished epoch, the input for the next `m`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EpochMeasurements {
    /// Blocks in the epoch.
    pub blocks: u64,
    /// Transactions that paid a fee or carried a work ticket. A batch
    /// transfer counts as one.
    pub counted_tx: u64,
    /// Transactions from the free quota. Recorded for transparency only:
    /// they never influence `m`, otherwise spam could raise the emission.
    pub free_tx: u64,
    /// Signatures received from the committee.
    pub signatures_received: u64,
    /// Signatures expected: committee size times blocks.
    pub signatures_expected: u64,
}

impl EpochMeasurements {
    /// Adds the numbers of another block or epoch.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] if a counter overflows.
    pub fn checked_add(&self, other: &Self) -> Result<Self, ArithmeticError> {
        let add = |a: u64, b: u64| a.checked_add(b).ok_or(ArithmeticError::Overflow);
        Ok(Self {
            blocks: add(self.blocks, other.blocks)?,
            counted_tx: add(self.counted_tx, other.counted_tx)?,
            free_tx: add(self.free_tx, other.free_tx)?,
            signatures_received: add(self.signatures_received, other.signatures_received)?,
            signatures_expected: add(self.signatures_expected, other.signatures_expected)?,
        })
    }
}

/// Intermediate values of one evaluation, for ToyScan and tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactorBreakdown {
    /// Usage `u` in ppm.
    pub usage: i128,
    /// Participation `p` in ppm.
    pub participation: i128,
    /// Performance part `L` (-500,000 .. +500,000).
    pub performance: i128,
    /// Deviation from plan `d` in ppm (positive = ahead of plan).
    pub deviation: i128,
    /// Schedule part `Z`.
    pub schedule: i128,
    /// Target before the step limit.
    pub target: i128,
    /// New factor `m`.
    pub factor: Ppm,
}

fn ovf() -> ArithmeticError {
    ArithmeticError::Overflow
}

fn to_i128(amount: Amount) -> Result<i128, ArithmeticError> {
    i128::try_from(amount.units()).map_err(|_| ovf())
}

fn mul(a: i128, b: i128) -> Result<i128, ArithmeticError> {
    a.checked_mul(b).ok_or_else(ovf)
}

fn add(a: i128, b: i128) -> Result<i128, ArithmeticError> {
    a.checked_add(b).ok_or_else(ovf)
}

fn sub(a: i128, b: i128) -> Result<i128, ArithmeticError> {
    a.checked_sub(b).ok_or_else(ovf)
}

fn div(a: i128, b: i128) -> Result<i128, ArithmeticError> {
    if b == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    a.checked_div_euclid(b).ok_or_else(ovf)
}

/// Computes the next `m` from the previous one, the measurements of the
/// previous epoch, the sum issued so far and the plan value `P(t)`.
///
/// # Errors
/// [`EmissionError::InvalidMeasurements`] if `blocks` or
/// `signatures_expected` is zero, [`EmissionError::FactorOutOfRange`] if
/// `previous` is outside [0.5, 2.0].
pub fn next_factor(
    previous: Ppm,
    measurements: &EpochMeasurements,
    issued: Amount,
    plan: Amount,
) -> Result<FactorBreakdown, EmissionError> {
    if previous < FACTOR_MIN || previous > FACTOR_MAX {
        return Err(EmissionError::FactorOutOfRange);
    }
    if measurements.blocks == 0 || measurements.signatures_expected == 0 {
        return Err(EmissionError::InvalidMeasurements);
    }
    let m_prev = i128::from(previous.get());
    let capacity = mul(
        i128::from(measurements.blocks),
        i128::from(USAGE_CAPACITY_PER_BLOCK),
    )?;
    let usage = div(mul(i128::from(measurements.counted_tx), SCALE)?, capacity)?;
    let participation = div(
        mul(i128::from(measurements.signatures_received), SCALE)?,
        i128::from(measurements.signatures_expected),
    )?;
    let n = SCALE.min(mul(2, usage)?);
    let b = div(mul(sub(participation, 670_000)?, SCALE)?, 280_000)?.clamp(0, SCALE);
    let performance = sub(div(add(n, b)?, 2)?, 500_000)?;

    let issued = to_i128(issued)?;
    let plan = to_i128(plan)?;
    let deviation = if plan > 0 {
        div(mul(sub(issued, plan)?, SCALE)?, plan)?
    } else {
        0
    };
    let schedule = mul(-10, deviation)?.clamp(-500_000, 750_000);
    let target = add(add(SCALE, div(performance, 2)?)?, schedule)?
        .clamp(i128::from(FACTOR_MIN.get()), i128::from(FACTOR_MAX.get()));
    let step = i128::from(FACTOR_MAX_STEP.get());
    let next = add(m_prev, sub(target, m_prev)?.clamp(sub(0, step)?, step))?;
    let factor = Ppm::new(u64::try_from(next).map_err(|_| EmissionError::FactorOutOfRange)?);
    if factor < FACTOR_MIN || factor > FACTOR_MAX {
        return Err(EmissionError::FactorOutOfRange);
    }
    Ok(FactorBreakdown {
        usage,
        participation,
        performance,
        deviation,
        schedule,
        target,
        factor,
    })
}
