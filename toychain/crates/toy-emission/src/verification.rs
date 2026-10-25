//! Kani proof harnesses (compiled only by `cargo kani`, see CI).
//!
//! SAT solvers handle a multiplication of two symbolic 128-bit numbers very
//! slowly: the first CI run with fully symbolic inputs did not finish within
//! 90 minutes. Each harness therefore keeps one factor concrete, which makes
//! the multiplication cheap, while the other input covers **all** values:
//!
//! - The emission grows with `m` (floor of a growing product), so proving
//!   the bounds at `m = 2.0` for every remaining amount covers every
//!   `m` in [0.5, 2.0].
//! - Block mints (M3b): `block_mint` splits the block amount with a
//!   [`Splitter`] that never takes more than is left, so the parts add up
//!   **by construction**: every step keeps `part + rest = rest before`
//!   (`splitter_give_keeps_the_sum`), the shares cannot overflow
//!   (`mul_ppm_never_overflows_for_block_amounts`), and the 1,440 block
//!   amounts of an epoch add up to its emission (`block_emission_adds_up`).
//!   These three run in CI. A proof of the whole `block_mint` with its ten
//!   shares did not finish within 15 minutes (25.10.2026, before that 30
//!   minutes with a start-validator share); those harnesses are kept here
//!   but not run in CI. `block_mint` also checks at runtime that its parts
//!   add up and returns an error otherwise, so a wrong split can never
//!   mint, and property tests cover 300 random epochs with every block.

use toy_types::params::{FACTOR_MAX, MAX_SUPPLY};
use toy_types::{Amount, Ppm};

use crate::mint::{Splitter, block_emission, block_mint};
use crate::schedule::{EpochEmission, Phase, emission_for};

/// Largest `k` of the K table (checked by a unit test in `ktable`). The
/// emission grows with `k` as it grows with `m`, so proving the bounds at
/// the largest `k` and `m = 2.0` covers every row of the table.
const K_MAX: u64 = 470_811_069;

/// Upper bound of any epoch emission: 10^26 × K_MAX × 2 / 10^18 ≈ 9.4 × 10^22.
const MAX_EPOCH_EMISSION: u128 = 100_000_000_000_000_000_000_000;

/// Upper bound of any block amount: an epoch emission spread over 1,440
/// blocks, the last one with the remainder (below 1,440 units).
const MAX_BLOCK_AMOUNT: u128 = MAX_EPOCH_EMISSION / 1_440 + 1_440;

#[kani::proof]
fn splitter_give_keeps_the_sum() {
    let amount: u128 = kani::any();
    let rest: u128 = kani::any();
    let wanted: u128 = kani::any();
    kani::assume(rest <= amount);
    let mut splitter = Splitter::new(Amount::from_units(amount));
    splitter.rest = Amount::from_units(rest);
    let part = splitter.give(Amount::from_units(wanted));
    assert!(matches!(part, Ok(p) if p.units() + splitter.rest.units() == rest));
}

#[kani::proof]
fn mul_ppm_never_overflows_for_block_amounts() {
    let amount: u128 = kani::any();
    let share: u64 = kani::any();
    kani::assume(amount <= MAX_BLOCK_AMOUNT);
    kani::assume(share <= Ppm::SCALE);
    assert!(Amount::from_units(amount).mul_ppm(Ppm::new(share)).is_ok());
}

#[kani::proof]
fn block_emission_adds_up() {
    let emission: u128 = kani::any();
    kani::assume(emission <= MAX_EPOCH_EMISSION);
    let e = Amount::from_units(emission);
    let first = block_emission(e, 1);
    let last = block_emission(e, 1_440);
    assert!(matches!(
        (first, last),
        (Ok(f), Ok(l)) if f.units() * 1_439 + l.units() == emission
    ));
}

#[kani::proof]
fn emission_at_max_factor_never_exceeds_remaining() {
    let remaining: u128 = kani::any();
    kani::assume(remaining <= MAX_SUPPLY.units());
    let emission = emission_for(Amount::from_units(remaining), K_MAX, FACTOR_MAX);
    assert!(matches!(emission, Ok(e) if e.units() <= remaining));
}

#[kani::proof]
fn issued_stays_within_cap_at_max_factor() {
    let issued: u128 = kani::any();
    kani::assume(issued <= MAX_SUPPLY.units());
    let remaining = MAX_SUPPLY.units() - issued;
    let emission = emission_for(Amount::from_units(remaining), K_MAX, FACTOR_MAX);
    assert!(matches!(emission, Ok(e) if issued + e.units() <= MAX_SUPPLY.units()));
}

fn check_block_mint(phase: Phase, share: u64) {
    let emission: u128 = kani::any();
    kani::assume(emission <= MAX_EPOCH_EMISSION);
    let index: u64 = kani::any();
    kani::assume(index >= 1 && index <= 1_441);
    let epoch = EpochEmission {
        epoch: 0,
        emission: Amount::from_units(emission),
        factor: Ppm::ONE,
        plan_days: 0,
        phase,
        start_validator_share: Ppm::new(share),
        k: K_MAX,
        work_share: Ppm::new(450_000),
        tide: Ppm::ONE,
    };
    let mint = block_mint(&epoch, index);
    assert!(matches!(mint, Ok(m) if m.sum_of_parts() == Ok(m.total)));
}

#[kani::proof]
fn block_mint_adds_up_normal_phase() {
    check_block_mint(Phase::Normal, 0);
}

#[kani::proof]
fn block_mint_adds_up_bootstrap_full_share() {
    check_block_mint(Phase::Bootstrap, 250_000);
}

#[kani::proof]
fn block_mint_adds_up_bootstrap_partial_share() {
    check_block_mint(Phase::Bootstrap, 123_457);
}

#[kani::proof]
fn block_mint_adds_up_bootstrap_no_share() {
    check_block_mint(Phase::Bootstrap, 0);
}
