//! Start wage of the start validators (spec 5.3, decisions 110, 139).
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod common;

use toy_emission::bootstrap::effective_share;
use toy_emission::ktable::k_at;
use toy_emission::mint::{block_emission, block_mint};
use toy_emission::plan::{PlanCurve, plan_step};
use toy_emission::{EmissionState, EpochEmission, EpochStart, Phase, emission_for};
use toy_types::params::{EMISSION_BLOCKS_PER_EPOCH, MAX_SUPPLY, START_VALIDATORS};
use toy_types::{Amount, Ppm};

/// Exact start wage for 1-day epochs at m = 1 over the whole 365 days
/// (reference: integer replay in Python with the K table, same rounding
/// per block, wage only).
const WAGE_ONE_DAY_EPOCHS: u128 = 1_141_767_405_168_125_713_529_575;

/// Start wage of an epoch: the wage share of every block, rounded down.
fn wage(epoch: &EpochEmission) -> Amount {
    let mut sum = Amount::ZERO;
    for index in 1..=EMISSION_BLOCKS_PER_EPOCH {
        let block = block_emission(epoch.emission, index).unwrap();
        sum = sum
            .checked_add(block.mul_ppm(epoch.start_validator_share).unwrap())
            .unwrap();
    }
    sum
}

/// Paid to the start validators (wage plus committee pot), in equal parts.
fn paid_to_validators(epoch: &EpochEmission) -> Amount {
    let mut sum = Amount::ZERO;
    for index in 1..=EMISSION_BLOCKS_PER_EPOCH {
        let mint = block_mint(epoch, index).unwrap();
        assert_eq!(mint.start_count, START_VALIDATORS);
        sum = sum.checked_add(mint.start_total().unwrap()).unwrap();
    }
    sum
}

fn epoch_at(e: u64, t: u64, emission: Amount, factor: Ppm, share: Ppm) -> EpochEmission {
    let row = toy_emission::ktable::at(t);
    EpochEmission {
        epoch: e,
        emission,
        factor,
        plan_days: t,
        phase: Phase::Bootstrap,
        start_validator_share: share,
        k: row.k,
        work_share: Ppm::new(u64::from(row.w)),
        tide: Ppm::new(u64::from(row.tide)),
    }
}

/// Replays the bootstrap with a constant factor and `days_per_epoch`.
fn wage_with_constant_factor(days_per_epoch: u64, factor: Ppm, epochs: u64) -> Amount {
    let mut curve = PlanCurve::new();
    let (mut issued, mut mark, mut total) = (Amount::ZERO, Amount::ZERO, Amount::ZERO);
    for e in 0..epochs {
        let t = e * days_per_epoch;
        let remaining = MAX_SUPPLY.checked_sub(issued).unwrap();
        let emission = emission_for(remaining, k_at(t), factor).unwrap();
        let next = plan_step(curve.at(t).unwrap(), t).unwrap();
        let share = effective_share(t, emission, next.checked_sub(mark).unwrap()).unwrap();
        mark = next;
        total = total
            .checked_add(wage(&epoch_at(e, t, emission, factor, share)))
            .unwrap();
        issued = issued.checked_add(emission).unwrap();
    }
    total
}

#[test]
fn first_epoch_pays_wage_and_committee_in_equal_parts() {
    let mut state = EmissionState::genesis();
    let e = state
        .start_epoch(&EpochStart {
            epoch: 0,
            plan_days: 0,
            measurements: None,
            phase: Phase::Bootstrap,
        })
        .unwrap();
    assert_eq!(e.start_validator_share, Ppm::new(250_000));
    // 25 % wage + 5 % committee of 15,821.9394 TOY = 4,746.58 TOY.
    let paid = paid_to_validators(&e);
    let expected = Amount::from_units(4_746_581_820_000_000_000_000);
    assert!(paid <= expected);
    // Only rounding to whole units per validator goes to the refund pool.
    assert!(expected.units() - paid.units() < 1_440 * u128::from(START_VALIDATORS) * 2);
}

