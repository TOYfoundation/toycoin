//! Minting per block (spec 9.0 and 9.2, decisions 107, 138, 139, 149).
//!
//! The emission `E` of an epoch is spread over its first 1,440 blocks:
//! every block `floor(E / 1440)`, block 1,440 additionally the remainder.
//! Each block amount `T` is split with `floor`:
//!
//! | part | share of `T` |
//! |---|---|
//! | work reward | `w(t)` from the K table (45 % → 25 %), split 85 / 8 / 5 / 2 % into base pot, Funke, Welle and Gezeiten |
//! | start wage | `a(d)` after the cap, bootstrap phase only |
//! | committee pot | 5 % (bootstrap phase: to the start validators) |
//! | full-node pot | 8 % |
//! | refund pool | 2 % plus every rounding remainder |
//! | staking pot | the rest |
//!
//! Exactly `E` is minted per epoch, never more and never less.

use toy_types::params::{
    COMMITTEE_SHARE, EMISSION_BLOCKS_PER_EPOCH, FULL_NODE_SHARE, JACKPOT_FUNKE_SHARE,
    JACKPOT_GEZEITEN_SHARE, JACKPOT_WELLE_SHARE, LOTTERY_BASE_SHARE, REFUND_POOL_SHARE,
    START_VALIDATORS,
};
use toy_types::{Amount, ArithmeticError, Ppm};

use crate::error::EmissionError;
use crate::schedule::{EpochEmission, Phase};

/// Work reward of a block, split into the base pot and the three jackpots.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WorkMint {
    /// Base pot, paid to the tickets of a block (spec 9.3).
    pub base: Amount,
    /// Jackpot "Funke" (about every 4 hours).
    pub funke: Amount,
    /// Jackpot "Welle" (about every 5 days).
    pub welle: Amount,
    /// Jackpot "Gezeiten" (about every 60 days).
    pub gezeiten: Amount,
}

impl WorkMint {
    /// Sum of the four parts.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] on overflow (not expected).
    pub fn total(&self) -> Result<Amount, ArithmeticError> {
        Amount::checked_sum([self.base, self.funke, self.welle, self.gezeiten])
    }
}

/// Newly minted TOY of one block and where they go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockMint {
    /// Total minted in this block.
    pub total: Amount,
    /// Bootstrap phase: amount for each start validator (start wage plus
    /// committee pot), locked for 365 epochs by the state layer. Zero in
    /// the normal phase.
    pub start_each: Amount,
    /// Number of start validators the amount is paid to.
    pub start_count: u32,
    /// Work reward (base pot and jackpots).
    pub work: WorkMint,
    /// Committee pot (normal phase; distributed by consensus, M4).
    pub committee: Amount,
    /// Full-node pot (tiers 4 to 7).
    pub full_node: Amount,
    /// Refund pool, including all rounding remainders.
    pub refund_pool: Amount,
    /// Staking pot (tiers 1 to 7, spec 9.4).
    pub staking: Amount,
}

