//! Reward of the start validators (spec 5.3, decisions 70, 78, 110).
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use toy_emission::bootstrap::effective_share;
use toy_emission::mint::block_mint;
use toy_emission::plan::{PlanCurve, plan_step};
use toy_emission::{
    ConsensusPayout, EmissionState, EpochEmission, EpochStart, Phase, emission_for,
};
use toy_types::params::{EMISSION_BLOCKS_PER_EPOCH, MAX_SUPPLY, START_VALIDATORS};
use toy_types::{Amount, Ppm};

/// Exact total for 1-day epochs at m = 1 (reference: integer replay of
/// spec 5.3 and 9.0 in Python, same rounding per block).
const TOTAL_ONE_DAY_EPOCHS: u128 = 2_217_786_592_564_682_482_674_618;
/// Upper bound from the spec: below 2,217,786.60 TOY.
const SPEC_BOUND: u128 = 2_217_786_600_000_000_000_000_000;

fn paid_to_validators(epoch: &EpochEmission) -> Amount {
    let mut sum = Amount::ZERO;
    for index in 1..=EMISSION_BLOCKS_PER_EPOCH {
        let mint = block_mint(epoch, index).unwrap();
        if let ConsensusPayout::StartValidators { each, count } = mint.consensus {
            assert_eq!(count, START_VALIDATORS);
            sum = sum
                .checked_add(each.checked_mul(u128::from(count)).unwrap())
                .unwrap();
        }
    }
    sum
}

/// Replays the bootstrap with a constant factor and `days_per_epoch`.
fn total_with_constant_factor(days_per_epoch: u64, factor: Ppm, epochs: u64) -> Amount {
    let mut curve = PlanCurve::new();
    let (mut issued, mut mark, mut total) = (Amount::ZERO, Amount::ZERO, Amount::ZERO);
    for e in 0..epochs {
        let t = e * days_per_epoch;
        let emission = emission_for(MAX_SUPPLY.checked_sub(issued).unwrap(), factor).unwrap();
        let next = plan_step(curve.at(t).unwrap()).unwrap();
        let share = effective_share(t, emission, next.checked_sub(mark).unwrap()).unwrap();
        mark = next;
        let epoch = EpochEmission {
            epoch: e,
            emission,
            factor,
            plan_days: t,
            phase: Phase::Bootstrap,
            start_validator_share: share,
        };
        total = total.checked_add(paid_to_validators(&epoch)).unwrap();
        issued = issued.checked_add(emission).unwrap();
    }
    total
}

#[test]
fn first_epoch_pays_about_25250_toy_in_equal_parts() {
    let mut state = EmissionState::genesis();
    let e = state
        .start_epoch(&EpochStart {
            epoch: 0,
            plan_days: 0,
            measurements: None,
            phase: Phase::Bootstrap,
        })
        .unwrap();
    assert_eq!(e.start_validator_share, Ppm::new(500_000));
    let paid = paid_to_validators(&e);
    let half = Amount::from_toy(25_250);
    assert!(paid <= half);
    // Only rounding to whole units per validator goes to the refund pool.
    assert!(half.units() - paid.units() < 1_440 * 21);
}

#[test]
fn total_over_180_days_matches_the_spec() {
    let mut state = EmissionState::genesis();
    let mut total = Amount::ZERO;
    for epoch in 0..200u64 {
        let e = state
            .start_epoch(&EpochStart {
                epoch,
                plan_days: epoch,
                measurements: (epoch > 0).then(|| common::sim_measurements(0, 990_000)),
                phase: Phase::Bootstrap,
            })
            .unwrap();
        assert_eq!(e.factor, Ppm::ONE);
        if epoch >= 180 {
            assert_eq!(
                e.start_validator_share,
                Ppm::ZERO,
                "no reward after 180 days"
            );
        }
        total = total.checked_add(paid_to_validators(&e)).unwrap();
    }
    assert_eq!(total.units(), TOTAL_ONE_DAY_EPOCHS);
    assert!(total.units() < SPEC_BOUND);
    assert_eq!(
        total_with_constant_factor(1, Ppm::ONE, 200).units(),
        TOTAL_ONE_DAY_EPOCHS
    );
}

#[test]
fn faster_emission_or_longer_epochs_never_pay_more() {
    for factor in [500_000, 1_000_000, 1_500_000, 2_000_000] {
        for days in [1, 2, 3, 7] {
            let total = total_with_constant_factor(days, Ppm::new(factor), 200);
            assert!(
                total.units() <= TOTAL_ONE_DAY_EPOCHS,
                "m {factor}, {days} days per epoch"
            );
        }
    }
    assert!(total_with_constant_factor(3, Ppm::ONE, 80).units() < TOTAL_ONE_DAY_EPOCHS / 2);
}

#[test]
fn epoch_without_a_new_plan_day_pays_nothing() {
    let mut state = EmissionState::genesis();
    let start = |epoch, plan_days| EpochStart {
        epoch,
        plan_days,
        measurements: (epoch > 0).then(|| common::sim_measurements(0, 990_000)),
        phase: Phase::Bootstrap,
    };
    state.start_epoch(&start(0, 0)).unwrap();
    // Long pauses: plan time still at day 0 when epoch 1 starts.
    let e = state.start_epoch(&start(1, 0)).unwrap();
    assert_eq!(e.start_validator_share, Ppm::ZERO);
    assert_eq!(paid_to_validators(&e), Amount::ZERO);
}

#[test]
fn unpaid_bootstrap_share_goes_to_the_work_lottery() {
    let epoch = EpochEmission {
        epoch: 200,
        emission: Amount::from_toy(14_400),
        factor: Ppm::ONE,
        plan_days: 200,
        phase: Phase::Bootstrap,
        start_validator_share: Ppm::ZERO,
    };
    let mint = block_mint(&epoch, 1).unwrap();
    assert_eq!(mint.total, Amount::from_toy(10));
    assert_eq!(mint.work_lottery, Amount::from_toy(9));
    assert_eq!(mint.refund_pool, Amount::from_toy(1));
}

#[test]
fn normal_phase_pays_stakers_and_bootstrap_cannot_return() {
    let mut state = EmissionState::genesis();
    let start = |epoch, phase| EpochStart {
        epoch,
        plan_days: epoch * 100,
        measurements: (epoch > 0).then(|| common::sim_measurements(0, 990_000)),
        phase,
    };
    state.start_epoch(&start(0, Phase::Bootstrap)).unwrap();
    let e = state.start_epoch(&start(1, Phase::Normal)).unwrap();
    assert_eq!(e.start_validator_share, Ppm::ZERO);
    let mint = block_mint(&e, 1).unwrap();
    assert!(matches!(mint.consensus, ConsensusPayout::Stakers(_)));
    assert!(state.start_epoch(&start(2, Phase::Bootstrap)).is_err());
    assert!(state.handover_done());
}