#[test]
fn total_wage_over_365_days_matches_the_reference() {
    let mut state = EmissionState::genesis();
    let mut total = Amount::ZERO;
    for epoch in 0..400u64 {
        let e = state
            .start_epoch(&EpochStart {
                epoch,
                plan_days: epoch,
                measurements: (epoch > 0).then(|| common::sim_measurements(0, 990_000)),
                phase: Phase::Bootstrap,
            })
            .unwrap();
        assert_eq!(e.factor, Ppm::ONE);
        if epoch >= 365 {
            assert_eq!(e.start_validator_share, Ppm::ZERO, "no wage after 365 days");
        }
        total = total.checked_add(wage(&e)).unwrap();
    }
    assert_eq!(total.units(), WAGE_ONE_DAY_EPOCHS);
    assert_eq!(
        wage_with_constant_factor(1, Ppm::ONE, 400).units(),
        WAGE_ONE_DAY_EPOCHS
    );
}

#[test]
fn faster_emission_or_longer_epochs_never_pay_more_wage() {
    for factor in [500_000, 1_000_000, 1_500_000, 2_000_000] {
        for days in [1, 2, 3, 7] {
            let total = wage_with_constant_factor(days, Ppm::new(factor), 400);
            assert!(
                total.units() <= WAGE_ONE_DAY_EPOCHS,
                "m {factor}, {days} days per epoch"
            );
        }
    }
    assert!(wage_with_constant_factor(3, Ppm::ONE, 150).units() < WAGE_ONE_DAY_EPOCHS / 2);
}

#[test]
fn epoch_without_a_new_plan_day_pays_no_wage() {
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
    assert_eq!(wage(&e), Amount::ZERO);
}

#[test]
fn after_the_wage_ends_start_validators_keep_the_committee_pot() {
    let epoch = epoch_at(400, 400, Amount::from_toy(14_400), Ppm::ONE, Ppm::ZERO);
    let mint = block_mint(&epoch, 1).unwrap();
    assert_eq!(mint.total, Amount::from_toy(10));
    // 5 % committee = 0.5 TOY for 64 start validators.
    assert_eq!(
        mint.start_total().unwrap(),
        Amount::from_units(500_000_000_000_000_000)
    );
    assert_eq!(mint.committee, Amount::ZERO);
    assert_eq!(mint.sum_of_parts().unwrap(), mint.total);
}

#[test]
fn normal_phase_fills_the_committee_pot_and_bootstrap_cannot_return() {
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
    assert_eq!(mint.start_total().unwrap(), Amount::ZERO);
    assert_eq!(
        mint.committee,
        mint.total.mul_ppm(Ppm::new(50_000)).unwrap()
    );
    assert!(state.start_epoch(&start(2, Phase::Bootstrap)).is_err());
    assert!(state.handover_done());
}

#[test]
fn split_follows_the_spec_on_day_0() {
    let mut state = EmissionState::genesis();
    let e = state
        .start_epoch(&EpochStart {
            epoch: 0,
            plan_days: 0,
            measurements: None,
            phase: Phase::Bootstrap,
        })
        .unwrap();
    let m = block_mint(&e, 1).unwrap();
    let t = m.total;
    let ppm = |p| t.mul_ppm(Ppm::new(p)).unwrap();
    // Work 45 %: base 85 %, Funke 8 %, Welle 5 %, Gezeiten 2 %.
    let work = ppm(450_000);
    assert_eq!(m.work.base, work.mul_ppm(Ppm::new(850_000)).unwrap());
    assert_eq!(m.work.funke, work.mul_ppm(Ppm::new(80_000)).unwrap());
    assert_eq!(m.work.welle, work.mul_ppm(Ppm::new(50_000)).unwrap());
    assert_eq!(m.work.gezeiten, work.mul_ppm(Ppm::new(20_000)).unwrap());
    assert_eq!(m.full_node, ppm(80_000));
    // Staking: 100 − 45 − 25 − 5 − 8 − 2 = 15 %.
    assert_eq!(m.staking, ppm(150_000));
    assert!(m.refund_pool >= ppm(20_000));
    assert_eq!(m.sum_of_parts().unwrap(), t);
}
