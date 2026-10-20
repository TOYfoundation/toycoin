//! Unit tests of the state layer. Every test that moves TOY checks rule I0
//! afterwards against an independently tracked minted sum.

#![allow(clippy::arithmetic_side_effects)]

use toy_types::params::{
    ACTIVATION_DEPOSIT, ACTIVATION_DEPOSIT_FLOOR, CLOSED_ACCOUNT_HISTORY_EPOCHS,
    DOUBLE_SIGN_HOLD_EPOCHS, STAKE_TIER_4, START_REWARD_LOCK_EPOCHS, UNBONDING_EPOCHS,
};
use toy_types::{Address, Amount, Hash};

use crate::*;

fn addr(n: u8) -> Address {
    let mut b = [0u8; 20];
    b[0] = 0xaa;
    b[19] = n;
    Address::from_bytes(b)
}

fn toy(n: u64) -> Amount {
    Amount::from_toy(n)
}

/// A state with a running minted counter, funded through the work
/// lottery (the only way TOY reaches accounts besides rewards).
struct World {
    state: State,
    minted: Amount,
}

impl World {
    fn new() -> Self {
        let config = GenesisConfig {
            start_validators: (100..121).map(addr).collect(),
            genesis_randomness: Hash::default(),
        };
        Self {
            state: State::genesis(&config).unwrap(),
            minted: Amount::ZERO,
        }
    }

    fn next_block(&mut self) {
        let h = self.state.height() + 1;
        let e = self.state.epoch();
        self.state.begin_block(h, e).unwrap();
    }

    fn next_epoch(&mut self) {
        let h = self.state.height() + 1;
        let e = self.state.epoch() + 1;
        self.state.begin_block(h, e).unwrap();
    }

    fn mint_to_lottery(&mut self, amount: Amount) {
        self.state
            .mint_block(&Mint {
                total: amount,
                consensus: ConsensusMint::Stakers(Amount::ZERO),
                work_lottery: amount,
                refund_pool: Amount::ZERO,
            })
            .unwrap();
        self.minted = self.minted.checked_add(amount).unwrap();
    }

    fn fund(&mut self, who: Address, amount: Amount) {
        self.mint_to_lottery(amount);
        self.state.pay_lottery_prizes(&[(who, amount)]).unwrap();
        self.i0();
    }

    fn i0(&self) {
        self.state.check_i0(self.minted).unwrap();
    }

    fn free(&self, who: Address) -> Amount {
        self.state.account(&who).map_or(Amount::ZERO, Account::free)
    }
}

#[test]
fn genesis_has_no_balances_and_i0_holds_with_zero() {
    let w = World::new();
    assert_eq!(w.state.account_count(), 0);
    assert_eq!(w.state.pot_totals().unwrap().sum().unwrap(), Amount::ZERO);
    for (pot, amount) in w.state.pot_totals().unwrap().entries() {
        assert!(amount.is_zero(), "{pot:?}");
    }
    w.i0();
    assert_eq!(w.state.start_validators().len(), 21);
}

#[test]
fn duplicate_start_validators_are_rejected() {
    let config = GenesisConfig {
        start_validators: vec![addr(1), addr(2), addr(1)],
        genesis_randomness: Hash::default(),
    };
    assert_eq!(
        State::genesis(&config),
        Err(StateError::DuplicateStartValidator)
    );
}

#[test]
fn pot_list_is_complete() {
    let w = World::new();
    let totals = w.state.pot_totals().unwrap();
    assert_eq!(totals.entries().len(), Pot::ALL.len());
    for (i, (pot, _)) in totals.entries().iter().enumerate() {
        assert_eq!(*pot, Pot::ALL[i]);
    }
}

#[test]
fn i0_detects_tampering() {
    let mut w = World::new();
    w.fund(addr(1), toy(5));
    w.state.tamper_free_balance(&addr(1), toy(6));
    assert_eq!(
        w.state.check_i0(w.minted),
        Err(StateError::I0Violated {
            // 6 TOY free plus the 0.1 TOY deposit.
            pots: toy(6).checked_add(ACTIVATION_DEPOSIT).unwrap(),
            minted: toy(5)
        })
    );
}

