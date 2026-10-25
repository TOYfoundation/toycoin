//! The K table of the tide curve (spec 9.1, decision 136).
//!
//! One row every 30 plan days for 200 years: `k` (emission rate per epoch
//! in units of 10^-12), the work share `w` and the tide `W` (both in ppm).
//! Between two rows the values are interpolated with integers:
//! `v(30 j + r) = v_j + (v_(j+1) − v_j) × r div 30` (rounding towards
//! negative infinity, like `div_euclid`). After the last row its values
//! stay. The table is generated once by `docs/anhang/v04/k_tabelle.py`;
//! its BLAKE3 hash is a constitution constant (`K_TABLE_BLAKE3`).
//!
//! `t` is the number of full plan days at the start of the epoch, the same
//! time as for the plan `P(t)`, so the wave follows real time.

use toy_types::Ppm;
use toy_types::params::{K_TABLE_ROWS, K_TABLE_STEP_DAYS};

use crate::k_table_data::{K_TABLE, ROWS};

const _: () = assert!(ROWS == K_TABLE_ROWS, "K table size must match the constant");

/// One row of the K table, in units of 10^-12 (`k`) and ppm (`w`, `tide`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KRow {
    /// Emission rate per epoch, in 10^-12 of the remaining amount.
    pub k: u64,
    /// Share of the work reward.
    pub w: u32,
    /// Tide factor `W` (1,000,000 = 1.0).
    pub tide: u32,
}

/// All rows, in order.
pub fn rows() -> impl Iterator<Item = KRow> {
    K_TABLE.iter().map(|&(k, w, tide)| KRow { k, w, tide })
}

fn row(index: usize) -> KRow {
    let last = K_TABLE.last().copied().unwrap_or((0, 0, 0));
    let (k, w, tide) = K_TABLE.get(index).copied().unwrap_or(last);
    KRow { k, w, tide }
}

/// Integer interpolation between `a` (at `r = 0`) and `b` (at `r = 30`).
fn interpolate(a: u64, b: u64, r: u64) -> u64 {
    let (a, b, r) = (i128::from(a), i128::from(b), i128::from(r));
    let step = i128::from(K_TABLE_STEP_DAYS);
    // |b − a| < 2^40 and r < 30, so nothing here can overflow.
    let delta = b.saturating_sub(a).saturating_mul(r).div_euclid(step);
    u64::try_from(a.saturating_add(delta)).unwrap_or(0)
}

/// Values of the K table on plan day `days`.
#[must_use]
pub fn at(days: u64) -> KRow {
    let index = days.checked_div(K_TABLE_STEP_DAYS).unwrap_or(0);
    let r = days.checked_rem(K_TABLE_STEP_DAYS).unwrap_or(0);
    let Ok(j) = usize::try_from(index) else {
        return row(K_TABLE_ROWS);
    };
    if j.saturating_add(1) >= K_TABLE_ROWS {
        return row(K_TABLE_ROWS);
    }
    let (a, b) = (row(j), row(j.saturating_add(1)));
    KRow {
        k: interpolate(a.k, b.k, r),
        w: u32::try_from(interpolate(u64::from(a.w), u64::from(b.w), r)).unwrap_or(a.w),
        tide: u32::try_from(interpolate(u64::from(a.tide), u64::from(b.tide), r)).unwrap_or(a.tide),
    }
}

/// Emission rate `k(t)` in 10^-12 per epoch.
#[must_use]
pub fn k_at(days: u64) -> u64 {
    at(days).k
}

/// Work share `w(t)`.
#[must_use]
pub fn work_share_at(days: u64) -> Ppm {
    Ppm::new(u64::from(at(days).w))
}

/// Tide `W(t)` (1.0 = average level).
#[must_use]
pub fn tide_at(days: u64) -> Ppm {
    Ppm::new(u64::from(at(days).tide))
}

/// Largest `k` of the whole table.
#[must_use]
pub fn k_max() -> u64 {
    K_TABLE.iter().map(|&(k, _, _)| k).max().unwrap_or(0)
}

#[cfg(test)]
#[allow(clippy::arithmetic_side_effects)]
mod tests {
    use super::*;

    #[test]
    fn values_match_the_reference_simulation() {
        // From docs/anhang/v04/m_simulation_v04.py.
        assert_eq!(k_at(0), 158_219_394);
        assert_eq!(k_at(15), 171_149_831);
        assert_eq!(k_at(29), 183_218_238);
        assert_eq!(k_at(30), 184_080_268);
        assert_eq!(k_at(1_000), 461_150_456);
        assert_eq!(work_share_at(0), Ppm::new(450_000));
        assert_eq!(work_share_at(15), Ppm::new(447_966));
        assert_eq!(tide_at(0), Ppm::new(750_000));
        assert_eq!(tide_at(912), Ppm::new(1_249_679));
    }

    #[test]
    fn last_row_holds_forever() {
        let last = row(K_TABLE_ROWS);
        assert_eq!(at(u64::MAX), last);
        assert_eq!(at(30 * 2_433), last);
        assert_eq!(at(30 * 2_433 + 29), last);
        assert_eq!(work_share_at(u64::MAX), Ppm::new(250_000));
    }

    #[test]
    fn k_stays_far_below_one_even_at_twice_the_rate() {
        // Constitution (spec 13 no. 3): k × 2 < 1 for every row.
        assert_eq!(k_max(), 470_811_069);
        assert!(u128::from(k_max()) * 2 < toy_types::params::K_SCALE);
        assert!(rows().all(|r| r.k > 0 && r.w <= 450_000 && r.w >= 250_000));
    }
}
