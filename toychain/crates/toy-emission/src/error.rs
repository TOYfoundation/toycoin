use core::fmt;

use toy_types::ArithmeticError;

/// Errors of the emission rules. Any error rejects the block or epoch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmissionError {
    /// A checked arithmetic operation failed.
    Arithmetic(ArithmeticError),
    /// The emission would exceed the remaining supply (never expected).
    SupplyCapExceeded,
    /// An epoch was started out of order.
    UnexpectedEpoch {
        /// Epoch the state expects next.
        expected: u64,
        /// Epoch that was given.
        got: u64,
    },
    /// Plan time of a new epoch is smaller than that of the previous one.
    PlanTimeWentBackwards,
    /// An epoch after epoch 0 was started without measurements.
    MissingMeasurements,
    /// Measurements with zero blocks or zero expected signatures.
    InvalidMeasurements,
    /// The current factor m is outside [0.5, 2.0].
    FactorOutOfRange,
    /// The bootstrap phase cannot start again after the handover.
    PhaseRegression,
    /// A block index within the epoch must start at 1.
    InvalidBlockIndex,
    /// Block timestamp less than 10 seconds after its parent.
    BlockTooEarly,
    /// Internal consistency check failed (never expected).
    Inconsistent,
}

impl From<ArithmeticError> for EmissionError {
    fn from(err: ArithmeticError) -> Self {
        Self::Arithmetic(err)
    }
}

impl fmt::Display for EmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Arithmetic(err) => write!(f, "emission arithmetic: {err}"),
            Self::SupplyCapExceeded => f.write_str("emission would exceed the supply cap"),
            Self::UnexpectedEpoch { expected, got } => {
                write!(f, "expected epoch {expected}, got {got}")
            }
            Self::PlanTimeWentBackwards => f.write_str("plan time went backwards"),
            Self::MissingMeasurements => f.write_str("measurements of the previous epoch missing"),
            Self::InvalidMeasurements => {
                f.write_str("measurements need blocks and expected signatures")
            }
            Self::FactorOutOfRange => f.write_str("emission factor outside [0.5, 2.0]"),
            Self::PhaseRegression => f.write_str("bootstrap phase cannot restart"),
            Self::InvalidBlockIndex => f.write_str("block index in epoch starts at 1"),
            Self::BlockTooEarly => f.write_str("block less than 10 s after its parent"),
            Self::Inconsistent => f.write_str("internal emission consistency check failed"),
        }
    }
}

impl std::error::Error for EmissionError {}
