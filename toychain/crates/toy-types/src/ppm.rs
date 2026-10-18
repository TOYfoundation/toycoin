use core::fmt;

/// A non-negative fixed-point factor in parts per million.
///
/// `Ppm::ONE` (1,000,000) stands for 1.0. Used for the emission factor `m`,
/// for reward shares and for the participation and usage measurements
/// (spec section 9.1).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ppm(u64);

impl Ppm {
    /// 0.0
    pub const ZERO: Self = Self(0);
    /// 1.0
    pub const ONE: Self = Self(1_000_000);
    /// Denominator of every `Ppm` value.
    pub const SCALE: u64 = 1_000_000;

    /// Creates a factor from its value in millionths.
    #[must_use]
    pub const fn new(millionths: u64) -> Self {
        Self(millionths)
    }

    /// Returns the value in millionths.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for Ppm {
    /// Formats as a decimal number with six places, e.g. `1.050000`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let whole = self.0.checked_div(Self::SCALE).unwrap_or(0);
        let frac = self.0.checked_rem(Self::SCALE).unwrap_or(0);
        write!(f, "{whole}.{frac:06}")
    }
}