#[test]
fn bootstrap_mint_pays_bonded_rewards_and_activates_with_deposit() {
    let mut w = World::new();
    w.next_block();
    let each = Amount::from_units(835_000_000_000_000_000);
    let consensus = each.checked_mul(21).unwrap();
    let mint = Mint {
        total: consensus.checked_add(toy(10)).unwrap(),
        consensus: ConsensusMint::StartValidators { each },
        work_lottery: toy(8),
        refund_pool: toy(2),
    };
    w.state.mint_block(&mint).unwrap();
    w.minted = mint.total;
    w.i0();
    for v in w.state.start_validators().to_vec() {
        let a = w.state.account(&v).unwrap();
        assert_eq!(a.deposit(), ACTIVATION_DEPOSIT);
        assert_eq!(a.free(), Amount::ZERO);
        assert_eq!(
            a.reward_locks(),
            &[RewardLock {
                amount: each.checked_sub(ACTIVATION_DEPOSIT).unwrap(),
                unlock_epoch: START_REWARD_LOCK_EPOCHS
            }]
        );
    }
    // A second block in the same epoch adds to the same lock entry.
    w.state.mint_block(&mint).unwrap();
    w.minted = w.minted.checked_add(mint.total).unwrap();
    w.i0();
    let a = w.state.account(&w.state.start_validators()[0]).unwrap();
    assert_eq!(a.reward_locks().len(), 1);
    // After 365 epochs the reward becomes normal stake (not free).
    for _ in 0..START_REWARD_LOCK_EPOCHS {
        w.next_epoch();
    }
    let a = w.state.account(&w.state.start_validators()[0]).unwrap();
    assert!(a.reward_locks().is_empty());
    assert_eq!(
        a.stake(),
        each.checked_mul(2)
            .unwrap()
            .checked_sub(ACTIVATION_DEPOSIT)
            .unwrap()
    );
    assert_eq!(a.free(), Amount::ZERO);
    w.i0();
}

#[test]
fn reward_too_small_to_activate_goes_to_work_lottery() {
    let mut w = World::new();
    let each = Amount::from_units(1_000);
    let mint = Mint {
        total: each.checked_mul(21).unwrap(),
        consensus: ConsensusMint::StartValidators { each },
        work_lottery: Amount::ZERO,
        refund_pool: Amount::ZERO,
    };
    w.state.mint_block(&mint).unwrap();
    w.minted = mint.total;
    w.i0();
    assert_eq!(w.state.account_count(), 0);
    assert_eq!(w.state.work_lottery(), mint.total);
}

#[test]
fn mint_parts_must_add_up() {
    let mut w = World::new();
    let before = w.state.clone();
    let bad = Mint {
        total: toy(10),
        consensus: ConsensusMint::Stakers(toy(5)),
        work_lottery: toy(4),
        refund_pool: toy(2),
    };
    assert_eq!(w.state.mint_block(&bad), Err(StateError::PartsMismatch));
    assert_eq!(w.state, before);
}

#[test]
fn activation_takes_deposit_and_rejects_small_first_transfer() {
    let mut w = World::new();
    w.fund(addr(1), toy(10));
    let a = w.state.account(&addr(1)).unwrap();
    assert_eq!(a.deposit(), ACTIVATION_DEPOSIT);
    assert_eq!(a.free(), toy(10).checked_sub(ACTIVATION_DEPOSIT).unwrap());

    // Below the deposit to a new address: rejected, nothing changes.
    let before = w.state.clone();
    let small = ACTIVATION_DEPOSIT
        .checked_sub(Amount::from_units(1))
        .unwrap();
    assert_eq!(
        w.state.transfer(&addr(1), &addr(2), small),
        Err(StateError::BelowActivationDeposit {
            amount: small,
            required: ACTIVATION_DEPOSIT
        })
    );
    assert_eq!(w.state, before);

    // Exactly the deposit: a valid account with free balance 0.
    let out = w
        .state
        .transfer(&addr(1), &addr(2), ACTIVATION_DEPOSIT)
        .unwrap();
    assert!(out.activated);
    assert_eq!(w.free(addr(2)), Amount::ZERO);
    // Any amount to an existing account is fine.
    w.state
        .transfer(&addr(1), &addr(2), Amount::from_units(1))
        .unwrap();
    w.i0();
}

