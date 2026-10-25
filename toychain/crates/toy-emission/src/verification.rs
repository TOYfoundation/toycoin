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
//! - Block mints are proven for every epoch emission and block index with a
//!   zero start-validator share. With a non-zero share the proofs did not
//!   finish within 30 minutes (October 2026); they are kept here but not run
//!   in CI. `block_mint` checks at runtime that its parts add up and returns
//!   an error otherwise, so a wrong split can never mint, and property tests
//!   cover 300 random epochs with every block.

use toy_types::params::{FACTOR_MAX, MAX_SUPPLY};
use toy_types::{Amount, Ppm};

use crate::mint::block_mint;
use crate::schedule::{EpochEmission, Phase, emission_for};

/// Largest `k` of the K table (checked by a unit test in `ktable`). The
/// emission grows with `k` as it grows with `m`, so proving the bounds at
/// the largest `k` and `m = 2.0` covers every row of the table.
const K_MAX: u64 = 470_811_069;

/// Upper bound of any epoch emission: 10^26 × K_MAX × 2 / 10^18 ≈ 9.4 × 10^22.
const MAX_EPOCH_EMISSION: u128 = 100_000_000_000_000_000_000_000;

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
