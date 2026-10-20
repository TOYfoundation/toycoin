use core::fmt;

use toy_types::{Address, Amount, ArithmeticError, EpochNumber, Height};

/// Why an account cannot be closed yet (spec 6.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloseBlocker {
    /// Bonded stake must be unbonded first (21 epochs).
    Stake,
    /// Stake is still unbonding.
    Unbonding,
    /// Bonded start validator rewards are still locked.
    RewardLocks,
    /// Stake is held after a double signature.
    HeldStake,
    /// The account is a registered validator.
    Validator,
}

/// Error of a state operation. On error the state is unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum StateError {
    /// Checked arithmetic failed.
    Arithmetic(ArithmeticError),
    /// The address has no active account.
    UnknownAccount(Address),
    /// Not enough free balance.
    InsufficientFunds {
        /// Needed amount.
        needed: Amount,
        /// Available free balance.
        available: Amount,
    },
    /// Not enough bonded stake.
    InsufficientStake {
        /// Needed amount.
        needed: Amount,
        /// Bonded stake.
        available: Amount,
    },
    /// A first incoming amount to a new address must cover the activation
    /// deposit (spec 6.1).
    BelowActivationDeposit {
        /// Incoming amount.
        amount: Amount,
        /// Current activation deposit.
        required: Amount,
    },
    /// The transaction nonce is not the expected one.
    NonceMismatch {
        /// Expected nonce.
        expected: u64,
        /// Nonce of the transaction.
        found: u64,
    },
    /// The nonce space of the account is exhausted.
    NonceExhausted,
    /// Blocks must be applied in order.
    UnexpectedHeight {
        /// Expected height.
        expected: Height,
        /// Given height.
        found: Height,
    },
    /// Epochs must not go backwards or skip.
    UnexpectedEpoch {
        /// Current epoch.
        current: EpochNumber,
        /// Given epoch.
        found: EpochNumber,
    },
    /// The account cannot be closed yet.
    CannotClose(CloseBlocker),
    /// The remaining balance of a closed account needs a target.
    MissingCloseTarget,
    /// An account cannot be closed to itself.
    CloseTargetIsSelf,
    /// The donation is larger than free balance plus deposit.
    DonationTooLarge,
    /// A validator needs at least the tier 4 stake.
    ValidatorStakeTooLow {
        /// Bonded stake.
        stake: Amount,
        /// Required stake.
        required: Amount,
    },
    /// The account is already a registered validator.
    AlreadyValidator,
    /// The account is not a registered validator.
    NotValidator,
    /// The BLS key is registered for another address.
    BlsKeyInUse,
    /// The role of this address is locked by a penalty.
    RoleLocked,
    /// The activation deposit may only fall, and never below 0.01 TOY.
    InvalidDepositChange,
    /// The parts of a mint or fee split do not add up.
    PartsMismatch,
    /// Start validators must be unique.
    DuplicateStartValidator,
    /// Not enough TOY in a protocol pot.
    PotTooSmall,
    /// Rule I0 failed: the pots do not add up to the minted sum. A block
    /// that causes this is invalid.
    I0Violated {
        /// Sum over all pots.
        pots: Amount,
        /// Sum of all emissions so far.
        minted: Amount,
    },
}

impl From<ArithmeticError> for StateError {
    fn from(e: ArithmeticError) -> Self {
        Self::Arithmetic(e)
    }
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Arithmetic(e) => write!(f, "{e}"),
            Self::UnknownAccount(a) => write!(f, "no active account {a}"),
            Self::InsufficientFunds { needed, available } => {
                write!(f, "needs {needed}, free balance is {available}")
            }
            Self::InsufficientStake { needed, available } => {
                write!(f, "needs {needed} stake, bonded is {available}")
            }
            Self::BelowActivationDeposit { amount, required } => write!(
                f,
                "{amount} is below the activation deposit {required} for a new address"
            ),
            Self::NonceMismatch { expected, found } => {
                write!(f, "nonce {found}, expected {expected}")
            }
            Self::NonceExhausted => f.write_str("nonce exhausted"),
            Self::UnexpectedHeight { expected, found } => {
                write!(f, "block height {found}, expected {expected}")
            }
            Self::UnexpectedEpoch { current, found } => {
                write!(f, "epoch {found} does not follow epoch {current}")
            }
            Self::CannotClose(b) => write!(f, "account cannot be closed yet: {b:?}"),
            Self::MissingCloseTarget => f.write_str("closing needs a target address"),
            Self::CloseTargetIsSelf => f.write_str("an account cannot be closed to itself"),
            Self::DonationTooLarge => f.write_str("donation exceeds the account balance"),
            Self::ValidatorStakeTooLow { stake, required } => {
                write!(f, "validator stake {stake} is below {required}")
            }
            Self::AlreadyValidator => f.write_str("already a validator"),
            Self::NotValidator => f.write_str("not a validator"),
            Self::BlsKeyInUse => f.write_str("BLS key already registered"),
            Self::RoleLocked => f.write_str("role locked by a penalty"),
            Self::InvalidDepositChange => f.write_str("invalid activation deposit change"),
            Self::PartsMismatch => f.write_str("parts do not add up"),
            Self::DuplicateStartValidator => f.write_str("duplicate start validator"),
            Self::PotTooSmall => f.write_str("protocol pot too small"),
            Self::I0Violated { pots, minted } => {
                write!(f, "rule I0 violated: pots {pots} != minted {minted}")
            }
        }
    }
}

impl std::error::Error for StateError {}
