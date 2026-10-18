//! Kani proof harnesses (compiled only by `cargo kani`, see CI).
//!
//! They prove for **all** inputs in range, not just samples:
//! one epoch never emits more than the remaining amount, the issued sum
//! stays at or below 10^26, and a block mint always adds up exactly.

use toy_types::params::{FACTOR_MAX, FACTOR_MIN, MAX_SUPPLY, START_REWARD_INITIAL_SHARE};
use toy_types::{Amount, Ppm};

use crate::mint::block_mint;
use crate::schedule::{EpochEmission, Phase, emission_for};

fn any_factor() -> Ppm {
    let m: u64 = kani::any();
    kani::assume(m >= FACTOR_MIN.get() && m <= FACTOR_MAX.get());
    Ppm::new(m)
}

#[kani::proof]
fn emission_never_exceeds_remaining() {
    let remaining: u128 = kani::any();
    kani::assume(remaining <= MAX_SUPPLY.units());
    let emission = emission_for(Amount::from_units(remaining), any_factor());
    assert!(matches!(emission, Ok(e) if e.units() <= remaining));
}

#[kani::proof]
fn issued_stays_within_cap() {
    let issued: u128 = kani::any();
    kani::assume(issued <= MAX_SUPPLY.units());
    let remaining = MAX_SUPPLY.units() - issued;
    let emission = emission_for(Amount::from_units(remaining), any_factor());
    assert!(matches!(emission, Ok(e) if issued + e.units() <= MAX_SUPPLY.units()));
}

#[kani::proof]
fn block_mint_adds_up() {
    let emission: u128 = kani::any();
    // Largest possible epoch emission: 10^26 * 505 * 2 / 10^6.
    kani::assume(emission <= 101_000_000_000_000_000_000_000);
    let index: u64 = kani::any();
    kani::assume(index >= 1 && index <= 1_441);
    let share: u64 = kani::any();
    kani::assume(share <= START_REWARD_INITIAL_SHARE.get());
    let bootstrap: bool = kani::any();
    let epoch = EpochEmission {
        epoch: 0,
        emission: Amount::from_units(emission),
        factor: Ppm::ONE,
        plan_days: 0,
        phase: if bootstrap {
            Phase::Bootstrap
        } else {
            Phase::Normal
        },
        start_validator_share: Ppm::new(share),
    };
    let mint = block_mint(&epoch, index);
    assert!(matches!(mint, Ok(m) if m.sum_of_parts() == Ok(m.total)));
}
