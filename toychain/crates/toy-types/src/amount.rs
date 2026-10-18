use core::fmt;

use crate::error::ArithmeticError;
use crate::params::{DECIMALS, UNITS_PER_TOY};
use crate::ppm::Ppm;

/// An amount of TOY in smallest units (1 TOY = 10^18 units).
///
/// Amounts are always integers. The type offers only checked arithmetic:
/// every operation returns an [`ArithmeticError`] instead of wrapping.
/// Values above the supply cap are representable (e.g. intermediate
/// results), but protocol code checks them against
/// [`crate::params::MAX_SUPPLY`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Amount(u128);

impl Amount {
    /// Zero.
    pub const ZERO: Self = Self(0);

    /// Creates an amount from smallest units.
    #[must_use]
    pub const fn from_units(units: u128) -> Self {
        Self(units)
    }

    /// Creates an amount of whole TOY. Cannot overflow: `u64::MAX * 10^18`
    /// is far below `u128::MAX`.
    #[must_use]
    pub const fn from_toy(whole: u64) -> Self {
        match (whole as u128).checked_mul(UNITS_PER_TOY) {
            Some(units) => Self(units),
            // Unreachable for any u64 input; kept total without panicking.
            None => Self(u128::MAX),
        }
    }

    /// Returns the value in smallest units.
    #[must_use]
    pub const fn units(self) -> u128 {
        self.0
    }

    /// `true` if the amount is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// `self + rhs`.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] if the sum does not fit.
    pub const fn checked_add(self, rhs: Self) -> Result<Self, ArithmeticError> {
        match self.0.checked_add(rhs.0) {
            Some(v) => Ok(Self(v)),
            None => Err(ArithmeticError::Overflow),
        }
    }

    /// `self - rhs`.
    ///
    /// # Errors
    /// [`ArithmeticError::Underflow`] if `rhs > self`.
    pub const fn checked_sub(self, rhs: Self) -> Result<Self, ArithmeticError> {
        match self.0.checked_sub(rhs.0) {
            Some(v) => Ok(Self(v)),
            None => Err(ArithmeticError::Underflow),
        }
    }

    /// `self * factor`.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] if the product does not fit.
    pub const fn checked_mul(self, factor: u128) -> Result<Self, ArithmeticError> {
        match self.0.checked_mul(factor) {
            Some(v) => Ok(Self(v)),
            None => Err(ArithmeticError::Overflow),
        }
    }

    /// `floor(self / divisor)`.
    ///
    /// # Errors
    /// [`ArithmeticError::DivisionByZero`] if `divisor == 0`.
    pub const fn checked_div(self, divisor: u128) -> Result<Self, ArithmeticError> {
        match self.0.checked_div(divisor) {
            Some(v) => Ok(Self(v)),
            None => Err(ArithmeticError::DivisionByZero),
        }
    }

    /// `floor(self * numerator / denominator)`, the basic rounding rule of
    /// the protocol (always towards zero for non-negative values).
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] if `self * numerator` does not fit,
    /// [`ArithmeticError::DivisionByZero`] if `denominator == 0`.
    pub const fn mul_div_floor(
        self,
        numerator: u128,
        denominator: u128,
    ) -> Result<Self, ArithmeticError> {
        let product = match self.0.checked_mul(numerator) {
            Some(v) => v,
            None => return Err(ArithmeticError::Overflow),
        };
        match product.checked_div(denominator) {
            Some(v) => Ok(Self(v)),
            None => Err(ArithmeticError::DivisionByZero),
        }
    }

    /// `floor(self * share)` for a factor in parts per million.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] if the intermediate product does not fit.
    pub const fn mul_ppm(self, share: Ppm) -> Result<Self, ArithmeticError> {
        self.mul_div_floor(share.get() as u128, Ppm::SCALE as u128)
    }

    /// Splits the amount into `parts` equal shares, rounded down.
    /// Returns `(each, remainder)` with `each * parts + remainder == self`.
    ///
    /// # Errors
    /// [`ArithmeticError::DivisionByZero`] if `parts == 0`.
    pub const fn split_even(self, parts: u128) -> Result<(Self, Self), ArithmeticError> {
        let each = match self.0.checked_div(parts) {
            Some(v) => v,
            None => return Err(ArithmeticError::DivisionByZero),
        };
        let remainder = match self.0.checked_rem(parts) {
            Some(v) => v,
            None => return Err(ArithmeticError::DivisionByZero),
        };
        Ok((Self(each), Self(remainder)))
    }

    /// Sums amounts with overflow check.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] if the sum does not fit.
    pub fn checked_sum<I: IntoIterator<Item = Self>>(items: I) -> Result<Self, ArithmeticError> {
        items
            .into_iter()
            .try_fold(Self::ZERO, |acc, item| acc.checked_add(item))
    }
}

