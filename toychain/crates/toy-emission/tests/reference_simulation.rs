//! The Rust emission must produce exactly the numbers of
//! `docs/anhang/v04/m_simulation_v04.py` (bauplan M3b, spec 9.1). The
//! factor `m` itself still follows `docs/anhang/m-simulation.py`.
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
use toy_emission::ktable::k_at;
use toy_emission::plan::plan_after_days;
use toy_emission::{EmissionState, EpochMeasurements, EpochStart, Phase, emission_for};
use toy_types::params::MAX_SUPPLY;
use toy_types::{Amount, Ppm};

const DAY: u64 = 86_400;
/// Plan after 365 days (`docs/anhang/v04/m_simulation_v04.py`).
const PLAN_YEAR_1: u128 = 10_163_927_417_485_694_669_151_879;

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
        (10_392_737_620_032_826_824_030_966, 1_000_000, 1_050_000)
    );
}

#[test]
fn case_low_usage_weak_participation() {
    assert_eq!(
        simulate(365, 0, 700_000, DAY),
        (9_959_552_212_332_794_068_076_824, 950_000, 1_000_000)
    );
}

#[test]
fn case_high_usage_25_hour_epochs() {
    assert_eq!(
        simulate(365, 500_000, 990_000, 25 * 3_600),
        (10_359_573_917_483_290_000_778_214, 1_000_000, 1_222_800)
    );
}

#[test]
fn plan_matches_the_reference() {
    assert_eq!(
        plan_after_days(1).unwrap().units(),
        15_821_939_400_000_000_000_000
    );
    assert_eq!(
        plan_after_days(3_650).unwrap().units(),
        71_998_471_628_079_577_347_334_240
    );
    assert_eq!(
        plan_after_days(36_500).unwrap().units(),
        99_996_716_612_229_183_548_880_646
    );
}

#[test]
fn hundred_years_of_high_usage_stay_below_the_cap() {
    let (issued, lo, hi) = simulate(100 * 365, 500_000, 990_000, DAY);
    assert_eq!(issued, 99_999_505_188_155_176_791_876_346);
    assert_eq!(MAX_SUPPLY.units() - issued, 494_811_844_823_208_123_654);
    assert_eq!((lo, hi), (1_000_000, 1_249_730));
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
fn first_epoch_emits_15822_toy() {
    let mut state = EmissionState::genesis();
    let e = state
        .start_epoch(&EpochStart {
            epoch: 0,
            plan_days: 0,
            measurements: None,
            phase: Phase::Bootstrap,
        })
        .unwrap();
    assert_eq!(
        e.emission,
        Amount::from_units(15_821_939_400_000_000_000_000)
    );
    assert_eq!(e.factor, Ppm::ONE);
}

#[test]
fn half_is_reached_after_about_5_2_years_at_m_1() {
    let mut remaining = MAX_SUPPLY;
    let mut days = 0u64;
    while remaining.units() * 2 > MAX_SUPPLY.units() {
        let e = emission_for(remaining, k_at(days), Ppm::ONE).unwrap();
        remaining = remaining.checked_sub(e).unwrap();
        days += 1;
    }
    // Reference simulation: half after 1,892 plan days (5.18 years).
    assert_eq!(days, 1_892);
}
