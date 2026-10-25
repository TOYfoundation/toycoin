//! The K table in the code equals the generated CSV, whose BLAKE3 hash is a
//! constitution constant (spec 13 no. 3, decision 136).
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use toy_crypto::hash::blake3;
use toy_emission::ktable::{KRow, rows};
use toy_types::params::{K_SCALE, K_TABLE_BLAKE3, K_TABLE_ROWS};

const CSV: &str = include_str!("../data/k-tabelle.csv");

#[test]
fn csv_hash_is_the_constitution_constant() {
    assert_eq!(blake3(CSV.as_bytes()).as_bytes(), &K_TABLE_BLAKE3);
}

#[test]
fn code_table_equals_the_csv() {
    let mut lines = CSV.lines();
    assert_eq!(lines.next(), Some("j,k_ppt,w_ppm,W_ppm"));
    let parsed: Vec<KRow> = lines
        .enumerate()
        .map(|(i, line)| {
            let cols: Vec<u64> = line.split(',').map(|c| c.parse().unwrap()).collect();
            assert_eq!(cols.len(), 4, "line {i}");
            assert_eq!(cols[0], i as u64, "row number");
            KRow {
                k: cols[1],
                w: u32::try_from(cols[2]).unwrap(),
                tide: u32::try_from(cols[3]).unwrap(),
            }
        })
        .collect();
    assert_eq!(parsed.len(), K_TABLE_ROWS);
    assert_eq!(parsed, rows().collect::<Vec<_>>());
}

#[test]
fn every_row_keeps_the_emission_below_the_remaining_amount() {
    // k × m ≤ k × 2 < 1 (in units of 10^-12).
    assert!(rows().all(|r| u128::from(r.k) * 2 < K_SCALE));
}