#[test]
fn free_balance_can_reach_zero_without_losing_the_account() {
    let mut w = World::new();
    w.fund(addr(1), toy(1));
    let all = w.free(addr(1));
    w.state.transfer(&addr(1), &addr(2), all).unwrap();
    let a = w.state.account(&addr(1)).unwrap();
    assert_eq!(a.free(), Amount::ZERO);
    assert_eq!(a.deposit(), ACTIVATION_DEPOSIT);
    w.i0();
}

#[test]
fn double_spend_and_overflow_are_rejected_without_change() {
    let mut w = World::new();
    w.fund(addr(1), toy(2));
    let free = w.free(addr(1));
    let before = w.state.clone();
    // More than the free balance (deposit is not spendable).
    let too_much = free.checked_add(Amount::from_units(1)).unwrap();
    assert!(matches!(
        w.state.transfer(&addr(1), &addr(2), too_much),
        Err(StateError::InsufficientFunds { .. })
    ));
    assert!(matches!(
        w.state
            .transfer(&addr(1), &addr(2), Amount::from_units(u128::MAX)),
        Err(StateError::InsufficientFunds { .. })
    ));
    assert_eq!(w.state, before);

    // Overflow of a pot: rejected, unchanged.
    let mut w = World::new();
    w.mint_to_lottery(Amount::from_units(u128::MAX));
    let before = w.state.clone();
    let more = Mint {
        total: Amount::from_units(1),
        consensus: ConsensusMint::Stakers(Amount::ZERO),
        work_lottery: Amount::from_units(1),
        refund_pool: Amount::ZERO,
    };
    assert!(matches!(
        w.state.mint_block(&more),
        Err(StateError::Arithmetic(_))
    ));
    assert_eq!(w.state, before);
}

#[test]
fn nonce_must_match_and_new_accounts_start_at_height_times_2_pow_16() {
    let mut w = World::new();
    for _ in 0..7 {
        w.next_block();
    }
    w.fund(addr(1), toy(1));
    let base = 7u64 << 16;
    assert_eq!(w.state.account(&addr(1)).unwrap().nonce(), base);
    assert_eq!(
        w.state.use_nonce(&addr(1), base + 1),
        Err(StateError::NonceMismatch {
            expected: base,
            found: base + 1
        })
    );
    w.state.use_nonce(&addr(1), base).unwrap();
    // Reuse of the same nonce (replay) fails.
    assert!(w.state.use_nonce(&addr(1), base).is_err());
}

#[test]
fn fee_split_and_proposer_without_account() {
    let mut w = World::new();
    w.fund(addr(1), toy(10));
    let split = FeeSplit {
        fee_lottery: Amount::from_units(40),
        proposer: Amount::from_units(20),
        refund_pool: Amount::from_units(41),
    };
    let before_free = w.free(addr(1));
    // Proposer share too small to activate: goes to the refund pool.
    w.state.pay_fee(&addr(1), &split, &addr(9)).unwrap();
    assert_eq!(
        w.free(addr(1)),
        before_free.checked_sub(Amount::from_units(101)).unwrap()
    );
    assert_eq!(w.state.fee_lottery(), Amount::from_units(40));
    assert_eq!(w.state.refund_pool(), Amount::from_units(61));
    assert!(w.state.account(&addr(9)).is_none());
    w.i0();
    // Proposer with account: gets its share.
    w.fund(addr(9), toy(1));
    let p_before = w.free(addr(9));
    w.state.pay_fee(&addr(1), &split, &addr(9)).unwrap();
    assert_eq!(
        w.free(addr(9)),
        p_before.checked_add(Amount::from_units(20)).unwrap()
    );
    w.i0();
}

