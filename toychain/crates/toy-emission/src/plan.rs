//! Plan curve `P(t)`: the emission at m = 1 after `t` whole plan days
//! (spec 9.1, decision 136):
//! `P(0) = 0`, `P(i+1) = P(i) + floor(floor((10^26 − P(i)) × k(i) ÷ 10^6) ÷ 10^6)`
//! with `k(i)` from the K table.

use toy_types::params::MAX_SUPPLY;
use toy_types::{Amount, ArithmeticError, Ppm};

use crate::ktable::k_at;
use crate::schedule::emission_for;

/// One plan day: `p + emission(10^26 − p, k(day), m = 1)`.
///
/// # Errors
/// [`ArithmeticError::Underflow`] if `p` exceeds the supply cap.
pub fn plan_step(p: Amount, day: u64) -> Result<Amount, ArithmeticError> {
    p.checked_add(plan_day_amount(p, day)?)
}

/// `P(days)`, computed from scratch.
///
/// # Errors
/// Only on internal arithmetic failure (not expected).
pub fn plan_after_days(days: u64) -> Result<Amount, ArithmeticError> {
    PlanCurve::new().at(days)
}

/// Incremental evaluation of `P(t)` that remembers the last point, so a
/// node does not recompute the whole curve every epoch.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PlanCurve {
    days: u64,
    value: Amount,
}

impl PlanCurve {
    /// Curve at day 0.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            days: 0,
            value: Amount::ZERO,
        }
    }

    /// Returns `P(days)`. Moving forward is incremental; moving backwards
    /// restarts from day 0. Once the daily step is zero the value is final,
    /// so very large `days` return quickly.
    ///
    /// # Errors
    /// Only on internal arithmetic failure (not expected).
    pub fn at(&mut self, days: u64) -> Result<Amount, ArithmeticError> {
        if days < self.days {
            *self = Self::new();
        }
        while self.days < days {
            let next = plan_step(self.value, self.days)?;
            if next == self.value {
                self.days = days;
                break;
            }
            self.value = next;
            self.days = self.days.checked_add(1).ok_or(ArithmeticError::Overflow)?;
        }
        Ok(self.value)
    }
}

/// Plan emission of plan day `day` starting at `P(day)`; equals the
/// emission of an epoch at m = 1 when issued matches the plan.
///
/// # Errors
/// [`ArithmeticError::Underflow`] if `p` exceeds the supply cap.
pub fn plan_day_amount(p: Amount, day: u64) -> Result<Amount, ArithmeticError> {
    emission_for(MAX_SUPPLY.checked_sub(p)?, k_at(day), Ppm::ONE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_day_is_15821_toy() {
        // 10^26 × 158,219,394 × 10^-12 = 15,821.9394 TOY.
        let first = Amount::from_units(15_821_939_400_000_000_000_000);
        assert_eq!(plan_after_days(1), Ok(first));
        assert_eq!(plan_day_amount(Amount::ZERO, 0), Ok(first));
    }

    #[test]
    fn incremental_equals_from_scratch() {
        let mut curve = PlanCurve::new();
        for day in [0, 5, 3, 365, 366, 100, 2_000] {
            assert_eq!(curve.at(day), plan_after_days(day));
        }
    }

    #[test]
    fn huge_day_counts_terminate_below_cap() {
        let mut curve = PlanCurve::new();
        let end = curve.at(u64::MAX).unwrap();
        assert!(end <= MAX_SUPPLY);
        assert_eq!(plan_step(end, u64::MAX), Ok(end));
    }
}