impl fmt::Display for Amount {
    /// Formats as TOY with up to 18 decimals and no trailing zeros,
    /// e.g. `50500 TOY` or `0.1 TOY`. Integer arithmetic only.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let whole = self.0.checked_div(UNITS_PER_TOY).unwrap_or(0);
        let frac = self.0.checked_rem(UNITS_PER_TOY).unwrap_or(0);
        if frac == 0 {
            return write!(f, "{whole} TOY");
        }
        let width = DECIMALS as usize;
        let digits = format!("{frac:0width$}");
        write!(f, "{whole}.{} TOY", digits.trim_end_matches('0'))
    }
}

#[cfg(test)]
#[allow(clippy::arithmetic_side_effects)]
mod tests {
    use super::*;
    use crate::params::MAX_SUPPLY;

    #[test]
    fn supply_cap_is_10_pow_26_and_fits_u128() {
        assert_eq!(MAX_SUPPLY.units(), 10u128.pow(26));
        assert_eq!(MAX_SUPPLY, Amount::from_toy(100_000_000));
        // Largest intermediate of the emission formula (spec 9.1): ~10^35.
        assert!(MAX_SUPPLY.units().checked_mul(505 * 2_000_000).is_some());
    }

    #[test]
    fn checked_operations_report_errors() {
        let max = Amount::from_units(u128::MAX);
        assert_eq!(
            max.checked_add(Amount::from_units(1)),
            Err(ArithmeticError::Overflow)
        );
        assert_eq!(
            Amount::ZERO.checked_sub(Amount::from_units(1)),
            Err(ArithmeticError::Underflow)
        );
        assert_eq!(max.checked_mul(2), Err(ArithmeticError::Overflow));
        assert_eq!(
            Amount::from_toy(1).checked_div(0),
            Err(ArithmeticError::DivisionByZero)
        );
        assert_eq!(
            Amount::from_toy(1).mul_div_floor(1, 0),
            Err(ArithmeticError::DivisionByZero)
        );
        assert_eq!(max.mul_ppm(Ppm::new(2)), Err(ArithmeticError::Overflow));
        assert_eq!(
            Amount::from_toy(1).split_even(0),
            Err(ArithmeticError::DivisionByZero)
        );
    }

    #[test]
    fn rounding_is_floor() {
        let seven = Amount::from_units(7);
        assert_eq!(seven.mul_div_floor(1, 2), Ok(Amount::from_units(3)));
        assert_eq!(
            seven.split_even(3),
            Ok((Amount::from_units(2), Amount::from_units(1)))
        );
        assert_eq!(
            Amount::from_units(999_999).mul_ppm(Ppm::new(1)),
            Ok(Amount::ZERO)
        );
    }

    #[test]
    fn display_uses_integer_formatting() {
        assert_eq!(Amount::from_toy(50_500).to_string(), "50500 TOY");
        assert_eq!(
            Amount::from_units(100_000_000_000_000_000).to_string(),
            "0.1 TOY"
        );
        assert_eq!(
            Amount::from_units(1).to_string(),
            "0.000000000000000001 TOY"
        );
        assert_eq!(Amount::ZERO.to_string(), "0 TOY");
    }

    #[test]
    fn checked_sum() {
        let parts = [Amount::from_toy(1), Amount::from_toy(2)];
        assert_eq!(Amount::checked_sum(parts), Ok(Amount::from_toy(3)));
        let too_big = [Amount::from_units(u128::MAX), Amount::from_units(1)];
        assert_eq!(Amount::checked_sum(too_big), Err(ArithmeticError::Overflow));
    }
}
