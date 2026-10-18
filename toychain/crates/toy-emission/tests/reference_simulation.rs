//! The Rust emission must produce exactly the numbers of
//! `docs/anhang/m-simulation.py` (bauplan M1, spec 9.1).
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;
mod vectors;

use common::simulate;
use toy_emission::factor::next_factor;
use toy_emission::plan::plan_after_days;
use toy_emission::{EmissionState, EpochMeasurements, EpochStart, Phase, emission_for};
use toy_types::params::MAX_SUPPLY;
use toy_types::{Amount, Ppm};

const DAY: u64 = 86_400;
/// Plan after 365 days (reference simulation).
const PLAN_YEAR_1: u128 = 16_837_325_093_902_467_806_409_350;

#[test]
fn case_low_usage_good_participation() {
    assert_eq!(
        simulate(365, 0, 990_000, DAY),
        (PLAN_YEAR_1, 1_000_000, 1_000_000)
    );
    assert_eq!(plan_after_days(365).unwrap().units(), PLAN_YEAR_1);
}

#[test]
fn case_high_usage_good_participation() {
    assert_eq!(
        simulate(365, 500_000, 990_000, DAY),
        (17_213_576_046_918_562_635_373_598, 1_000_000, 1_050_070)
    );
}

#[test]
fn case_low_usage_weak_participation() {
    assert_eq!(
        simulate(365, 0, 700_000, DAY),
        (16_501_127_939_480_130_788_957_341, 950_000, 1_000_000)
    );
}

#[test]
fn case_high_usage_25_hour_epochs() {
    assert_eq!(
        simulate(365, 500_000, 990_000, 25 * 3_600),
        (17_156_582_138_340_268_583_222_705, 1_000_000, 1_222_950)
    );
}

#[test]
fn hundred_years_of_high_usage_stay_below_the_cap() {
    let (issued, lo, hi) = simulate(100 * 365, 500_000, 990_000, DAY);
    assert_eq!(issued, 99_999_999_980_508_995_028_158_158);
    assert_eq!(MAX_SUPPLY.units() - issued, 19_491_004_971_841_842);
    assert_eq!((lo, hi), (1_000_000, 1_250_000));
}

#[test]
fn factor_matches_reference_vectors() {
    for (i, &(m, blocks, counted, got, expected, issued, plan, want)) in
        vectors::factor_vectors::FACTOR_VECTORS.iter().enumerate()
    {
        let measurements = EpochMeasurements {
            blocks,
            counted_tx: counted,
            free_tx: 0,
            signatures_received: got,
            signatures_expected: expected,
        };
        let result = next_factor(
            Ppm::new(m),
            &measurements,
            Amount::from_units(issued),
            Amount::from_units(plan),
        )
        .unwrap();
        assert_eq!(result.factor.get(), want, "vector {i}");
    }
}

#[test]
fn first_epoch_emits_50500_toy() {
    let mut state = EmissionState::genesis();
    let e = state
        .start_epoch(&EpochStart {
            epoch: 0,
            plan_days: 0,
            measurements: None,
            phase: Phase::Bootstrap,
        })
        .unwrap();
    assert_eq!(e.emission, Amount::from_toy(50_500));
    assert_eq!(e.factor, Ppm::ONE);
}

#[test]
fn half_life_is_about_1372_epochs_at_m_1() {
    let mut remaining = MAX_SUPPLY;
    let mut epochs = 0u64;
    while remaining.units() * 2 > MAX_SUPPLY.units() {
        let e = emission_for(remaining, Ppm::ONE).unwrap();
        remaining = remaining.checked_sub(e).unwrap();
        epochs += 1;
    }
    // ln 2 / ln(1 / (1 - 0.000505)) = 1372.25: half is reached in epoch 1373.
    assert_eq!(epochs, 1_373);
}
