//! Property tests: the supply cap is never exceeded and m behaves as
//! specified, for many random inputs (bauplan M1, spec 9 and 9.1).
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use common::TestRng;
use toy_emission::factor::next_factor;
use toy_emission::mint::block_mint;
use toy_emission::plan::plan_after_days;
use toy_emission::{
    EmissionState, EpochEmission, EpochMeasurements, EpochStart, Phase, emission_for,
};
use toy_types::params::{FACTOR_MAX, FACTOR_MIN, MAX_SUPPLY, START_REWARD_INITIAL_SHARE};
use toy_types::{Amount, Ppm};

fn random_measurements(rng: &mut TestRng) -> EpochMeasurements {
    let blocks = rng.range(1_440, 20_000);
    let expected = blocks * rng.range(1, 128);
    EpochMeasurements {
        blocks,
        counted_tx: rng.range(0, blocks * 3_000),
        free_tx: rng.range(0, blocks * 3_000),
        signatures_received: rng.range(0, expected),
        signatures_expected: expected,
    }
}

#[test]
fn constant_m_2_over_100_years_never_exceeds_cap() {
    let mut issued = Amount::ZERO;
    for _ in 0..(100 * 365) {
        let remaining = MAX_SUPPLY.checked_sub(issued).unwrap();
        let e = emission_for(remaining, FACTOR_MAX).unwrap();
        assert!(e <= remaining);
        issued = issued.checked_add(e).unwrap();
        assert!(issued <= MAX_SUPPLY);
    }
    // At m = 2 almost everything is out after 100 years, but never more.
    assert!(issued.units() > MAX_SUPPLY.units() / 100 * 99);
}

#[test]
fn random_histories_never_exceed_cap_and_m_moves_slowly() {
    for seed in 0..12u64 {
        let mut rng = TestRng::new(seed);
        let mut state = EmissionState::genesis();
        let mut plan_days = 0u64;
        let mut previous_m = Ppm::ONE;
        let mut previous_issued = Amount::ZERO;
        let epochs = if seed < 2 { 100 * 365 } else { 3_000 };
        for epoch in 0..epochs {
            let start = EpochStart {
                epoch,
                plan_days,
                measurements: (epoch > 0).then(|| random_measurements(&mut rng)),
                phase: Phase::Normal,
            };
            let e = state.start_epoch(&start).unwrap();
            let m = e.factor.get();
            assert!(
                (FACTOR_MIN.get()..=FACTOR_MAX.get()).contains(&m),
                "seed {seed} epoch {epoch}"
            );
            assert!(
                m.abs_diff(previous_m.get()) <= 50_000,
                "seed {seed} epoch {epoch}: m jumped"
            );
            assert!(state.issued() <= MAX_SUPPLY, "seed {seed} epoch {epoch}");
            assert!(state.issued() >= previous_issued, "issued never shrinks");
            assert_eq!(
                state.remaining().checked_add(state.issued()),
                Ok(MAX_SUPPLY)
            );
            previous_m = e.factor;
            previous_issued = state.issued();
            // Epochs of 1 to 3 plan days, sometimes no full day (capped gaps).
            plan_days += rng.range(0, 3);
        }
    }
}

#[test]
fn free_transactions_never_change_m() {
    let mut rng = TestRng::new(99);
    for _ in 0..2_000 {
        let base = random_measurements(&mut rng);
        let spam = EpochMeasurements {
            free_tx: rng.next_u64() >> 8,
            ..base
        };
        let m = Ppm::new(rng.range(FACTOR_MIN.get(), FACTOR_MAX.get()));
        let plan = plan_after_days(rng.range(1, 5_000)).unwrap();
        let issued = Amount::from_units(rng.u128_below(MAX_SUPPLY.units()));
        assert_eq!(
            next_factor(m, &base, issued, plan).unwrap().factor,
            next_factor(m, &spam, issued, plan).unwrap().factor
        );
    }
}

#[test]
fn factor_rejects_invalid_input() {
    let ok = EpochMeasurements {
        blocks: 1_440,
        counted_tx: 0,
        free_tx: 0,
        signatures_received: 1,
        signatures_expected: 1,
    };
    let plan = plan_after_days(1).unwrap();
    assert!(next_factor(Ppm::new(499_999), &ok, plan, plan).is_err());
    assert!(next_factor(Ppm::new(2_000_001), &ok, plan, plan).is_err());
    let no_blocks = EpochMeasurements { blocks: 0, ..ok };
    assert!(next_factor(Ppm::ONE, &no_blocks, plan, plan).is_err());
    let no_sigs = EpochMeasurements {
        signatures_expected: 0,
        ..ok
    };
    assert!(next_factor(Ppm::ONE, &no_sigs, plan, plan).is_err());
}

#[test]
fn every_epoch_mints_exactly_its_emission() {
    let mut rng = TestRng::new(7);
    let largest = 101_000_000_000_000_000_000_000u128; // 10^26 * 0.000505 * 2
    for case in 0..300 {
        let emission = if case == 0 {
            largest
        } else {
            rng.u128_below(largest + 1)
        };
        let share = rng.range(0, START_REWARD_INITIAL_SHARE.get());
        for phase in [Phase::Bootstrap, Phase::Normal] {
            let epoch = EpochEmission {
                epoch: 1,
                emission: Amount::from_units(emission),
                factor: Ppm::ONE,
                plan_days: 1,
                phase,
                start_validator_share: Ppm::new(share),
            };
            let mut sum = Amount::ZERO;
            for index in 1..=1_500 {
                let mint = block_mint(&epoch, index).unwrap();
                assert_eq!(mint.sum_of_parts(), Ok(mint.total));
                sum = sum.checked_add(mint.total).unwrap();
            }
            assert_eq!(sum.units(), emission, "case {case}");
        }
    }
}

#[test]
fn block_index_zero_is_rejected() {
    let epoch = EpochEmission {
        epoch: 0,
        emission: Amount::from_toy(50_500),
        factor: Ppm::ONE,
        plan_days: 0,
        phase: Phase::Normal,
        start_validator_share: Ppm::ZERO,
    };
    assert!(block_mint(&epoch, 0).is_err());
}

#[test]
fn epochs_must_come_in_order_and_plan_time_cannot_go_back() {
    let mut state = EmissionState::genesis();
    let m = common::sim_measurements(0, 990_000);
    let start = |epoch, plan_days| EpochStart {
        epoch,
        plan_days,
        measurements: Some(m),
        phase: Phase::Normal,
    };
    assert!(state.start_epoch(&start(1, 0)).is_err());
    state.start_epoch(&start(0, 0)).unwrap();
    state.start_epoch(&start(1, 5)).unwrap();
    let before = state.clone();
    assert!(state.start_epoch(&start(2, 4)).is_err());
    let missing = EpochStart {
        measurements: None,
        ..start(2, 6)
    };
    assert!(state.start_epoch(&missing).is_err());
    assert_eq!(state, before, "failed calls leave the state unchanged");
}
