//! Shared helpers for the emission tests.
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]
#![allow(dead_code)]

use toy_emission::{EmissionState, EpochMeasurements, EpochStart, Phase};
use toy_types::params::SECONDS_PER_DAY;

/// Small deterministic generator (SplitMix64) for property tests.
///
/// It only produces test inputs; it is not used for anything secret.
/// crates.io was not reachable when M1 was built, so `proptest` is not
/// used yet; every failing case prints its seed and can be replayed.
pub struct TestRng(u64);

impl TestRng {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform value in `lo..=hi`.
    pub fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.next_u64() % (hi - lo + 1)
    }

    pub fn u128_below(&mut self, bound: u128) -> u128 {
        let wide = (u128::from(self.next_u64()) << 64) | u128::from(self.next_u64());
        wide % bound
    }
}

/// Measurements as used by `docs/anhang/m-simulation.py`.
pub fn sim_measurements(paid_share_ppm: u64, participation_ppm: u64) -> EpochMeasurements {
    let blocks = 1_440;
    EpochMeasurements {
        blocks,
        counted_tx: paid_share_ppm * blocks * 1_000 / 1_000_000,
        free_tx: 0,
        signatures_received: participation_ppm,
        signatures_expected: 1_000_000,
    }
}

/// Port of `simulate()` from the reference simulation, using the real
/// `EmissionState`. Returns (issued, min m, max m).
pub fn simulate(
    days: u64,
    paid_share_ppm: u64,
    participation_ppm: u64,
    epoch_seconds: u64,
) -> (u128, u64, u64) {
    let mut state = EmissionState::genesis();
    let (mut t, mut epoch) = (0u64, 0u64);
    let (mut lo, mut hi) = (u64::MAX, 0u64);
    while t < days * SECONDS_PER_DAY {
        let start = EpochStart {
            epoch,
            plan_days: t / SECONDS_PER_DAY,
            measurements: (t > 0).then(|| sim_measurements(paid_share_ppm, participation_ppm)),
            phase: Phase::Normal,
        };
        let e = state.start_epoch(&start).expect("epoch starts");
        lo = lo.min(e.factor.get());
        hi = hi.max(e.factor.get());
        t += epoch_seconds;
        epoch += 1;
    }
    (state.issued().units(), lo, hi)
}
