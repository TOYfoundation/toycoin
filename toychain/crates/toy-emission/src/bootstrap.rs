//! Reward of the start validators in the bootstrap phase (spec 5.3,
//! decisions 70, 78 and 110).
//!
//! Share of the emission: `a(d) = 500,000 * max(0, 180 - d) / 180` ppm,
//! with `d` the full plan days at the start of the epoch. Cap: the reward
//! of an epoch is at most `a(d)` times the plan emission (m = 1) of the plan
//! days since the previous epoch. Both rules together mean that faster
//! emission (m > 1), slow blocks or long epochs never raise the reward.

use toy_types::params::{START_REWARD_DAYS, START_REWARD_INITIAL_SHARE};
use toy_types::{Amount, ArithmeticError, Ppm};

/// `a(d)` in ppm of the emission, before the cap.
///
/// # Errors
/// Only on internal arithmetic failure (not expected).
pub fn start_share(plan_days: u64) -> Result<Ppm, ArithmeticError> {
    let Some(left) = START_REWARD_DAYS.checked_sub(plan_days) else {
        return Ok(Ppm::ZERO);
    };
    let scaled = START_REWARD_INITIAL_SHARE
        .get()
        .checked_mul(left)
        .ok_or(ArithmeticError::Overflow)?;
    Ok(Ppm::new(
        scaled
            .checked_div(START_REWARD_DAYS)
            .ok_or(ArithmeticError::DivisionByZero)?,
    ))
}

/// Share actually paid in an epoch: `min(a(d), floor(a(d) * plan_delta / emission))`.
///
/// `plan_delta` is the plan emission of the days this epoch covers,
/// `P(t + 1) - P(t_prev + 1)` (first epoch: `P(1) - P(0)`). Paying
/// `floor(block_emission * share)` in every block then never exceeds
/// `a(d) * plan_delta` for the epoch.
///
/// # Errors
/// Only on internal arithmetic failure (not expected).
pub fn effective_share(
    plan_days: u64,
    emission: Amount,
    plan_delta: Amount,
) -> Result<Ppm, ArithmeticError> {
    let share = start_share(plan_days)?;
    if share == Ppm::ZERO || emission.is_zero() {
        return Ok(Ppm::ZERO);
    }
    let capped = plan_delta
        .units()
        .checked_mul(u128::from(share.get()))
        .ok_or(ArithmeticError::Overflow)?
        .checked_div(emission.units())
        .ok_or(ArithmeticError::DivisionByZero)?;
    let capped = u64::try_from(capped).unwrap_or(u64::MAX);
    Ok(Ppm::new(share.get().min(capped)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn share_falls_linearly_to_zero() {
        assert_eq!(start_share(0), Ok(Ppm::new(500_000)));
        assert_eq!(start_share(90), Ok(Ppm::new(250_000)));
        assert_eq!(start_share(179), Ok(Ppm::new(2_777)));
        assert_eq!(start_share(180), Ok(Ppm::ZERO));
        assert_eq!(start_share(u64::MAX), Ok(Ppm::ZERO));
    }

    #[test]
    fn cap_limits_share_when_emission_is_larger_than_plan() {
        let plan = Amount::from_toy(50_500);
        // m = 2: emission twice the plan day, share halves.
        let doubled = Amount::from_toy(101_000);
        assert_eq!(effective_share(0, doubled, plan), Ok(Ppm::new(250_000)));
        // m = 1: no cap.
        assert_eq!(effective_share(0, plan, plan), Ok(Ppm::new(500_000)));
        // No plan day passed: nothing.
        assert_eq!(effective_share(0, plan, Amount::ZERO), Ok(Ppm::ZERO));
        // No emission: nothing.
        assert_eq!(effective_share(0, Amount::ZERO, plan), Ok(Ppm::ZERO));
    }
}
