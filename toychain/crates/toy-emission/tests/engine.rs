//! Block-by-block emission with real timestamps (spec 5.2, 9.0).
#![allow(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use toy_emission::{BlockInput, EmissionEngine, Phase};
use toy_types::Amount;

const GENESIS: u64 = 1_800_000_000;

fn block(timestamp: u64) -> BlockInput {
    BlockInput {
        timestamp,
        counted_tx: 0,
        free_tx: 3,
        signatures_received: 64,
        signatures_expected: 64,
        phase: Phase::Bootstrap,
    }
}

/// Runs blocks with a fixed gap until `epochs` epochs ended; returns the
/// number of blocks of each epoch.
fn run(engine: &mut EmissionEngine, gap: u64, epochs: usize) -> Vec<u64> {
    let mut lengths = Vec::new();
    let mut ts = GENESIS;
    let mut in_epoch = 0;
    while lengths.len() < epochs {
        ts += gap;
        let out = engine.apply_block(&block(ts)).unwrap();
        in_epoch += 1;
        assert_eq!(out.index_in_epoch, in_epoch);
        assert!(engine.minted() <= engine.state().issued());
        if out.ends_epoch {
            assert_eq!(engine.minted(), engine.state().issued(), "I0 at epoch end");
            lengths.push(in_epoch);
            in_epoch = 0;
        }
    }
    lengths
}

#[test]
fn genesis_has_no_balances_and_no_emission() {
    let engine = EmissionEngine::genesis(GENESIS);
    assert_eq!(engine.minted(), Amount::ZERO);
    assert_eq!(engine.state().issued(), Amount::ZERO);
    assert_eq!(engine.height(), 0);
    assert!(engine.current_epoch().is_none());
}

#[test]
fn sixty_second_blocks() {
    let mut engine = EmissionEngine::genesis(GENESIS);
    let lengths = run(&mut engine, 60, 3);
    // Epoch 0 is counted from genesis; later epochs from their first block.
    assert_eq!(lengths, vec![1_440, 1_441, 1_441]);
    assert_eq!(engine.state().next_epoch(), 3);
    let first = engine.state().issued();
    // 15,821.94 + about 15,906 + about 15,990 TOY (K table, m = 1).
    assert!(first > Amount::from_toy(47_000) && first < Amount::from_toy(48_500));
}

#[test]
fn ten_second_blocks_still_give_24_hour_epochs_and_one_emission() {
    let mut engine = EmissionEngine::genesis(GENESIS);
    let lengths = run(&mut engine, 10, 1);
    assert_eq!(lengths, vec![8_640]);
    assert_eq!(
        engine.minted(),
        Amount::from_units(15_821_939_400_000_000_000_000)
    );
    assert_eq!(engine.plan_clock().seconds(), 86_400);
}

#[test]
fn start_validators_get_equal_parts_in_epoch_0() {
    let mut engine = EmissionEngine::genesis(GENESIS);
    let out = engine.apply_block(&block(GENESIS + 60)).unwrap();
    assert_eq!(out.height, 1);
    assert_eq!(out.epoch, 0);
    assert_eq!(out.mint.start_count, 64);
    // 15,821.94 TOY / 1,440 blocks × 30 % (wage + committee) / 64
    // validators = about 0.0515 TOY.
    let each = out.mint.start_each;
    assert!(each > Amount::from_units(51_400_000_000_000_000));
    assert!(each < Amount::from_units(51_600_000_000_000_000));
}

#[test]
fn invalid_block_leaves_the_engine_unchanged() {
    let mut engine = EmissionEngine::genesis(GENESIS);
    engine.apply_block(&block(GENESIS + 60)).unwrap();
    let before = engine.clone();
    assert!(engine.apply_block(&block(GENESIS + 65)).is_err());
    assert_eq!(engine, before);
}

#[test]
fn long_pauses_do_not_speed_up_plan_time() {
    let mut engine = EmissionEngine::genesis(GENESIS);
    // 1,440 blocks one hour apart: 60 days of wall-clock time.
    let mut ts = GENESIS;
    for _ in 0..1_440 {
        ts += 3_600;
        engine.apply_block(&block(ts)).unwrap();
    }
    assert_eq!(engine.plan_clock().seconds(), 1_440 * 600);
    assert_eq!(engine.plan_clock().full_days(), 10);
}