#[test]
fn donation_lands_exactly_in_the_pool() {
    let mut w = World::new();
    w.fund(addr(1), toy(10));
    let pool = w.state.refund_pool();
    w.state.donate(&addr(1), toy(3)).unwrap();
    assert_eq!(w.state.refund_pool(), pool.checked_add(toy(3)).unwrap());
    w.i0();
    // Donating more than the free balance fails.
    assert!(w.state.donate(&addr(1), toy(100)).is_err());
}

#[test]
fn bond_unbond_and_release_after_21_epochs() {
    let mut w = World::new();
    w.fund(addr(1), toy(50));
    w.state.bond(&addr(1), toy(20)).unwrap();
    assert_eq!(w.state.account(&addr(1)).unwrap().stake(), toy(20));
    w.i0();
    w.state.unbond(&addr(1), toy(5)).unwrap();
    assert!(matches!(
        w.state.unbond(&addr(1), toy(16)),
        Err(StateError::InsufficientStake { .. })
    ));
    let free = w.free(addr(1));
    for _ in 0..UNBONDING_EPOCHS - 1 {
        w.next_epoch();
    }
    assert_eq!(w.free(addr(1)), free, "still unbonding");
    w.next_epoch();
    assert_eq!(w.free(addr(1)), free.checked_add(toy(5)).unwrap());
    assert!(w.state.account(&addr(1)).unwrap().unbonding().is_empty());
    w.i0();
}

#[test]
fn validator_registration_rules() {
    let mut w = World::new();
    let key = [7u8; BLS_KEY_LEN];
    w.fund(addr(1), toy(2_000));
    assert!(matches!(
        w.state.register_validator(&addr(1), key),
        Err(StateError::ValidatorStakeTooLow { .. })
    ));
    w.state.bond(&addr(1), STAKE_TIER_4).unwrap();
    w.state.register_validator(&addr(1), key).unwrap();
    assert!(w.state.can_sign(&addr(1)));
    assert!(w.state.can_propose(&addr(1)));
    assert_eq!(w.state.validator_by_bls_key(&key), Some(addr(1)));
    assert_eq!(
        w.state.register_validator(&addr(1), key),
        Err(StateError::AlreadyValidator)
    );
    // Another address cannot take the same key.
    w.fund(addr(2), toy(2_000));
    w.state.bond(&addr(2), STAKE_TIER_4).unwrap();
    assert_eq!(
        w.state.register_validator(&addr(2), key),
        Err(StateError::BlsKeyInUse)
    );
    // A validator must keep 1,000 TOY bonded.
    assert!(matches!(
        w.state.unbond(&addr(1), toy(1)),
        Err(StateError::ValidatorStakeTooLow { .. })
    ));
    w.state.deregister_validator(&addr(1)).unwrap();
    assert!(!w.state.can_sign(&addr(1)));
    w.state.unbond(&addr(1), toy(1)).unwrap();
    w.i0();
}

#[test]
fn closing_rules_with_and_without_donation() {
    let mut w = World::new();
    w.fund(addr(1), toy(10));
    w.fund(addr(2), toy(1));
    let total = toy(10);

    // With stake it cannot be closed.
    w.state.bond(&addr(1), toy(1)).unwrap();
    assert_eq!(
        w.state
            .close_account(&addr(1), Some(&addr(2)), Amount::ZERO),
        Err(StateError::CannotClose(CloseBlocker::Stake))
    );
    w.state.unbond(&addr(1), toy(1)).unwrap();
    assert_eq!(
        w.state
            .close_account(&addr(1), Some(&addr(2)), Amount::ZERO),
        Err(StateError::CannotClose(CloseBlocker::Unbonding))
    );
    for _ in 0..UNBONDING_EPOCHS {
        w.next_epoch();
    }

    // Target needed, not itself; donation not above the total.
    assert_eq!(
        w.state.close_account(&addr(1), None, Amount::ZERO),
        Err(StateError::MissingCloseTarget)
    );
    assert_eq!(
        w.state
            .close_account(&addr(1), Some(&addr(1)), Amount::ZERO),
        Err(StateError::CloseTargetIsSelf)
    );
    assert_eq!(
        w.state.close_account(&addr(1), Some(&addr(2)), toy(11)),
        Err(StateError::DonationTooLarge)
    );

    // Partial donation: free balance plus deposit split exactly.
    let pool = w.state.refund_pool();
    let b2 = w.free(addr(2));
    w.state
        .close_account(&addr(1), Some(&addr(2)), toy(4))
        .unwrap();
    assert!(w.state.account(&addr(1)).is_none());
    assert_eq!(w.state.refund_pool(), pool.checked_add(toy(4)).unwrap());
    assert_eq!(w.free(addr(2)), b2.checked_add(toy(6)).unwrap());
    let _ = total;
    w.i0();

    // Full donation needs no target.
    w.fund(addr(3), toy(2));
    let pool = w.state.refund_pool();
    w.state.close_account(&addr(3), None, toy(2)).unwrap();
    assert_eq!(w.state.refund_pool(), pool.checked_add(toy(2)).unwrap());
    w.i0();
}

