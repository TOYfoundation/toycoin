//! Plan curve `P(t)`: the emission at m = 1 after `t` whole plan days
//! (spec 9.1): `P(0) = 0`, `P(i+1) = P(i) + floor((10^26 - P(i)) * 505 / 10^6)`.

use toy_types::params::{K0, MAX_SUPPLY};
use toy_types::{Amount, ArithmeticError};

/// One plan day: `p + floor((10^26 - p) * k0)`.
///
/// # Errors
/// [`ArithmeticError::Underflow`] if `p` exceeds the supply cap.
pub fn plan_step(p: Amount) -> Result<Amount, ArithmeticError> {
    let rest = MAX_SUPPLY.checked_sub(p)?;
    p.checked_add(rest.mul_ppm(K0)?)
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
            let next = plan_step(self.value)?;
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

/// Plan emission for one day starting at `P(t)`; equals the emission of an
/// epoch at m = 1 when issued matches the plan.
///
/// # Errors
/// [`ArithmeticError::Underflow`] if `p` exceeds the supply cap.
pub fn plan_day_amount(p: Amount) -> Result<Amount, ArithmeticError> {
    MAX_SUPPLY.checked_sub(p)?.mul_ppm(K0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_day_is_50500_toy() {
        assert_eq!(plan_after_days(1), Ok(Amount::from_toy(50_500)));
        assert_eq!(plan_day_amount(Amount::ZERO), Ok(Amount::from_toy(50_500)));
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
        assert_eq!(plan_step(end), Ok(end));
    }
}
