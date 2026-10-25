//! Unit tests of the state layer. Every test that moves TOY checks rule I0
//! afterwards against an independently tracked minted sum.

#![allow(clippy::arithmetic_side_effects)]

use toy_types::params::{
    ACTIVATION_DEPOSIT, ACTIVATION_DEPOSIT_FLOOR, CLOSED_ACCOUNT_HISTORY_EPOCHS,
    DOUBLE_SIGN_HOLD_EPOCHS, STAKING_MAX_PER_EPOCH, START_REWARD_LOCK_EPOCHS, UNBONDING_EPOCHS,
    VALIDATOR_MIN_STAKE,
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

/// `n` thousandths of a TOY.
fn milli(n: u128) -> Amount {
    Amount::from_units(n * 1_000_000_000_000_000)
}

/// A mint of `total` with every part zero (fill in the parts).
fn empty_mint(total: Amount) -> Mint {
    Mint {
        total,
        start_each: Amount::ZERO,
        work_base: Amount::ZERO,
        jackpots: [Amount::ZERO; 3],
        committee: Amount::ZERO,
        full_node: Amount::ZERO,
        refund_pool: Amount::ZERO,
        staking: Amount::ZERO,
        bootstrap: false,
    }
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
        self.mint(&Mint {
            work_base: amount,
            ..empty_mint(amount)
        });
    }

    fn mint(&mut self, mint: &Mint) {
        self.state.mint_block(mint).unwrap();
        self.minted = self.minted.checked_add(mint.total).unwrap();
    }

    fn fund(&mut self, who: Address, amount: Amount) {
        self.mint_to_lottery(amount);
        self.state
            .pay_lottery_prizes(LotteryPot::Base, &[(who, amount)])
            .unwrap();
        self.i0();
    }

    fn weight(&self, who: Address) -> u128 {
        self.state.account(&who).map_or(0, Account::reward_weight)
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
fn bootstrap_mint_locks_the_start_wage_and_activates_with_deposit() {
    let mut w = World::new();
    w.next_block();
    let each = Amount::from_units(835_000_000_000_000_000);
    let start_total = each.checked_mul(21).unwrap();
    let mint = Mint {
        start_each: each,
        work_base: toy(8),
        refund_pool: toy(2),
        bootstrap: true,
        ..empty_mint(start_total.checked_add(toy(10)).unwrap())
    };
    w.mint(&mint);
    w.i0();
    for v in w.state.start_validators().to_vec() {
        let a = w.state.account(&v).unwrap();
        assert_eq!(a.deposit(), ACTIVATION_DEPOSIT);
        assert_eq!(a.free(), Amount::ZERO);
        assert_eq!(a.stake(), Amount::ZERO, "the start wage is no stake");
        assert_eq!(
            a.reward_locks(),
            &[RewardLock {
                amount: each.checked_sub(ACTIVATION_DEPOSIT).unwrap(),
                unlock_epoch: START_REWARD_LOCK_EPOCHS
            }]
        );
    }
    // A second block in the same epoch adds to the same lock entry.
    w.mint(&mint);
    w.i0();
    let a = w.state.account(&w.state.start_validators()[0]).unwrap();
    assert_eq!(a.reward_locks().len(), 1);
    // After 365 epochs the wage becomes free (decision 139: a time lock,
    // not stake).
    for _ in 0..START_REWARD_LOCK_EPOCHS {
        w.next_epoch();
    }
    let a = w.state.account(&w.state.start_validators()[0]).unwrap();
    assert!(a.reward_locks().is_empty());
    assert_eq!(a.stake(), Amount::ZERO);
    assert_eq!(
        a.free(),
        each.checked_mul(2)
            .unwrap()
            .checked_sub(ACTIVATION_DEPOSIT)
            .unwrap()
    );
    w.i0();
}

#[test]
fn small_start_wage_waits_until_it_covers_the_deposit() {
    // With 64 start validators one block pays about 0.05 TOY each, less
    // than the 0.1 TOY deposit (spec 5.3, 6.1).
    let mut w = World::new();
    let each = Amount::from_units(60_000_000_000_000_000);
    let mint = Mint {
        start_each: each,
        bootstrap: true,
        ..empty_mint(each.checked_mul(21).unwrap())
    };
    w.mint(&mint);
    w.i0();
    assert_eq!(w.state.account_count(), 0);
    let v = w.state.start_validators()[0];
    assert_eq!(w.state.start_wage_pending(&v), each);
    assert_eq!(
        w.state.pot_total(Pot::StartValidatorRewards).unwrap(),
        mint.total
    );
    // Second block: 0.12 TOY >= 0.1 TOY, the account is created.
    w.next_block();
    w.mint(&mint);
    w.i0();
    assert_eq!(w.state.account_count(), 21);
    assert_eq!(w.state.start_wage_pending(&v), Amount::ZERO);
    let a = w.state.account(&v).unwrap();
    assert_eq!(a.deposit(), ACTIVATION_DEPOSIT);
    assert_eq!(
        a.reward_locks_total().unwrap(),
        each.checked_mul(2)
            .unwrap()
            .checked_sub(ACTIVATION_DEPOSIT)
            .unwrap()
    );
}

#[test]
fn mint_parts_must_add_up() {
    let mut w = World::new();
    let before = w.state.clone();
    let bad = Mint {
        staking: toy(5),
        work_base: toy(4),
        refund_pool: toy(2),
        ..empty_mint(toy(10))
    };
    assert_eq!(w.state.mint_block(&bad), Err(StateError::PartsMismatch));
    let bad = Mint {
        jackpots: [toy(1), toy(1), toy(1)],
        committee: toy(1),
        full_node: toy(1),
        start_each: Amount::from_units(1),
        ..empty_mint(toy(5))
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
        work_base: Amount::from_units(1),
        ..empty_mint(Amount::from_units(1))
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
        staking: Amount::from_units(40),
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
    assert_eq!(w.state.staking_pot(), Amount::from_units(40));
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
    w.fund(addr(1), toy(200));
    assert!(matches!(
        w.state.register_validator(&addr(1), key),
        Err(StateError::ValidatorStakeTooLow { .. })
    ));
    w.state.bond(&addr(1), VALIDATOR_MIN_STAKE).unwrap();
    w.state.register_validator(&addr(1), key).unwrap();
    assert!(w.state.can_sign(&addr(1)));
    assert!(w.state.can_propose(&addr(1)));
    assert_eq!(w.state.validator_by_bls_key(&key), Some(addr(1)));
    assert_eq!(
        w.state.register_validator(&addr(1), key),
        Err(StateError::AlreadyValidator)
    );
    // Another address cannot take the same key.
    w.fund(addr(2), toy(200));
    w.state.bond(&addr(2), VALIDATOR_MIN_STAKE).unwrap();
    assert_eq!(
        w.state.register_validator(&addr(2), key),
        Err(StateError::BlsKeyInUse)
    );
    // A validator must keep 100 TOY (tier 7) bonded.
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
    w.fund(addr(1), toy(150));
    w.state.bond(&addr(1), VALIDATOR_MIN_STAKE).unwrap();
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
    assert_eq!(a.held_total().unwrap(), VALIDATOR_MIN_STAKE);
    assert!(!w.state.can_sign(&addr(1)));
    w.i0();
    // Paid transfers keep working during the penalty.
    w.state.transfer(&addr(1), &addr(2), toy(10)).unwrap();
    // Until M4 fixes the slashing, the held stake returns after 18 epochs.
    for _ in 0..DOUBLE_SIGN_HOLD_EPOCHS {
        w.next_epoch();
    }
    assert_eq!(
        w.state.account(&addr(1)).unwrap().stake(),
        VALIDATOR_MIN_STAKE
    );
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
    w.fund(addr(1), toy(110));
    w.state.bond(&addr(1), VALIDATOR_MIN_STAKE).unwrap();
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
    w.state.chain_mut().jackpot_due[1] = Some(9);
    assert_ne!(w.state.root(), r2);
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
    assert_eq!(active, 82, "account used in the current epoch");
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
    w.fund(addr(2), toy(200));
    w.state.bond(&addr(2), VALIDATOR_MIN_STAKE).unwrap();
    w.state
        .register_validator(&addr(2), [5; BLS_KEY_LEN])
        .unwrap();
    let before = w.state.clone();
    let big = FeeSplit {
        staking: toy(3),
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
        w.state
            .pay_lottery_prizes(LotteryPot::Base, &[(addr(1), toy(1))]),
        w.state
            .pay_lottery_prizes(LotteryPot::Funke, &[(addr(1), small)]),
        w.state.move_refund_surplus_to_staking(toy(1)),
        w.state.settle_rewards(&addr(9)).map(|_| ()),
        w.state.use_nonce(&addr(1), 12345),
        w.state.lower_activation_deposit(toy(1)),
    ];
    for (i, r) in results.iter().enumerate() {
        assert!(r.is_err(), "operation {i} should fail");
    }
    assert_eq!(w.state, before);
    w.i0();
}

// ---- Staking pot and reward counter (spec 9.4) -----------------------------

/// 20 accounts with 5 TOY (tier 2, weight 1.0) and 5 accounts with 1 TOY
/// (tier 1, saver weight 0.8): weighted stake 104 TOY, no one above the
/// 5 % cap of 105 TOY.
fn stakers(w: &mut World) {
    for n in 1..=20 {
        w.fund(addr(n), toy(6));
        w.state.bond(&addr(n), toy(5)).unwrap();
    }
    for n in 21..=25 {
        w.fund(addr(n), toy(2));
        w.state.bond(&addr(n), toy(1)).unwrap();
    }
}

fn staking_mint(amount: Amount) -> Mint {
    Mint {
        staking: amount,
        ..empty_mint(amount)
    }
}

#[test]
fn staking_rewards_follow_the_weights_of_the_epoch() {
    let mut w = World::new();
    stakers(&mut w);
    // Weights change only at epoch boundaries.
    assert_eq!(w.state.total_reward_weight(), 0);
    w.next_epoch();
    assert_eq!(w.state.total_reward_weight(), toy(104).units());
    assert_eq!(w.weight(addr(1)), toy(5).units());
    assert_eq!(w.weight(addr(21)), milli(800).units());

    let pot = milli(10_400);
    w.mint(&staking_mint(pot));
    w.i0();
    assert_eq!(w.state.pending_reward(&addr(1)).unwrap(), milli(500));
    assert_eq!(w.state.pending_reward(&addr(21)).unwrap(), milli(80));
    // Credited with the next own transaction ...
    let free = w.free(addr(1));
    let claimed = w.state.settle_rewards(&addr(1)).unwrap();
    assert_eq!(claimed, milli(500));
    assert_eq!(w.free(addr(1)), free.checked_add(claimed).unwrap());
    assert_eq!(w.state.pending_reward(&addr(1)).unwrap(), Amount::ZERO);
    assert_eq!(w.state.settle_rewards(&addr(1)).unwrap(), Amount::ZERO);
    w.i0();
    // ... or at the latest when the next epoch starts.
    let free = w.free(addr(21));
    w.next_epoch();
    assert_eq!(w.free(addr(21)), free.checked_add(milli(80)).unwrap());
    // Everything was distributed exactly: the pot is empty.
    assert_eq!(w.state.staking_pot(), Amount::ZERO);
    w.i0();
}

#[test]
fn stake_above_the_cap_counts_only_up_to_the_cap() {
    let mut w = World::new();
    stakers(&mut w);
    w.fund(addr(50), toy(101));
    w.state.bond(&addr(50), toy(100)).unwrap();
    w.next_epoch();
    // Total stake 205 TOY, fewer than 100 validators: cap 5 % = 10.25 TOY.
    assert_eq!(w.weight(addr(50)), milli(10_250).units());
    assert_eq!(w.weight(addr(1)), toy(5).units());
    // Below 1 TOY there is no weight at all.
    w.fund(addr(60), toy(1));
    w.state.bond(&addr(60), milli(900)).unwrap();
    w.next_epoch();
    assert_eq!(w.weight(addr(60)), 0);
    w.i0();
}

#[test]
fn start_validators_earn_nothing_from_staking_in_the_bootstrap_phase() {
    let mut w = World::new();
    stakers(&mut w);
    let v = w.state.start_validators()[0];
    w.fund(v, toy(6));
    w.state.bond(&v, toy(5)).unwrap();
    w.mint(&Mint {
        bootstrap: true,
        ..empty_mint(Amount::ZERO)
    });
    w.next_epoch();
    assert_eq!(w.weight(v), 0);
    assert!(w.weight(addr(1)) > 0, "stakers are paid from day 1");
    w.mint(&Mint {
        bootstrap: true,
        ..staking_mint(toy(1))
    });
    assert_eq!(w.state.pending_reward(&v).unwrap(), Amount::ZERO);
    w.i0();
    // Normal phase: the start validator is a staker like everyone else.
    w.mint(&empty_mint(Amount::ZERO));
    w.next_epoch();
    assert_eq!(w.weight(v), toy(5).units());
    w.i0();
}

#[test]
fn without_stakers_the_staking_pot_waits() {
    let mut w = World::new();
    w.mint(&staking_mint(toy(7)));
    w.i0();
    assert_eq!(w.state.staking_pot(), toy(7));
    stakers(&mut w);
    w.next_epoch();
    // The carry is paid out with the next block.
    w.mint(&staking_mint(milli(3_400)));
    assert_eq!(w.state.pending_reward(&addr(1)).unwrap(), milli(500));
    w.i0();
}

#[test]
fn staking_distribution_per_epoch_is_limited() {
    // Keeps `weight × (I_now − I_last)` far inside u128 (spec 9.4).
    let mut w = World::new();
    stakers(&mut w);
    w.next_epoch();
    let big = STAKING_MAX_PER_EPOCH.checked_add(toy(52_000)).unwrap();
    w.mint(&staking_mint(big));
    w.i0();
    let all: Amount =
        Amount::checked_sum((1..=25).map(|n| w.state.pending_reward(&addr(n)).unwrap())).unwrap();
    assert!(all <= STAKING_MAX_PER_EPOCH);
    assert!(all > STAKING_MAX_PER_EPOCH.checked_sub(toy(1)).unwrap());
    // The rest follows in the next epoch.
    w.next_epoch();
    w.mint(&staking_mint(Amount::ZERO));
    let rest: Amount =
        Amount::checked_sum((1..=25).map(|n| w.state.pending_reward(&addr(n)).unwrap())).unwrap();
    assert!(rest > toy(51_999));
    w.i0();
}

#[test]
fn fees_and_pool_surplus_feed_the_staking_pot() {
    let mut w = World::new();
    stakers(&mut w);
    w.next_epoch();
    let split = FeeSplit {
        staking: milli(1_040),
        proposer: Amount::ZERO,
        refund_pool: milli(500),
    };
    w.fund(addr(80), toy(5));
    w.state.pay_fee(&addr(80), &split, &addr(2)).unwrap();
    w.state.move_refund_surplus_to_staking(milli(500)).unwrap();
    w.i0();
    // Distributed with the next block: 1.54 TOY over 104 TOY weight.
    w.mint(&empty_mint(Amount::ZERO));
    assert_eq!(
        w.state.pending_reward(&addr(2)).unwrap(),
        // Counter step floor(1.54e18 × 1e15 ÷ 1.04e20) = 14,807,692,307,692,
        // times 5 TOY of weight ÷ 1e15.
        Amount::from_units(74_038_461_538_460_000)
    );
    w.i0();
}

#[test]
fn the_full_node_pot_goes_to_the_stakers_until_probes_exist() {
    let mut w = World::new();
    stakers(&mut w);
    w.next_epoch();
    w.mint(&Mint {
        full_node: milli(10_400),
        ..empty_mint(milli(10_400))
    });
    assert_eq!(w.state.full_node_pot(), milli(10_400));
    w.i0();
    w.next_epoch();
    assert_eq!(w.state.full_node_pot(), Amount::ZERO);
    w.mint(&empty_mint(Amount::ZERO));
    assert_eq!(w.state.pending_reward(&addr(1)).unwrap(), milli(500));
    w.i0();
}

#[test]
fn jackpots_and_committee_pot_are_kept_apart() {
    let mut w = World::new();
    w.mint(&Mint {
        work_base: toy(85),
        jackpots: [toy(8), toy(5), toy(2)],
        committee: toy(3),
        ..empty_mint(toy(103))
    });
    assert_eq!(w.state.lottery_pot(LotteryPot::Base), toy(85));
    assert_eq!(w.state.lottery_pot(LotteryPot::Funke), toy(8));
    assert_eq!(w.state.lottery_pot(LotteryPot::Welle), toy(5));
    assert_eq!(w.state.lottery_pot(LotteryPot::Gezeiten), toy(2));
    assert_eq!(w.state.committee_pot(), toy(3));
    w.i0();
    // A jackpot pays only from its own pot.
    assert_eq!(
        w.state
            .pay_lottery_prizes(LotteryPot::Gezeiten, &[(addr(1), toy(3))]),
        Err(StateError::PotTooSmall)
    );
    w.state
        .pay_lottery_prizes(
            LotteryPot::Welle,
            &[(addr(1), toy(2)), (addr(2), toy(2)), (addr(1), toy(1))],
        )
        .unwrap();
    assert_eq!(
        w.free(addr(1)),
        toy(3).checked_sub(ACTIVATION_DEPOSIT).unwrap()
    );
    assert_eq!(w.state.lottery_pot(LotteryPot::Welle), Amount::ZERO);
    w.i0();
}

#[test]
fn closing_takes_the_pending_reward_along() {
    let mut w = World::new();
    stakers(&mut w);
    w.next_epoch();
    w.mint(&staking_mint(milli(10_400)));
    // addr(1) unbonds; the weight of this epoch stays (spec 9.4).
    w.state.unbond(&addr(1), toy(5)).unwrap();
    for _ in 0..UNBONDING_EPOCHS {
        w.next_epoch();
    }
    assert_eq!(w.weight(addr(1)), 0);
    let target = w.free(addr(2));
    let total = w.free(addr(1)).checked_add(ACTIVATION_DEPOSIT).unwrap();
    w.state
        .close_account(&addr(1), Some(&addr(2)), Amount::ZERO)
        .unwrap();
    assert_eq!(w.free(addr(2)), target.checked_add(total).unwrap());
    w.i0();
}

/// Size test for the new account fields: an account without weight stores
/// no reward counter.
#[test]
fn reward_counter_costs_space_only_for_stakers() {
    let mut w = World::new();
    stakers(&mut w);
    w.fund(addr(70), toy(1));
    w.next_epoch();
    w.mint(&staking_mint(toy(1)));
    w.next_epoch();
    let plain = w.state.account(&addr(70)).unwrap().encode().len();
    assert_eq!(plain, 49);
    let staker = w.state.account(&addr(1)).unwrap().encode().len();
    assert_eq!(staker, 49 + 16 + 32, "stake plus weight and mark");
    assert!(staker + 64 <= 200);
}

/// Size test for the lottery history in the rule data (CLAUDE.md: every
/// storage structure needs one; spec 6: 256 MB per node).
#[test]
fn lottery_history_fits_the_data_budget() {
    // Worst case of tickets in one epoch: 24 hours of blocks at the 10 s
    // minimum gap carry at least 8 tickets each (8,640 × 8 = 69,120);
    // from 15 s on the time-scaled limit gives 46,080 per day.
    const MAX_TICKETS_PER_EPOCH: usize = 8_640 * 8;
    let mut chain = ChainData::genesis(Hash::default());
    let owners = vec![addr(1); MAX_TICKETS_PER_EPOCH];
    chain.epoch_ticket_owners.clone_from(&owners);
    chain.prev_epoch_ticket_owners = owners;
    for h in 0..62u64 {
        chain.recent_tickets.push_back((h, vec![addr(2); 32]));
    }
    for h in 0..3u64 {
        chain
            .pending_draws
            .insert(h, vec![(Hash::default(), addr(3)); 32]);
    }
    for _ in 0..364 {
        chain.minted_history.push_back(toy(50_000));
    }
    let lists =
        (chain.epoch_ticket_owners.len() + chain.prev_epoch_ticket_owners.len()) * Address::LEN;
    assert_eq!(lists, 2_764_800, "about 2.8 MB, 1.1 % of 256 MB");
    // The state root binds the long lists by digest, so the encoding per
    // block stays small.
    let encoded = chain.encode().len();
    assert!(encoded < 64 * 1024, "{encoded}");
}

/// Bauplan M3b: splitting stake over many accounts of the same weight
/// brings nothing. 100 accounts with 5 TOY each earn, up to rounding,
/// exactly what one account with 500 TOY earns.
#[test]
fn splitting_stake_over_accounts_brings_nothing() {
    let mut w = World::new();
    // Background stake keeps the 5 % cap far away (total 11,000 TOY).
    for n in 1..=20 {
        w.fund(addr(n), toy(501));
        w.state.bond(&addr(n), toy(500)).unwrap();
    }
    let single = addr(30);
    w.fund(single, toy(501));
    w.state.bond(&single, toy(500)).unwrap();
    let many: Vec<Address> = (0..100u8)
        .map(|i| {
            let mut b = [0u8; 20];
            b[0] = 0xbb;
            b[19] = i;
            Address::from_bytes(b)
        })
        .collect();
    for a in &many {
        w.fund(*a, toy(6));
        w.state.bond(a, toy(5)).unwrap();
    }
    w.next_epoch();
    for _ in 0..1_440 {
        w.mint(&staking_mint(Amount::from_units(7_777_777_777_777_777)));
    }
    let one = w.state.pending_reward(&single).unwrap();
    let split =
        Amount::checked_sum(many.iter().map(|a| w.state.pending_reward(a).unwrap())).unwrap();
    // 500 of 11,000 TOY weight: about 0.509 of 11.2 TOY.
    assert!(one > milli(500), "{one:?}");
    // Each of the 100 accounts loses less than one unit per block.
    let diff = one.units().abs_diff(split.units());
    assert!(diff <= 100 * 1_440, "{diff}");
    w.i0();
}
