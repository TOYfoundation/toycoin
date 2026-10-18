use core::fmt;

/// Error of a checked arithmetic operation.
///
/// Consensus code never wraps or saturates silently: every overflow,
/// underflow or division by zero is reported and rejects the operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArithmeticError {
    /// The result is larger than the type can hold.
    Overflow,
    /// The result would be negative.
    Underflow,
    /// Division or remainder by zero.
    DivisionByZero,
}

impl fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Overflow => "arithmetic overflow",
            Self::Underflow => "arithmetic underflow",
            Self::DivisionByZero => "division by zero",
        };
        f.write_str(text)
    }
}

impl std::error::Error for ArithmeticError {}