#[test]
fn closed_account_keeps_penalty_history_for_64_epochs() {
    let mut w = World::new();
    w.fund(addr(1), toy(1));
    w.state
        .apply_violation(&addr(1), Violation::MassAccountCreation)
        .unwrap();
    let all = w.free(addr(1));
    w.state.transfer(&addr(1), &addr(2), all).unwrap();
    w.state
        .close_account(&addr(1), Some(&addr(2)), Amount::ZERO)
        .unwrap();
    assert!(w.state.detached_record(&addr(1)).is_some());
    assert!(w.state.is_locked(&addr(1), Right::FreeQuota));

    // Re-opening within 64 epochs brings the history back.
    w.next_epoch();
    w.fund(addr(1), toy(1));
    let p = w.state.account(&addr(1)).unwrap().penalties();
    assert_eq!(p.strikes(), 1);
    assert!(w.state.detached_record(&addr(1)).is_none());

    // Closed again and left alone: pruned after 64 epochs.
    let all = w.free(addr(1));
    w.state.transfer(&addr(1), &addr(2), all).unwrap();
    w.state
        .close_account(&addr(1), Some(&addr(2)), Amount::ZERO)
        .unwrap();
    for _ in 0..CLOSED_ACCOUNT_HISTORY_EPOCHS {
        w.next_epoch();
    }
    assert!(w.state.detached_record(&addr(1)).is_some());
    w.next_epoch();
    assert!(w.state.detached_record(&addr(1)).is_none());
    w.i0();
}

#[test]
fn reopened_account_gets_a_higher_nonce_base() {
    let mut w = World::new();
    w.next_block();
    w.fund(addr(1), toy(1));
    let first_base = w.state.account(&addr(1)).unwrap().nonce();
    w.state.use_nonce(&addr(1), first_base).unwrap();
    let all = w.free(addr(1));
    w.state.transfer(&addr(1), &addr(2), all).unwrap();
    w.state
        .close_account(&addr(1), Some(&addr(2)), Amount::ZERO)
        .unwrap();
    w.next_block();
    w.fund(addr(1), toy(1));
    let second_base = w.state.account(&addr(1)).unwrap().nonce();
    assert!(second_base > first_base + 1);
    // The old nonce can never match again.
    assert!(w.state.use_nonce(&addr(1), first_base).is_err());
}

#[test]
fn lowering_the_deposit_frees_the_difference() {
    let mut w = World::new();
    w.fund(addr(1), toy(1));
    assert_eq!(
        w.state.lower_activation_deposit(toy(1)),
        Err(StateError::InvalidDepositChange)
    );
    assert_eq!(
        w.state.lower_activation_deposit(
            ACTIVATION_DEPOSIT_FLOOR
                .checked_sub(Amount::from_units(1))
                .unwrap()
        ),
        Err(StateError::InvalidDepositChange)
    );
    w.state
        .lower_activation_deposit(ACTIVATION_DEPOSIT_FLOOR)
        .unwrap();
    let free = w.free(addr(1));
    let freed = w.state.release_excess_deposit(&addr(1)).unwrap();
    assert_eq!(
        freed,
        ACTIVATION_DEPOSIT
            .checked_sub(ACTIVATION_DEPOSIT_FLOOR)
            .unwrap()
    );
    assert_eq!(w.free(addr(1)), free.checked_add(freed).unwrap());
    assert_eq!(
        w.state.account(&addr(1)).unwrap().deposit(),
        ACTIVATION_DEPOSIT_FLOOR
    );
    // Releasing twice frees nothing more.
    assert_eq!(
        w.state.release_excess_deposit(&addr(1)).unwrap(),
        Amount::ZERO
    );
    // New accounts pay only the new deposit.
    w.state
        .transfer(&addr(1), &addr(2), ACTIVATION_DEPOSIT_FLOOR)
        .unwrap();
    w.i0();
}

