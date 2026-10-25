//! Transactions of ToyChain (spec sections 7, 8 and 9).
//!
//! - [`transaction`]: both formats (ToyChain after EIP-2718 with one type
//!   byte per kind, Ethereum legacy and EIP-1559 for MetaMask), signatures
//!   with chain ID, memo (32 bytes) and expiry (decisions 84, 106).
//! - [`fees`]: progressive fee with base fee (EIP-1559 style, factor at
//!   most 1,024), split 40/20/40 (staking, proposer, pool), free quota with
//!   water level, heavy users, refund budget and pool cap.
//! - [`ticket`]: ToyWork tickets (Argon2id, 16 MiB), bound to address,
//!   epoch and seed, signed by the owner.
//! - [`lottery`]: work lottery with luck levels, the jackpots Funke, Welle
//!   and Gezeiten, randomness mix and difficulty.
//! - [`evidence`]: proof of a double signature.
//! - [`block`]: executes a block all or nothing and checks rule I0.
//!
//! There is no contract execution: every function is a fixed kind of
//! transaction (decision 36).

pub mod block;
mod error;
pub mod ethereum;
pub mod evidence;
pub mod fees;
pub mod lottery;
pub mod rlp;
pub mod ticket;
pub mod transaction;

pub use block::{Block, BlockContext, BlockReceipt, JackpotPayout, execute_block};
pub use error::{BlockError, TxError};
pub use lottery::{Jackpot, Trigger};
pub use transaction::{
    Action, Transaction, TxFields, TxFormat, decode_transaction, sign_transaction,
};
