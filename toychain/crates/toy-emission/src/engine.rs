//! Block-by-block driver of the emission rules.

use toy_types::{Amount, ArithmeticError, EpochNumber, Height, UnixSeconds};

use crate::clock::{EpochClock, PlanClock};
use crate::error::EmissionError;
use crate::factor::EpochMeasurements;
use crate::mint::{BlockMint, block_mint};
use crate::schedule::{EmissionState, EpochEmission, EpochStart, Phase};

/// What the emission rules need from a block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockInput {
    /// Block timestamp in seconds.
    pub timestamp: UnixSeconds,
    /// Paid or work-backed transactions in the block.
    pub counted_tx: u64,
    /// Free-quota transactions (never influence `m`).
    pub free_tx: u64,
    /// Committee signatures received for the block.
    pub signatures_received: u64,
    /// Committee signatures expected for the block (committee size).
    pub signatures_expected: u64,
    /// Phase for the epoch this block starts (ignored for other blocks).
    pub phase: Phase,
}

/// Result of applying a block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockOutcome {
    /// Height of the block.
    pub height: Height,
    /// Epoch of the block.
    pub epoch: EpochNumber,
    /// 1-based position within the epoch.
    pub index_in_epoch: u64,
    /// Newly minted TOY.
    pub mint: BlockMint,
    /// `true` if this block ends its epoch.
    pub ends_epoch: bool,
}

/// Applies the emission rules block by block, starting at genesis.
///
/// Invariant (part of rule I0): the minted sum never exceeds the issued
/// sum, and at the end of every epoch both are equal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmissionEngine {
    state: EmissionState,
    plan_clock: PlanClock,
    epoch_clock: EpochClock,
    current: Option<EpochEmission>,
    measurements: EpochMeasurements,
    height: Height,
    last_timestamp: UnixSeconds,
    index_in_epoch: u64,
    needs_new_epoch: bool,
    minted: Amount,
}

impl EmissionEngine {
    /// Starts at the genesis block: height 0, epoch 0, no balances and no
    /// emission. The emission of epoch 0 is fixed with block 1.
    #[must_use]
    pub const fn genesis(genesis_timestamp: UnixSeconds) -> Self {
        Self {
            state: EmissionState::genesis(),
            plan_clock: PlanClock::new(),
            epoch_clock: EpochClock::start(genesis_timestamp),
            current: None,
            measurements: EpochMeasurements {
                blocks: 0,
                counted_tx: 0,
                free_tx: 0,
                signatures_received: 0,
                signatures_expected: 0,
            },
            height: 0,
            last_timestamp: genesis_timestamp,
            index_in_epoch: 0,
            needs_new_epoch: true,
            minted: Amount::ZERO,
        }
    }

    /// Applies the next block. On error the engine is unchanged.
    ///
    /// # Errors
    /// [`EmissionError::BlockTooEarly`] for timestamps less than 10 s after
    /// the parent, and any error of [`EmissionState::start_epoch`].
    pub fn apply_block(&mut self, input: &BlockInput) -> Result<BlockOutcome, EmissionError> {
        let mut next = self.clone();
        let outcome = next.apply_in_place(input)?;
        *self = next;
        Ok(outcome)
    }

    fn apply_in_place(&mut self, input: &BlockInput) -> Result<BlockOutcome, EmissionError> {
        if self.needs_new_epoch {
            let epoch = self.state.next_epoch();
            // Plan time up to the last block of the previous epoch.
            let start = EpochStart {
                epoch,
                plan_days: self.plan_clock.full_days(),
                measurements: (epoch > 0).then_some(self.measurements),
                phase: input.phase,
            };
            self.current = Some(self.state.start_epoch(&start)?);
            if epoch > 0 {
                self.epoch_clock = EpochClock::start(input.timestamp);
            }
            self.measurements = EpochMeasurements::default();
            self.index_in_epoch = 0;
            self.needs_new_epoch = false;
        }
        let current = self.current.ok_or(EmissionError::Inconsistent)?;
        self.plan_clock
            .record(self.last_timestamp, input.timestamp)?;
        self.index_in_epoch = self
            .index_in_epoch
            .checked_add(1)
            .ok_or(ArithmeticError::Overflow)?;
        let mint = block_mint(&current, self.index_in_epoch)?;
        self.minted = self.minted.checked_add(mint.total)?;
        if self.minted > self.state.issued() {
            return Err(EmissionError::Inconsistent);
        }
        self.measurements = self.measurements.checked_add(&EpochMeasurements {
            blocks: 1,
            counted_tx: input.counted_tx,
            free_tx: input.free_tx,
            signatures_received: input.signatures_received,
            signatures_expected: input.signatures_expected,
        })?;
        let ends_epoch = self.epoch_clock.record(input.timestamp)?;
        if ends_epoch && self.minted != self.state.issued() {
            return Err(EmissionError::Inconsistent);
        }
        self.needs_new_epoch = ends_epoch;
        self.height = self
            .height
            .checked_add(1)
            .ok_or(ArithmeticError::Overflow)?;
        self.last_timestamp = input.timestamp;
        Ok(BlockOutcome {
            height: self.height,
            epoch: current.epoch,
            index_in_epoch: self.index_in_epoch,
            mint,
            ends_epoch,
        })
    }

    /// Emission state (issued sum, factor, next epoch).
    #[must_use]
    pub const fn state(&self) -> &EmissionState {
        &self.state
    }

    /// Emission of the running epoch (`None` before block 1).
    #[must_use]
    pub const fn current_epoch(&self) -> Option<&EpochEmission> {
        self.current.as_ref()
    }

    /// Sum minted in all blocks so far.
    #[must_use]
    pub const fn minted(&self) -> Amount {
        self.minted
    }

    /// Height of the last applied block.
    #[must_use]
    pub const fn height(&self) -> Height {
        self.height
    }

    /// Plan time since genesis.
    #[must_use]
    pub const fn plan_clock(&self) -> &PlanClock {
        &self.plan_clock
    }
}