#[test]
fn double_sign_holds_stake_but_never_touches_free_balance() {
    let mut w = World::new();
    w.fund(addr(1), toy(1_500));
    w.state.bond(&addr(1), STAKE_TIER_4).unwrap();
    w.state
        .register_validator(&addr(1), [1; BLS_KEY_LEN])
        .unwrap();
    w.state.unbond(&addr(1), Amount::ZERO).unwrap();
    let free = w.free(addr(1));
    let out = w
        .state
        .apply_violation(&addr(1), Violation::DoubleSign)
        .unwrap();
    assert_eq!(out.epochs, Some(7));
    let a = w.state.account(&addr(1)).unwrap();
    assert_eq!(a.free(), free, "free balance untouched");
    assert_eq!(a.deposit(), ACTIVATION_DEPOSIT, "deposit untouched");
    assert_eq!(a.stake(), Amount::ZERO);
    assert_eq!(a.held_total().unwrap(), STAKE_TIER_4);
    assert!(!w.state.can_sign(&addr(1)));
    w.i0();
    // Paid transfers keep working during the penalty.
    w.state.transfer(&addr(1), &addr(2), toy(10)).unwrap();
    // Until M4 fixes the slashing, the held stake returns after 18 epochs.
    for _ in 0..DOUBLE_SIGN_HOLD_EPOCHS {
        w.next_epoch();
    }
    assert_eq!(w.state.account(&addr(1)).unwrap().stake(), STAKE_TIER_4);
    w.i0();
}

#[test]
fn penalties_for_addresses_without_account_are_kept() {
    let mut w = World::new();
    w.state
        .apply_violation(&addr(5), Violation::InvalidTicket)
        .unwrap();
    assert!(w.state.is_locked(&addr(5), Right::Tickets));
    w.next_epoch();
    assert!(w.state.is_locked(&addr(5), Right::Tickets));
    w.next_epoch();
    assert!(!w.state.is_locked(&addr(5), Right::Tickets));
    // The record survives for escalation until it is old enough.
    assert!(w.state.detached_record(&addr(5)).is_some());
}

#[test]
fn no_penalty_changes_any_balance() {
    let mut w = World::new();
    w.fund(addr(1), toy(100));
    for v in Violation::ALL {
        let before: Vec<Amount> = Pot::ALL
            .iter()
            .map(|p| w.state.pot_total(*p).unwrap())
            .collect();
        let free = w.free(addr(1));
        w.state.apply_violation(&addr(1), v).unwrap();
        assert_eq!(w.free(addr(1)), free, "{v:?}");
        let after: Vec<Amount> = Pot::ALL
            .iter()
            .map(|p| w.state.pot_total(*p).unwrap())
            .collect();
        // addr(1) has no stake, so even a double signature moves nothing.
        assert_eq!(before, after, "{v:?}");
        w.i0();
    }
}

#[test]
fn inactivity_pause_ends_when_the_node_reports_back() {
    let mut w = World::new();
    let key = [3u8; BLS_KEY_LEN];
    w.fund(addr(1), toy(1_100));
    w.state.bond(&addr(1), STAKE_TIER_4).unwrap();
    w.state.register_validator(&addr(1), key).unwrap();
    w.state
        .apply_violation(&addr(1), Violation::Inactivity)
        .unwrap();
    assert!(!w.state.can_sign(&addr(1)));
    w.state.register_validator(&addr(1), key).unwrap();
    assert!(w.state.can_sign(&addr(1)));
}

