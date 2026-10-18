//! Time rules (spec 5.2 and 5.3, decisions 58 and 110).
//!
//! - **Plan time:** sum of all block gaps, each counted at most 600 s.
//!   Long pauses or deliberately slow blocks cannot make it run ahead.
//! - **Epoch end:** the first block with at least 1,440 blocks in the epoch
//!   and a timestamp at least 86,400 s after the epoch's first block. The
//!   genesis block is not counted as a block of epoch 0, but its timestamp
//!   is the start of epoch 0.

use toy_types::UnixSeconds;
use toy_types::params::{
    EPOCH_MIN_BLOCKS, EPOCH_MIN_SECONDS, MIN_BLOCK_GAP_SECONDS, PLAN_GAP_CAP_SECONDS,
    SECONDS_PER_DAY,
};

use crate::error::EmissionError;
use toy_types::ArithmeticError;

/// Accumulated plan time since genesis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlanClock {
    seconds: u64,
}

/// Checks the minimum gap and returns the gap in seconds.
///
/// # Errors
/// [`EmissionError::BlockTooEarly`] if `timestamp < parent + 10`.
pub fn block_gap(parent: UnixSeconds, timestamp: UnixSeconds) -> Result<u64, EmissionError> {
    let gap = timestamp
        .checked_sub(parent)
        .ok_or(EmissionError::BlockTooEarly)?;
    if gap < MIN_BLOCK_GAP_SECONDS {
        return Err(EmissionError::BlockTooEarly);
    }
    Ok(gap)
}

impl PlanClock {
    /// Plan time zero (at genesis).
    #[must_use]
    pub const fn new() -> Self {
        Self { seconds: 0 }
    }

    /// Adds the gap between a parent and a child block, capped at 600 s.
    ///
    /// # Errors
    /// [`EmissionError::BlockTooEarly`] if the gap is below 10 s.
    pub fn record(
        &mut self,
        parent: UnixSeconds,
        timestamp: UnixSeconds,
    ) -> Result<(), EmissionError> {
        let credit = block_gap(parent, timestamp)?.min(PLAN_GAP_CAP_SECONDS);
        self.seconds = self
            .seconds
            .checked_add(credit)
            .ok_or(ArithmeticError::Overflow)?;
        Ok(())
    }

    /// Plan time in seconds.
    #[must_use]
    pub const fn seconds(&self) -> u64 {
        self.seconds
    }

    /// Full plan days.
    #[must_use]
    pub const fn full_days(&self) -> u64 {
        match self.seconds.checked_div(SECONDS_PER_DAY) {
            Some(days) => days,
            None => 0,
        }
    }
}

/// Tracks blocks and time of the running epoch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EpochClock {
    first_timestamp: UnixSeconds,
    blocks: u64,
}

impl EpochClock {
    /// Starts an epoch whose time is measured from `first_timestamp`
    /// (the genesis timestamp for epoch 0, otherwise the first block).
    #[must_use]
    pub const fn start(first_timestamp: UnixSeconds) -> Self {
        Self {
            first_timestamp,
            blocks: 0,
        }
    }

    /// Blocks counted in this epoch so far.
    #[must_use]
    pub const fn blocks(&self) -> u64 {
        self.blocks
    }

    /// Counts a block and returns `true` if it ends the epoch.
    ///
    /// # Errors
    /// [`EmissionError::BlockTooEarly`] if the timestamp lies before the
    /// start of the epoch.
    pub fn record(&mut self, timestamp: UnixSeconds) -> Result<bool, EmissionError> {
        self.blocks = self
            .blocks
            .checked_add(1)
            .ok_or(ArithmeticError::Overflow)?;
        let elapsed = timestamp
            .checked_sub(self.first_timestamp)
            .ok_or(EmissionError::BlockTooEarly)?;
        Ok(self.blocks >= EPOCH_MIN_BLOCKS && elapsed >= EPOCH_MIN_SECONDS)
    }
}

#[cfg(test)]
#[allow(clippy::arithmetic_side_effects)]
mod tests {
    use super::*;

    #[test]
    fn gaps_count_at_most_600_seconds() {
        let mut clock = PlanClock::new();
        clock.record(0, 60).unwrap();
        clock.record(60, 3_660).unwrap();
        assert_eq!(clock.seconds(), 660);
    }

    #[test]
    fn too_early_blocks_are_rejected() {
        let mut clock = PlanClock::new();
        assert_eq!(clock.record(100, 109), Err(EmissionError::BlockTooEarly));
        assert_eq!(clock.record(100, 50), Err(EmissionError::BlockTooEarly));
        assert_eq!(clock.seconds(), 0);
        assert!(clock.record(100, 110).is_ok());
    }

    /// Ends of an epoch with constant block gaps, measured from genesis.
    fn epoch_length(gap: u64) -> (u64, u64) {
        let mut clock = EpochClock::start(0);
        let mut ts = 0;
        loop {
            ts += gap;
            if clock.record(ts).unwrap() {
                return (clock.blocks(), ts);
            }
        }
    }

    #[test]
    fn epoch_with_60_second_blocks() {
        assert_eq!(epoch_length(60), (1_440, 86_400));
    }

    #[test]
    fn epoch_with_10_second_blocks_still_lasts_24_hours() {
        assert_eq!(epoch_length(10), (8_640, 86_400));
    }

    #[test]
    fn epoch_with_slow_blocks_needs_1440_blocks() {
        assert_eq!(epoch_length(120), (1_440, 172_800));
    }
}
