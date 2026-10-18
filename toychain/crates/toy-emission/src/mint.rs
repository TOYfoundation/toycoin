//! Minting per block (spec 9.0, decision 107).
//!
//! The emission `E` of an epoch is spread over its first 1,440 blocks:
//! every block `floor(E / 1440)`, block 1,440 additionally the remainder.
//! Each block amount is split with `floor` into consensus (50 %), work
//! lottery (40 %) and refund pool (10 % plus all rounding remainders).
//! Exactly `E` is minted per epoch, never more and never less.

use toy_types::params::{CONSENSUS_SHARE, EMISSION_BLOCKS_PER_EPOCH, START_VALIDATORS, WORK_SHARE};
use toy_types::{Amount, ArithmeticError};

use crate::error::EmissionError;
use crate::schedule::{EpochEmission, Phase};

/// Recipient of the consensus share of a block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsensusPayout {
    /// Bootstrap phase: each start validator receives `each`. The amount
    /// is bonded for 365 epochs (handled by the state layer, M3/M4).
    StartValidators {
        /// Amount per start validator.
        each: Amount,
        /// Number of start validators.
        count: u32,
    },
    /// Normal phase: pot for stakers, distributed by the consensus layer
    /// (base share per committee member plus linear part, decision 112).
    Stakers(Amount),
}

impl ConsensusPayout {
    /// Total amount of the payout.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] on overflow (not expected).
    pub fn total(&self) -> Result<Amount, ArithmeticError> {
        match *self {
            Self::StartValidators { each, count } => each.checked_mul(u128::from(count)),
            Self::Stakers(amount) => Ok(amount),
        }
    }
}

/// Newly minted TOY of one block and where they go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockMint {
    /// Total minted in this block.
    pub total: Amount,
    /// Consensus share.
    pub consensus: ConsensusPayout,
    /// Work lottery pot (includes the unpaid part of the bootstrap share).
    pub work_lottery: Amount,
    /// Refund pool (includes all rounding remainders).
    pub refund_pool: Amount,
}

impl BlockMint {
    /// A block without emission.
    #[must_use]
    pub const fn empty(phase: Phase) -> Self {
        let consensus = match phase {
            Phase::Bootstrap => ConsensusPayout::StartValidators {
                each: Amount::ZERO,
                count: START_VALIDATORS,
            },
            Phase::Normal => ConsensusPayout::Stakers(Amount::ZERO),
        };
        Self {
            total: Amount::ZERO,
            consensus,
            work_lottery: Amount::ZERO,
            refund_pool: Amount::ZERO,
        }
    }

    /// Sum of all parts; equals `total` for every mint produced here.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] on overflow (not expected).
    pub fn sum_of_parts(&self) -> Result<Amount, ArithmeticError> {
        Amount::checked_sum([self.consensus.total()?, self.work_lottery, self.refund_pool])
    }
}

/// Emission of block number `index` (1-based) within an epoch.
///
/// # Errors
/// [`EmissionError::InvalidBlockIndex`] for `index == 0`.
pub fn block_emission(epoch_emission: Amount, index: u64) -> Result<Amount, EmissionError> {
    if index == 0 {
        return Err(EmissionError::InvalidBlockIndex);
    }
    if index > EMISSION_BLOCKS_PER_EPOCH {
        return Ok(Amount::ZERO);
    }
    let (each, remainder) = epoch_emission.split_even(u128::from(EMISSION_BLOCKS_PER_EPOCH))?;
    if index == EMISSION_BLOCKS_PER_EPOCH {
        Ok(each.checked_add(remainder)?)
    } else {
        Ok(each)
    }
}

/// Mint of block number `index` (1-based) within the epoch.
///
/// # Errors
/// [`EmissionError::InvalidBlockIndex`] for `index == 0`,
/// [`EmissionError::Inconsistent`] if the parts would not add up.
pub fn block_mint(epoch: &EpochEmission, index: u64) -> Result<BlockMint, EmissionError> {
    let total = block_emission(epoch.emission, index)?;
    let consensus = total.mul_ppm(CONSENSUS_SHARE)?;
    let work = total.mul_ppm(WORK_SHARE)?;
    let pool = total.checked_sub(consensus)?.checked_sub(work)?;

    let mint = match epoch.phase {
        Phase::Normal => BlockMint {
            total,
            consensus: ConsensusPayout::Stakers(consensus),
            work_lottery: work,
            refund_pool: pool,
        },
        Phase::Bootstrap => {
            let paid = total.mul_ppm(epoch.start_validator_share)?;
            if paid > consensus {
                return Err(EmissionError::Inconsistent);
            }
            let (each, remainder) = paid.split_even(u128::from(START_VALIDATORS))?;
            BlockMint {
                total,
                consensus: ConsensusPayout::StartValidators {
                    each,
                    count: START_VALIDATORS,
                },
                work_lottery: work.checked_add(consensus.checked_sub(paid)?)?,
                refund_pool: pool.checked_add(remainder)?,
            }
        }
    };
    if mint.sum_of_parts()? != total {
        return Err(EmissionError::Inconsistent);
    }
    Ok(mint)
}
