use core::fmt;

use toy_crypto::CryptoError;
use toy_state::StateError;
use toy_types::{Amount, ArithmeticError, Height};

use crate::rlp::RlpError;

/// Error of a single transaction or ticket. A block that contains an
/// invalid transaction is invalid as a whole.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TxError {
    /// Malformed or non-canonical RLP.
    Rlp(RlpError),
    /// Signature or key error.
    Crypto(CryptoError),
    /// Checked arithmetic failed.
    Arithmetic(ArithmeticError),
    /// The state rejected the operation.
    State(StateError),
    /// Empty input.
    Empty,
    /// Unknown or unsupported transaction type byte.
    UnsupportedType(u8),
    /// The list has the wrong number of fields.
    WrongFieldCount,
    /// The chain ID in the transaction is not this chain's.
    WrongChainId,
    /// An Ethereum transaction that is not a plain transfer: contract
    /// creation, call data or access list. ToyChain has no contracts
    /// (decision 36).
    NotATransfer,
    /// A value does not fit into ToyChain's 128-bit amounts.
    ValueTooLarge,
    /// Ethereum: the gas limit is below 21,000.
    GasLimitTooLow,
    /// Ethereum: the priority fee is above the maximum fee.
    PriorityAboveMaxFee,
    /// The transaction expired.
    Expired {
        /// Last valid height.
        expiry: Height,
        /// Height of the block.
        height: Height,
    },
    /// The expiry is more than 1,440 blocks ahead.
    ExpiryTooFar,
    /// The memo is longer than 32 bytes.
    MemoTooLong,
    /// The maximum fee does not cover the required fee.
    FeeTooLow {
        /// Required fee.
        required: Amount,
        /// Maximum fee of the transaction.
        max_fee: Amount,
    },
    /// The account is younger than 100 blocks (free quota).
    AccountTooYoung,
    /// The free quota of this epoch is used up.
    QuotaExhausted,
    /// The free quota is locked by a penalty.
    QuotaLocked,
    /// The refund budget of this epoch is used up.
    RefundBudgetExhausted,
    /// Free transactions would use more than 50 % of the block.
    FreeBytesExceeded,
    /// A paid transaction follows a free one in the block.
    PaidAfterFree,
    /// Heavy user (from 1,000 transactions per epoch): only paid
    /// transactions, at least twice the base fee (decision 146).
    HeavyUserMustPay,
    /// Tickets of this address are refused (penalty).
    TicketsLocked,
    /// The ticket was already used in this epoch.
    TicketReused,
    /// The ticket is for another epoch.
    TicketWrongEpoch,
    /// The ticket's work is above the threshold.
    TicketWorkInvalid,
    /// The evidence does not prove a double signature.
    InvalidEvidence,
    /// This double signature was already punished.
    EvidenceAlreadyReported,
}

impl From<RlpError> for TxError {
    fn from(e: RlpError) -> Self {
        Self::Rlp(e)
    }
}

impl From<CryptoError> for TxError {
    fn from(e: CryptoError) -> Self {
        Self::Crypto(e)
    }
}

impl From<ArithmeticError> for TxError {
    fn from(e: ArithmeticError) -> Self {
        Self::Arithmetic(e)
    }
}

impl From<StateError> for TxError {
    fn from(e: StateError) -> Self {
        Self::State(e)
    }
}

impl fmt::Display for TxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rlp(e) => write!(f, "{e}"),
            Self::Crypto(e) => write!(f, "{e}"),
            Self::Arithmetic(e) => write!(f, "{e}"),
            Self::State(e) => write!(f, "{e}"),
            Self::Empty => f.write_str("empty transaction"),
            Self::UnsupportedType(t) => write!(f, "unsupported transaction type 0x{t:02x}"),
            Self::WrongFieldCount => f.write_str("wrong number of fields"),
            Self::WrongChainId => f.write_str("wrong chain ID"),
            Self::NotATransfer => f.write_str("only plain transfers are allowed"),
            Self::ValueTooLarge => f.write_str("value too large"),
            Self::GasLimitTooLow => f.write_str("gas limit below 21000"),
            Self::PriorityAboveMaxFee => f.write_str("priority fee above max fee"),
            Self::Expired { expiry, height } => {
                write!(f, "expired at height {expiry}, block is {height}")
            }
            Self::ExpiryTooFar => f.write_str("expiry more than 1440 blocks ahead"),
            Self::MemoTooLong => f.write_str("memo longer than 32 bytes"),
            Self::FeeTooLow { required, max_fee } => {
                write!(f, "fee {required} required, maximum is {max_fee}")
            }
            Self::AccountTooYoung => f.write_str("account younger than 100 blocks"),
            Self::QuotaExhausted => f.write_str("free quota used up"),
            Self::QuotaLocked => f.write_str("free quota locked by a penalty"),
            Self::RefundBudgetExhausted => f.write_str("refund budget of the epoch used up"),
            Self::FreeBytesExceeded => f.write_str("free transactions exceed 50 % of the block"),
            Self::PaidAfterFree => f.write_str("paid transaction after a free one"),
            Self::HeavyUserMustPay => {
                f.write_str("heavy user must pay at least twice the base fee")
            }
            Self::TicketsLocked => f.write_str("tickets of this address are refused"),
            Self::TicketReused => f.write_str("ticket already used"),
            Self::TicketWrongEpoch => f.write_str("ticket for another epoch"),
            Self::TicketWorkInvalid => f.write_str("ticket work above threshold"),
            Self::InvalidEvidence => f.write_str("evidence does not prove a double signature"),
            Self::EvidenceAlreadyReported => f.write_str("double signature already punished"),
        }
    }
}

impl std::error::Error for TxError {}

/// Why a block is invalid.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BlockError {
    /// Transaction number `index` (0-based) is invalid.
    Transaction {
        /// Position in the block.
        index: usize,
        /// Reason.
        error: TxError,
    },
    /// Ticket number `index` (0-based) is invalid.
    Ticket {
        /// Position in the block.
        index: usize,
        /// Reason.
        error: TxError,
    },
    /// The block is larger than 128 KB.
    TooLarge,
    /// More tickets than allowed for the block gap (32 at 60 seconds,
    /// at least 8; standalone and attached together).
    TooManyTickets {
        /// Allowed number.
        limit: u64,
    },
    /// The state rejected a block-level step (mint, epoch change).
    State(StateError),
    /// Rule I0 failed after the block.
    I0(StateError),
}

impl From<StateError> for BlockError {
    fn from(e: StateError) -> Self {
        Self::State(e)
    }
}

impl From<ArithmeticError> for BlockError {
    fn from(e: ArithmeticError) -> Self {
        Self::State(StateError::Arithmetic(e))
    }
}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transaction { index, error } => write!(f, "transaction {index}: {error}"),
            Self::Ticket { index, error } => write!(f, "ticket {index}: {error}"),
            Self::TooLarge => f.write_str("block larger than 128 KB"),
            Self::TooManyTickets { limit } => write!(f, "more than {limit} tickets"),
            Self::State(e) => write!(f, "{e}"),
            Self::I0(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for BlockError {}