#[test]
fn blocks_and_epochs_must_be_in_order() {
    let mut w = World::new();
    assert!(matches!(
        w.state.begin_block(2, 0),
        Err(StateError::UnexpectedHeight { .. })
    ));
    assert!(matches!(
        w.state.begin_block(1, 2),
        Err(StateError::UnexpectedEpoch { .. })
    ));
    w.state.begin_block(1, 0).unwrap();
}

#[test]
fn root_changes_with_every_part_of_the_state() {
    let mut w = World::new();
    let r0 = w.state.root();
    w.fund(addr(1), toy(1));
    let r1 = w.state.root();
    assert_ne!(r0, r1);
    w.state.chain_mut().epoch_tickets = 1;
    let r2 = w.state.root();
    assert_ne!(r1, r2);
    w.state
        .apply_violation(&addr(7), Violation::InvalidTicket)
        .unwrap();
    assert_ne!(w.state.root(), r2);
    // Same state, same root.
    assert_eq!(w.state.clone().root(), w.state.root());
}

/// Size test (CLAUDE.md: every storage structure needs one). Spec 6
/// assumes about 100 to 200 bytes per account including the tree.
#[test]
fn account_encoding_size() {
    let mut w = World::new();
    w.fund(addr(1), toy(1));
    let plain = w.state.account(&addr(1)).unwrap().encode().len();
    assert_eq!(plain, 49, "plain account");
    w.state
        .update_activity(&addr(1), |a| {
            a.tx_count = 1;
            Ok(())
        })
        .unwrap();
    let active = w.state.account(&addr(1)).unwrap().encode().len();
    assert_eq!(active, 90, "account used in the current epoch");
    // Plus about 32 to 64 bytes of Merkle tree per account: within the
    // 100 to 200 bytes of spec 6. M5 measures the real tree.
    assert!(active + 64 <= 200);
}

/// "Check first, then change": every failing operation leaves the state
/// exactly as it was (no copy is made any more, see `state.rs`).
#[test]
fn every_failing_operation_leaves_the_state_unchanged() {
    let mut w = World::new();
    w.fund(addr(1), toy(5));
    w.fund(addr(2), toy(2_000));
    w.state.bond(&addr(2), STAKE_TIER_4).unwrap();
    w.state
        .register_validator(&addr(2), [5; BLS_KEY_LEN])
        .unwrap();
    let before = w.state.clone();
    let big = FeeSplit {
        fee_lottery: toy(3),
        proposer: toy(3),
        refund_pool: toy(3),
    };
    let small = Amount::from_units(1);
    let results: Vec<Result<(), StateError>> = vec![
        w.state.pay_fee(&addr(1), &big, &addr(9)).map(|_| ()),
        w.state.transfer(&addr(1), &addr(9), small).map(|_| ()),
        w.state.transfer(&addr(9), &addr(1), small).map(|_| ()),
        w.state.donate(&addr(1), toy(6)),
        w.state.bond(&addr(1), toy(6)),
        w.state.unbond(&addr(1), small),
        w.state.unbond(&addr(2), small),
        w.state.register_validator(&addr(1), [6; BLS_KEY_LEN]),
        w.state.register_validator(&addr(2), [6; BLS_KEY_LEN]),
        w.state.deregister_validator(&addr(1)),
        w.state
            .close_account(&addr(2), Some(&addr(1)), Amount::ZERO)
            .map(|_| ()),
        w.state
            .close_account(&addr(1), None, Amount::ZERO)
            .map(|_| ()),
        w.state
            .close_account(&addr(1), Some(&addr(9)), toy(6))
            .map(|_| ()),
        w.state
            .close_account(&addr(1), Some(&addr(1)), Amount::ZERO)
            .map(|_| ()),
        w.state.pay_lottery_prizes(&[(addr(1), toy(1))]),
        w.state.pay_lottery_prizes(&[(addr(9), small)]),
        w.state.move_refund_surplus_to_lottery(toy(1)),
        w.state.use_nonce(&addr(1), 12345),
        w.state.lower_activation_deposit(toy(1)),
    ];
    for (i, r) in results.iter().enumerate() {
        assert!(r.is_err(), "operation {i} should fail");
    }
    assert_eq!(w.state, before);
    w.i0();
}