impl BlockMint {
    /// A block without emission.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            total: Amount::ZERO,
            start_each: Amount::ZERO,
            start_count: START_VALIDATORS,
            work: WorkMint {
                base: Amount::ZERO,
                funke: Amount::ZERO,
                welle: Amount::ZERO,
                gezeiten: Amount::ZERO,
            },
            committee: Amount::ZERO,
            full_node: Amount::ZERO,
            refund_pool: Amount::ZERO,
            staking: Amount::ZERO,
        }
    }

    /// Sum paid to the start validators.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] on overflow (not expected).
    pub fn start_total(&self) -> Result<Amount, ArithmeticError> {
        self.start_each.checked_mul(u128::from(self.start_count))
    }

    /// Sum of all parts; equals `total` for every mint produced here.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] on overflow (not expected).
    pub fn sum_of_parts(&self) -> Result<Amount, ArithmeticError> {
        Amount::checked_sum([
            self.start_total()?,
            self.work.total()?,
            self.committee,
            self.full_node,
            self.refund_pool,
            self.staking,
        ])
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

fn add_ppm(parts: &[Ppm]) -> Result<u64, ArithmeticError> {
    parts.iter().try_fold(0u64, |acc, p| {
        acc.checked_add(p.get()).ok_or(ArithmeticError::Overflow)
    })
}

/// Takes shares of an amount without ever taking more than is left. Each
/// part is `floor(amount × share)`, at most the rest; the rest stays for
/// the refund pool. Mathematically the sum of the floors never exceeds the
/// amount, so the cap only makes that fact visible to the type checker and
/// the proof tool: the parts always add up, there is no error path.
pub(crate) struct Splitter {
    amount: Amount,
    pub(crate) rest: Amount,
}

impl Splitter {
    pub(crate) const fn new(amount: Amount) -> Self {
        Self {
            amount,
            rest: amount,
        }
    }

    /// Takes `floor(amount × share)`, at most the rest.
    pub(crate) fn take(&mut self, share: Ppm) -> Result<Amount, EmissionError> {
        let wanted = self.amount.mul_ppm(share)?;
        self.give(wanted)
    }

    /// Takes `wanted`, at most the rest. Afterwards `part + rest` equals
    /// the rest before (proven by Kani, `verification.rs`).
    pub(crate) fn give(&mut self, wanted: Amount) -> Result<Amount, EmissionError> {
        let part = wanted.min(self.rest);
        self.rest = self
            .rest
            .checked_sub(part)
            .map_err(|_| EmissionError::Inconsistent)?;
        Ok(part)
    }
}

/// Mint of block number `index` (1-based) within the epoch.
///
/// # Errors
/// [`EmissionError::InvalidBlockIndex`] for `index == 0`,
/// [`EmissionError::Inconsistent`] if the shares exceed 100 % or the parts
/// would not add up.
pub fn block_mint(epoch: &EpochEmission, index: u64) -> Result<BlockMint, EmissionError> {
    let total = block_emission(epoch.emission, index)?;
    let wage_share = match epoch.phase {
        Phase::Bootstrap => epoch.start_validator_share,
        Phase::Normal => Ppm::ZERO,
    };
    let fixed = add_ppm(&[
        epoch.work_share,
        wage_share,
        COMMITTEE_SHARE,
        FULL_NODE_SHARE,
        REFUND_POOL_SHARE,
    ])?;
    let staking_share = Ppm::SCALE
        .checked_sub(fixed)
        .ok_or(EmissionError::Inconsistent)?;

    let mut parts = Splitter::new(total);
    let work_total = parts.take(epoch.work_share)?;
    let wage = parts.take(wage_share)?;
    let committee_total = parts.take(COMMITTEE_SHARE)?;
    let full_node = parts.take(FULL_NODE_SHARE)?;
    let pool_share = parts.take(REFUND_POOL_SHARE)?;
    let staking = parts.take(Ppm::new(staking_share))?;

    let mut work_parts = Splitter::new(work_total);
    let work = WorkMint {
        base: work_parts.take(LOTTERY_BASE_SHARE)?,
        funke: work_parts.take(JACKPOT_FUNKE_SHARE)?,
        welle: work_parts.take(JACKPOT_WELLE_SHARE)?,
        gezeiten: work_parts.take(JACKPOT_GEZEITEN_SHARE)?,
    };

    let (start_each, start_rest, committee) = match epoch.phase {
        Phase::Bootstrap => {
            let paid = wage.checked_add(committee_total)?;
            let (each, rest) = paid.split_even(u128::from(START_VALIDATORS))?;
            (each, rest, Amount::ZERO)
        }
        Phase::Normal => (Amount::ZERO, wage, committee_total),
    };
    // Every rounding remainder goes to the pool.
    let refund_pool = Amount::checked_sum([pool_share, parts.rest, work_parts.rest, start_rest])?;
    let mint = BlockMint {
        total,
        start_each,
        start_count: START_VALIDATORS,
        work,
        committee,
        full_node,
        refund_pool,
        staking,
    };
    if mint.sum_of_parts()? != total {
        return Err(EmissionError::Inconsistent);
    }
    Ok(mint)
}
