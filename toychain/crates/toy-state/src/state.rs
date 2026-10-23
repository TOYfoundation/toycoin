//! The account state and every operation that moves TOY.
//!
//! Design rule: TOY is created only by [`State::mint_block`] (from the
//! emission) and otherwise only **moved** between pots. Every method
//! below either debits exactly what it credits or fails without changing
//! anything. [`State::check_i0`] verifies rule I0 after each block.
//!
//! Constitution (spec 13 no. 8): no method takes TOY from an account
//! without its owner's signed transaction. Penalties ([`State::apply_violation`])
//! only remove rights; after a double signature the stake the owner bonded
//! for a role is *held* (still the owner's, still counted by I0) until the
//! slashing amount is fixed (spec 5.4, M4).

use std::collections::BTreeMap;

use toy_types::params::{
    ACTIVATION_DEPOSIT, ACTIVATION_DEPOSIT_FLOOR, CLOSED_ACCOUNT_HISTORY_EPOCHS,
    DOUBLE_SIGN_HOLD_EPOCHS, NONCE_BASE_SHIFT, STAKE_TIER_4, START_REWARD_LOCK_EPOCHS,
    UNBONDING_EPOCHS,
};
use toy_types::{Address, Amount, ArithmeticError, EpochNumber, Hash, Height};

use crate::account::{Account, BLS_KEY_LEN, EpochActivity, HeldStake, RewardLock, Unbonding};
use crate::chain::ChainData;
use crate::error::{CloseBlocker, StateError};
use crate::merkle::{leaf_hash, merkle_root};
use crate::penalty::{PenaltyOutcome, PenaltyRecord, Right, Violation};
use crate::pots::{Pot, PotTotals};

/// Configuration of the genesis block. It has **no field for balances**:
/// the genesis block holds no TOY (total fair launch, decision 62,
/// constitution no. 2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenesisConfig {
    /// Addresses of the start validators (spec 5.3: 21), which receive the
    /// consensus share during the bootstrap phase.
    pub start_validators: Vec<Address>,
    /// Randomness of the genesis block (seed of epoch 0).
    pub genesis_randomness: Hash,
}

/// How the consensus share of a block is minted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsensusMint {
    /// Bootstrap phase: every start validator receives `each`, bonded for
    /// 365 epochs.
    StartValidators {
        /// Amount per start validator.
        each: Amount,
    },
    /// Normal phase: the pot waits for distribution by consensus (M4).
    Stakers(Amount),
}

/// Newly minted TOY of one block, as computed by `toy-emission`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mint {
    /// Total minted in the block.
    pub total: Amount,
    /// Consensus share.
    pub consensus: ConsensusMint,
    /// Work lottery share.
    pub work_lottery: Amount,
    /// Refund pool share (with rounding remainders).
    pub refund_pool: Amount,
}

/// How a paid fee is split (computed by `toy-tx`, spec 7.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeeSplit {
    /// Share of the fee lottery.
    pub fee_lottery: Amount,
    /// Share of the block proposer.
    pub proposer: Amount,
    /// Share of the refund pool (with rounding remainders).
    pub refund_pool: Amount,
}

impl FeeSplit {
    /// Sum of the parts.
    ///
    /// # Errors
    /// [`ArithmeticError::Overflow`] on overflow.
    pub fn total(&self) -> Result<Amount, ArithmeticError> {
        Amount::checked_sum([self.fee_lottery, self.proposer, self.refund_pool])
    }
}

/// Result of a credit to an address.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CreditOutcome {
    /// `true` if the credit created (activated) the account.
    pub activated: bool,
}

/// Penalty record of an address without an active account: a closed
/// account (kept 64 epochs, spec 6.1) or a device without account that
/// sent an invalid ticket.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetachedRecord {
    /// The penalty record.
    pub record: PenaltyRecord,
    /// Last epoch in which the record changed or the account was closed.
    pub since_epoch: EpochNumber,
}

/// The state of ToyChain after a block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    accounts: BTreeMap<Address, Account>,
    detached: BTreeMap<Address, DetachedRecord>,
    bls_keys: BTreeMap<[u8; BLS_KEY_LEN], Address>,
    start_validators: Vec<Address>,
    refund_pool: Amount,
    work_lottery: Amount,
    fee_lottery: Amount,
    consensus_rewards: Amount,
    activation_deposit: Amount,
    height: Height,
    epoch: EpochNumber,
    chain: ChainData,
}

impl State {
    /// The genesis state: height 0, epoch 0, **no balances at all**.
    ///
    /// # Errors
    /// [`StateError::DuplicateStartValidator`] if an address repeats.
    pub fn genesis(config: &GenesisConfig) -> Result<Self, StateError> {
        let mut sorted = config.start_validators.clone();
        sorted.sort_unstable();
        if sorted.windows(2).any(|w| w.first() == w.get(1)) {
            return Err(StateError::DuplicateStartValidator);
        }
        Ok(Self {
            accounts: BTreeMap::new(),
            detached: BTreeMap::new(),
            bls_keys: BTreeMap::new(),
            start_validators: config.start_validators.clone(),
            refund_pool: Amount::ZERO,
            work_lottery: Amount::ZERO,
            fee_lottery: Amount::ZERO,
            consensus_rewards: Amount::ZERO,
            activation_deposit: ACTIVATION_DEPOSIT,
            height: 0,
            epoch: 0,
            chain: ChainData::genesis(config.genesis_randomness),
        })
    }

    // ---- Reading --------------------------------------------------------

    /// The active account at `address`.
    #[must_use]
    pub fn account(&self, address: &Address) -> Option<&Account> {
        self.accounts.get(address)
    }

    /// All active accounts, sorted by address.
    pub fn accounts(&self) -> impl Iterator<Item = (&Address, &Account)> {
        self.accounts.iter()
    }

    /// Number of active accounts.
    #[must_use]
    pub fn account_count(&self) -> usize {
        self.accounts.len()
    }

    /// Penalty record of an address with or without active account.
    #[must_use]
    pub fn penalties(&self, address: &Address) -> Option<&PenaltyRecord> {
        self.accounts
            .get(address)
            .map(Account::penalties)
            .or_else(|| self.detached.get(address).map(|d| &d.record))
    }

    /// Penalty record of an address without active account.
    #[must_use]
    pub fn detached_record(&self, address: &Address) -> Option<&DetachedRecord> {
        self.detached.get(address)
    }

    /// `true` if `right` of `address` is removed in the current epoch.
    #[must_use]
    pub fn is_locked(&self, address: &Address, right: Right) -> bool {
        self.penalties(address)
            .is_some_and(|p| p.is_locked(right, self.epoch))
    }

    /// `true` if `address` may sign as validator now (registered, enough
    /// stake, role neither locked nor paused).
    #[must_use]
    pub fn can_sign(&self, address: &Address) -> bool {
        self.accounts.get(address).is_some_and(|a| {
            a.validator.is_some()
                && a.stake >= STAKE_TIER_4
                && !a.penalties.is_locked(Right::Role, self.epoch)
        })
    }

    /// `true` if `address` may propose blocks now.
    #[must_use]
    pub fn can_propose(&self, address: &Address) -> bool {
        self.can_sign(address) && !self.is_locked(address, Right::Proposal)
    }

    /// Address that registered `bls_key`.
    #[must_use]
    pub fn validator_by_bls_key(&self, bls_key: &[u8; BLS_KEY_LEN]) -> Option<Address> {
        self.bls_keys.get(bls_key).copied()
    }

    /// Start validators from the genesis block.
    #[must_use]
    pub fn start_validators(&self) -> &[Address] {
        &self.start_validators
    }

    /// Balance of the refund pool.
    #[must_use]
    pub const fn refund_pool(&self) -> Amount {
        self.refund_pool
    }

    /// Balance of the work lottery pot.
    #[must_use]
    pub const fn work_lottery(&self) -> Amount {
        self.work_lottery
    }

    /// Balance of the fee lottery pot.
    #[must_use]
    pub const fn fee_lottery(&self) -> Amount {
        self.fee_lottery
    }

    /// Consensus share waiting for distribution (normal phase, M4).
    #[must_use]
    pub const fn consensus_rewards(&self) -> Amount {
        self.consensus_rewards
    }

    /// Current activation deposit for new accounts.
    #[must_use]
    pub const fn activation_deposit(&self) -> Amount {
        self.activation_deposit
    }

    /// Height of the current block.
    #[must_use]
    pub const fn height(&self) -> Height {
        self.height
    }

    /// Current epoch.
    #[must_use]
    pub const fn epoch(&self) -> EpochNumber {
        self.epoch
    }

    /// Rule data (base fee, quota, tickets).
    #[must_use]
    pub const fn chain(&self) -> &ChainData {
        &self.chain
    }

    /// Rule data, writable. It holds no TOY, so changing it cannot break
    /// rule I0.
    pub fn chain_mut(&mut self) -> &mut ChainData {
        &mut self.chain
    }

    // ---- Rule I0 and state root -----------------------------------------

    /// Amount in one pot.
    ///
    /// # Errors
    /// [`StateError::Arithmetic`] on overflow (not reachable below the cap).
    pub fn pot_total(&self, pot: Pot) -> Result<Amount, StateError> {
        let sum_accounts = |f: &dyn Fn(&Account) -> Result<Amount, ArithmeticError>| {
            self.accounts
                .values()
                .try_fold(Amount::ZERO, |acc, a| acc.checked_add(f(a)?))
        };
        let amount = match pot {
            Pot::Free => sum_accounts(&|a| Ok(a.free))?,
            Pot::ActivationDeposit => sum_accounts(&|a| Ok(a.deposit))?,
            Pot::OwnStake => sum_accounts(&|a| Ok(a.stake))?,
            Pot::Unbonding => sum_accounts(&Account::unbonding_total)?,
            Pot::HeldStake => sum_accounts(&Account::held_total)?,
            Pot::StartValidatorRewards => sum_accounts(&Account::reward_locks_total)?,
            Pot::RefundPool => self.refund_pool,
            Pot::WorkLottery => self.work_lottery,
            Pot::FeeLottery => self.fee_lottery,
            Pot::ConsensusRewards => self.consensus_rewards,
            // Filled by later milestones (M4, M5, M12); zero until then.
            Pot::DelegatedStake
            | Pot::VaultWithdrawals
            | Pot::HashTimeLocks
            | Pot::Escrow
            | Pot::DormantSnapshots => Amount::ZERO,
        };
        Ok(amount)
    }

    /// Amounts of all pots.
    ///
    /// # Errors
    /// See [`Self::pot_total`].
    pub fn pot_totals(&self) -> Result<PotTotals, StateError> {
        let entries = Pot::ALL
            .iter()
            .map(|p| self.pot_total(*p).map(|a| (*p, a)))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(PotTotals::new(entries))
    }

    /// Checks rule I0: the sum of all pots equals `minted`, the sum of all
    /// emissions so far (from `toy-emission`, computed independently).
    ///
    /// # Errors
    /// [`StateError::I0Violated`] if not.
    pub fn check_i0(&self, minted: Amount) -> Result<(), StateError> {
        let pots = self.pot_totals()?.sum()?;
        if pots == minted {
            Ok(())
        } else {
            Err(StateError::I0Violated { pots, minted })
        }
    }

    /// State root: BLAKE3 Merkle root over all accounts, detached penalty
    /// records and the global values (pots, parameters, rule data).
    #[must_use]
    pub fn root(&self) -> Hash {
        let account_leaves: Vec<Hash> = self
            .accounts
            .iter()
            .map(|(address, account)| {
                let mut data = address.as_bytes().to_vec();
                data.extend_from_slice(&account.encode());
                leaf_hash(&data)
            })
            .collect();
        let detached_leaves: Vec<Hash> = self
            .detached
            .iter()
            .map(|(address, d)| {
                let mut data = address.as_bytes().to_vec();
                d.record.encode_into(&mut data);
                data.extend_from_slice(&d.since_epoch.to_be_bytes());
                leaf_hash(&data)
            })
            .collect();
        let mut global = Vec::new();
        for amount in [
            self.refund_pool,
            self.work_lottery,
            self.fee_lottery,
            self.consensus_rewards,
            self.activation_deposit,
        ] {
            global.extend_from_slice(&amount.units().to_be_bytes());
        }
        global.extend_from_slice(&self.height.to_be_bytes());
        global.extend_from_slice(&self.epoch.to_be_bytes());
        for v in &self.start_validators {
            global.extend_from_slice(v.as_bytes());
        }
        global.extend_from_slice(&self.chain.encode());
        merkle_root(&[
            merkle_root(&account_leaves),
            merkle_root(&detached_leaves),
            leaf_hash(&global),
        ])
    }

    // ---- Blocks and epochs ----------------------------------------------
    //
    // All operations below are "check first, then change": every value
    // that could fail is computed before the first field is written, so an
    // error leaves the state untouched without copying it. (A full copy per
    // operation cost 3.5 ms at 10,000 accounts, measured in M3.)

    /// Starts block `height` in `epoch`. When the epoch advances, unbonded
    /// stake becomes free, bonded rewards and held stake whose time has
    /// come become normal stake, and old penalty records are pruned.
    ///
    /// # Errors
    /// [`StateError::UnexpectedHeight`] or [`StateError::UnexpectedEpoch`]
    /// if blocks or epochs are out of order.
    pub fn begin_block(&mut self, height: Height, epoch: EpochNumber) -> Result<(), StateError> {
        let expected = self
            .height
            .checked_add(1)
            .ok_or(ArithmeticError::Overflow)?;
        if height != expected {
            return Err(StateError::UnexpectedHeight {
                expected,
                found: height,
            });
        }
        let next_epoch = self.epoch.checked_add(1).ok_or(ArithmeticError::Overflow)?;
        if epoch == self.epoch {
            self.height = height;
            return Ok(());
        }
        if epoch != next_epoch {
            return Err(StateError::UnexpectedEpoch {
                current: self.epoch,
                found: epoch,
            });
        }
        // Once per epoch: walk all accounts. Work on a copy, so even an
        // (unreachable) overflow leaves the state unchanged.
        let mut next = self.clone();
        next.height = height;
        next.begin_epoch(epoch)?;
        *self = next;
        Ok(())
    }

    fn begin_epoch(&mut self, epoch: EpochNumber) -> Result<(), StateError> {
        self.epoch = epoch;
        for account in self.accounts.values_mut() {
            let (due, keep): (Vec<_>, Vec<_>) = core::mem::take(&mut account.unbonding)
                .into_iter()
                .partition(|u| u.release_epoch <= epoch);
            account.unbonding = keep;
            for u in due {
                account.free = account.free.checked_add(u.amount)?;
            }

            let (due, keep): (Vec<_>, Vec<_>) = core::mem::take(&mut account.reward_locks)
                .into_iter()
                .partition(|r| r.unlock_epoch <= epoch);
            account.reward_locks = keep;
            for r in due {
                account.stake = account.stake.checked_add(r.amount)?;
            }

            // Until M4 fixes the slashing amount, held stake returns
            // unslashed. M4 replaces this step with the slashing rule of
            // spec 5.4 (the slashed part goes to the refund pool).
            let (due, keep): (Vec<_>, Vec<_>) = core::mem::take(&mut account.held)
                .into_iter()
                .partition(|h| h.release_epoch <= epoch);
            account.held = keep;
            for h in due {
                account.stake = account.stake.checked_add(h.amount)?;
            }
        }
        self.detached.retain(|_, d| {
            let active = Right::ALL.iter().any(|r| d.record.is_locked(*r, epoch));
            active || epoch <= d.since_epoch.saturating_add(CLOSED_ACCOUNT_HISTORY_EPOCHS)
        });
        Ok(())
    }

    /// Mints the emission of one block: the only operation that creates
    /// TOY. The amounts come from `toy-emission`.
    ///
    /// # Errors
    /// [`StateError::PartsMismatch`] if the parts do not add up to the
    /// total.
    pub fn mint_block(&mut self, mint: &Mint) -> Result<(), StateError> {
        let consensus_total = match mint.consensus {
            ConsensusMint::StartValidators { each } => {
                let count = u128::try_from(self.start_validators.len())
                    .map_err(|_| ArithmeticError::Overflow)?;
                each.checked_mul(count)?
            }
            ConsensusMint::Stakers(amount) => amount,
        };
        let parts = Amount::checked_sum([consensus_total, mint.work_lottery, mint.refund_pool])?;
        if parts != mint.total {
            return Err(StateError::PartsMismatch);
        }
        let mut work_lottery = self.work_lottery.checked_add(mint.work_lottery)?;
        let refund_pool = self.refund_pool.checked_add(mint.refund_pool)?;
        let mut consensus_rewards = self.consensus_rewards;
        // Planned reward locks: (address, bonded amount, account to create).
        let mut plans: Vec<(Address, Amount, bool)> = Vec::new();
        let unlock = self
            .epoch
            .checked_add(START_REWARD_LOCK_EPOCHS)
            .ok_or(ArithmeticError::Overflow)?;
        match mint.consensus {
            ConsensusMint::Stakers(amount) => {
                consensus_rewards = consensus_rewards.checked_add(amount)?;
            }
            ConsensusMint::StartValidators { each } if !each.is_zero() => {
                for address in &self.start_validators {
                    if let Some(account) = self.accounts.get(address) {
                        let existing = account
                            .reward_locks
                            .iter()
                            .find(|r| r.unlock_epoch == unlock)
                            .map_or(Amount::ZERO, |r| r.amount);
                        existing.checked_add(each)?;
                        plans.push((*address, each, false));
                    } else if each >= self.activation_deposit {
                        // The first reward activates the account: the
                        // deposit is taken from it, the rest is bonded
                        // (spec 6.1, 5.3).
                        self.nonce_base()?;
                        plans.push((*address, each.checked_sub(self.activation_deposit)?, true));
                    } else {
                        // Too small to activate an account: nothing may be
                        // burned, so it goes to the work lottery like any
                        // unpaid bootstrap share.
                        work_lottery = work_lottery.checked_add(each)?;
                    }
                }
            }
            ConsensusMint::StartValidators { .. } => {}
        }
        // Everything checked: apply.
        self.work_lottery = work_lottery;
        self.refund_pool = refund_pool;
        self.consensus_rewards = consensus_rewards;
        let deposit = self.activation_deposit;
        for (address, bonded, create) in plans {
            if create {
                self.create_account(address, deposit, Amount::ZERO)?;
            }
            if bonded.is_zero() {
                continue;
            }
            let account = self.account_mut(&address)?;
            if let Some(lock) = account
                .reward_locks
                .iter_mut()
                .find(|r| r.unlock_epoch == unlock)
            {
                lock.amount = lock.amount.checked_add(bonded)?;
            } else {
                account.reward_locks.push(RewardLock {
                    amount: bonded,
                    unlock_epoch: unlock,
                });
            }
        }
        Ok(())
    }

    // ---- Operations signed by the account owner ---------------------------

    /// Checks and consumes the nonce of `sender`.
    ///
    /// # Errors
    /// [`StateError::UnknownAccount`], [`StateError::NonceMismatch`].
    pub fn use_nonce(&mut self, sender: &Address, nonce: u64) -> Result<(), StateError> {
        let account = self.account_mut(sender)?;
        if account.nonce != nonce {
            return Err(StateError::NonceMismatch {
                expected: account.nonce,
                found: nonce,
            });
        }
        account.nonce = nonce.checked_add(1).ok_or(StateError::NonceExhausted)?;
        Ok(())
    }

    /// Frees the part of the deposit above the current activation deposit
    /// (after the deposit was lowered, spec 6.1, decision 76). Returns the
    /// freed amount.
    ///
    /// # Errors
    /// [`StateError::UnknownAccount`].
    pub fn release_excess_deposit(&mut self, address: &Address) -> Result<Amount, StateError> {
        let param = self.activation_deposit;
        let account = self.account_mut(address)?;
        if account.deposit <= param {
            return Ok(Amount::ZERO);
        }
        let excess = account.deposit.checked_sub(param)?;
        let free = account.free.checked_add(excess)?;
        account.deposit = param;
        account.free = free;
        Ok(excess)
    }

    /// Moves `amount` from the free balance of `from` to `to`. A new
    /// address is activated if `amount` covers the deposit.
    ///
    /// # Errors
    /// [`StateError::InsufficientFunds`],
    /// [`StateError::BelowActivationDeposit`], [`StateError::UnknownAccount`].
    pub fn transfer(
        &mut self,
        from: &Address,
        to: &Address,
        amount: Amount,
    ) -> Result<CreditOutcome, StateError> {
        let remaining = self.free_after_debit(from, amount)?;
        if from == to {
            return Ok(CreditOutcome { activated: false });
        }
        self.check_credit(to, amount)?;
        self.account_mut(from)?.free = remaining;
        self.credit(*to, amount)
    }

    /// Debits a paid fee from `from` and splits it (spec 7.2). The
    /// proposer share goes to `proposer`; if that address has no account
    /// and the share is too small to activate one, it goes to the refund
    /// pool instead.
    ///
    /// # Errors
    /// [`StateError::InsufficientFunds`], [`StateError::UnknownAccount`].
    pub fn pay_fee(
        &mut self,
        from: &Address,
        split: &FeeSplit,
        proposer: &Address,
    ) -> Result<(), StateError> {
        let remaining = self.free_after_debit(from, split.total()?)?;
        let fee_lottery = self.fee_lottery.checked_add(split.fee_lottery)?;
        let mut refund_pool = self.refund_pool.checked_add(split.refund_pool)?;
        let pay_proposer = !split.proposer.is_zero()
            && (self.accounts.contains_key(proposer) || split.proposer >= self.activation_deposit);
        if pay_proposer {
            self.check_credit(proposer, split.proposer)?;
        } else {
            refund_pool = refund_pool.checked_add(split.proposer)?;
        }
        self.account_mut(from)?.free = remaining;
        self.fee_lottery = fee_lottery;
        self.refund_pool = refund_pool;
        if pay_proposer {
            self.credit(*proposer, split.proposer)?;
        }
        Ok(())
    }

    /// Donates `amount` from the free balance to the refund pool (spec 7.1,
    /// decision 79). Not reversible: no operation moves TOY from the pool
    /// back to a donor.
    ///
    /// # Errors
    /// [`StateError::InsufficientFunds`], [`StateError::UnknownAccount`].
    pub fn donate(&mut self, from: &Address, amount: Amount) -> Result<(), StateError> {
        let remaining = self.free_after_debit(from, amount)?;
        let refund_pool = self.refund_pool.checked_add(amount)?;
        self.account_mut(from)?.free = remaining;
        self.refund_pool = refund_pool;
        Ok(())
    }

    /// Bonds `amount` of the free balance as own stake.
    ///
    /// # Errors
    /// [`StateError::InsufficientFunds`], [`StateError::UnknownAccount`].
    pub fn bond(&mut self, address: &Address, amount: Amount) -> Result<(), StateError> {
        let remaining = self.free_after_debit(address, amount)?;
        let account = self.account_mut(address)?;
        let stake = account.stake.checked_add(amount)?;
        account.free = remaining;
        account.stake = stake;
        Ok(())
    }

    /// Starts unbonding `amount` of own stake; it becomes free after 21
    /// epochs. A registered validator must keep the tier 4 stake.
    ///
    /// # Errors
    /// [`StateError::InsufficientStake`], [`StateError::ValidatorStakeTooLow`].
    pub fn unbond(&mut self, address: &Address, amount: Amount) -> Result<(), StateError> {
        let release_epoch = self
            .epoch
            .checked_add(UNBONDING_EPOCHS)
            .ok_or(ArithmeticError::Overflow)?;
        let account = self.account_mut(address)?;
        let rest =
            account
                .stake
                .checked_sub(amount)
                .map_err(|_| StateError::InsufficientStake {
                    needed: amount,
                    available: account.stake,
                })?;
        if account.validator.is_some() && rest < STAKE_TIER_4 {
            return Err(StateError::ValidatorStakeTooLow {
                stake: rest,
                required: STAKE_TIER_4,
            });
        }
        account.stake = rest;
        if !amount.is_zero() {
            account.unbonding.push(Unbonding {
                amount,
                release_epoch,
            });
        }
        Ok(())
    }

    /// Registers `bls_key` (ownership already proven by the caller) as
    /// validator key of `address`. Registering the same key again ends an
    /// inactivity pause ("the node reports back", spec 8).
    ///
    /// # Errors
    /// [`StateError::ValidatorStakeTooLow`], [`StateError::RoleLocked`],
    /// [`StateError::BlsKeyInUse`], [`StateError::AlreadyValidator`].
    pub fn register_validator(
        &mut self,
        address: &Address,
        bls_key: [u8; BLS_KEY_LEN],
    ) -> Result<(), StateError> {
        let epoch = self.epoch;
        if let Some(owner) = self.bls_keys.get(&bls_key)
            && owner != address
        {
            return Err(StateError::BlsKeyInUse);
        }
        let account = self.account_mut(address)?;
        if account.stake < STAKE_TIER_4 {
            return Err(StateError::ValidatorStakeTooLow {
                stake: account.stake,
                required: STAKE_TIER_4,
            });
        }
        // A running penalty period blocks the role; a pause does not
        // (re-registering ends it).
        if account
            .penalties
            .locked_through(Right::Role)
            .is_some_and(|until| epoch <= until)
        {
            return Err(StateError::RoleLocked);
        }
        match account.validator {
            Some(info) if info.bls_key == bls_key => {
                if !account.penalties.role_paused() {
                    return Err(StateError::AlreadyValidator);
                }
                account.penalties.resume_role();
            }
            Some(_) => return Err(StateError::AlreadyValidator),
            None => {
                account.validator = Some(crate::account::ValidatorInfo { bls_key });
                account.penalties.resume_role();
                self.bls_keys.insert(bls_key, *address);
            }
        }
        Ok(())
    }

    /// Removes the validator registration of `address`.
    ///
    /// # Errors
    /// [`StateError::NotValidator`], [`StateError::UnknownAccount`].
    pub fn deregister_validator(&mut self, address: &Address) -> Result<(), StateError> {
        let account = self.account_mut(address)?;
        let info = account.validator.take().ok_or(StateError::NotValidator)?;
        self.bls_keys.remove(&info.bls_key);
        Ok(())
    }

    /// Closes the account (spec 6.1): `donation` goes to the refund pool,
    /// the rest of free balance plus deposit to `target`. Only possible
    /// without stake, unbonding, locked rewards, held stake or validator
    /// role. The penalty history stays for 64 epochs.
    ///
    /// # Errors
    /// [`StateError::CannotClose`], [`StateError::DonationTooLarge`],
    /// [`StateError::MissingCloseTarget`], [`StateError::CloseTargetIsSelf`],
    /// [`StateError::BelowActivationDeposit`] for a new target.
    pub fn close_account(
        &mut self,
        address: &Address,
        target: Option<&Address>,
        donation: Amount,
    ) -> Result<CreditOutcome, StateError> {
        let account = self
            .accounts
            .get(address)
            .ok_or(StateError::UnknownAccount(*address))?;
        let blocker = [
            (!account.stake.is_zero(), CloseBlocker::Stake),
            (!account.unbonding.is_empty(), CloseBlocker::Unbonding),
            (!account.reward_locks.is_empty(), CloseBlocker::RewardLocks),
            (!account.held.is_empty(), CloseBlocker::HeldStake),
            (account.validator.is_some(), CloseBlocker::Validator),
        ]
        .into_iter()
        .find_map(|(blocked, why)| blocked.then_some(why));
        if let Some(why) = blocker {
            return Err(StateError::CannotClose(why));
        }
        if target == Some(address) {
            return Err(StateError::CloseTargetIsSelf);
        }
        let total = account.free.checked_add(account.deposit)?;
        let rest = total
            .checked_sub(donation)
            .map_err(|_| StateError::DonationTooLarge)?;
        let refund_pool = self.refund_pool.checked_add(donation)?;
        let receiver = if rest.is_zero() {
            None
        } else {
            let to = target.ok_or(StateError::MissingCloseTarget)?;
            self.check_credit(to, rest)?;
            Some(*to)
        };
        // Everything checked: apply.
        let closed = self
            .accounts
            .remove(address)
            .ok_or(StateError::UnknownAccount(*address))?;
        self.refund_pool = refund_pool;
        if closed.penalties.has_history() {
            self.detached.insert(
                *address,
                DetachedRecord {
                    record: closed.penalties,
                    since_epoch: self.epoch,
                },
            );
        }
        match receiver {
            Some(to) => self.credit(to, rest),
            None => Ok(CreditOutcome { activated: false }),
        }
    }

    /// Updates the per-epoch counters of `address` (no TOY involved) and
    /// returns them.
    ///
    /// # Errors
    /// [`StateError::UnknownAccount`].
    pub fn update_activity<F>(
        &mut self,
        address: &Address,
        update: F,
    ) -> Result<EpochActivity, StateError>
    where
        F: FnOnce(&mut EpochActivity) -> Result<(), ArithmeticError>,
    {
        let epoch = self.epoch;
        let account = self.account_mut(address)?;
        let mut activity = account.activity.for_epoch(epoch);
        update(&mut activity)?;
        account.activity = activity;
        Ok(activity)
    }

    // ---- Operations of the protocol ---------------------------------------

    /// Pays lottery prizes from the work lottery pot. Winners without an
    /// account are activated (every prize is at least 0.2 TOY). Several
    /// prizes for the same address are paid as one credit.
    ///
    /// # Errors
    /// [`StateError::PotTooSmall`] if the prizes exceed the pot.
    pub fn pay_lottery_prizes(&mut self, prizes: &[(Address, Amount)]) -> Result<(), StateError> {
        let mut per_winner: BTreeMap<Address, Amount> = BTreeMap::new();
        for (winner, prize) in prizes {
            let sum = per_winner.entry(*winner).or_insert(Amount::ZERO);
            *sum = sum.checked_add(*prize)?;
        }
        let total = Amount::checked_sum(per_winner.values().copied())?;
        let work_lottery = self
            .work_lottery
            .checked_sub(total)
            .map_err(|_| StateError::PotTooSmall)?;
        for (winner, amount) in &per_winner {
            self.check_credit(winner, *amount)?;
        }
        self.work_lottery = work_lottery;
        for (winner, amount) in per_winner {
            self.credit(winner, amount)?;
        }
        Ok(())
    }

    /// Moves the surplus of the refund pool to the work lottery
    /// (decision 120).
    ///
    /// # Errors
    /// [`StateError::PotTooSmall`] if `amount` exceeds the pool.
    pub fn move_refund_surplus_to_lottery(&mut self, amount: Amount) -> Result<(), StateError> {
        let pool = self
            .refund_pool
            .checked_sub(amount)
            .map_err(|_| StateError::PotTooSmall)?;
        let lottery = self.work_lottery.checked_add(amount)?;
        self.refund_pool = pool;
        self.work_lottery = lottery;
        Ok(())
    }

    /// Records a proven violation of `address` (spec 8) in the current
    /// epoch. Removes only rights. For a double signature the owner's
    /// bonded and unbonding stake is held for 18 epochs (spec 5.4); its
    /// amount stays the owner's and stays in rule I0.
    ///
    /// Works for addresses without account too (e.g. invalid tickets from
    /// a device that has no account yet).
    ///
    /// # Errors
    /// [`StateError::Arithmetic`] on overflow.
    pub fn apply_violation(
        &mut self,
        address: &Address,
        violation: Violation,
    ) -> Result<PenaltyOutcome, StateError> {
        let epoch = self.epoch;
        if let Some(account) = self.accounts.get_mut(address) {
            let hold = if violation == Violation::DoubleSign {
                let release_epoch = epoch
                    .checked_add(DOUBLE_SIGN_HOLD_EPOCHS)
                    .ok_or(ArithmeticError::Overflow)?;
                Some((
                    account.stake.checked_add(account.unbonding_total()?)?,
                    release_epoch,
                ))
            } else {
                None
            };
            let outcome = account.penalties.record(violation, epoch);
            if let Some((amount, release_epoch)) = hold {
                account.stake = Amount::ZERO;
                account.unbonding.clear();
                if !amount.is_zero() {
                    account.held.push(HeldStake {
                        amount,
                        release_epoch,
                    });
                }
            }
            return Ok(outcome);
        }
        let entry = self.detached.entry(*address).or_insert(DetachedRecord {
            record: PenaltyRecord::default(),
            since_epoch: epoch,
        });
        entry.since_epoch = epoch;
        Ok(entry.record.record(violation, epoch))
    }

    /// Lowers the activation deposit for new accounts (decision 76). It may
    /// only fall, and never below 0.01 TOY. Existing accounts free the
    /// difference with their next own transaction.
    ///
    /// # Errors
    /// [`StateError::InvalidDepositChange`].
    pub fn lower_activation_deposit(&mut self, new_deposit: Amount) -> Result<(), StateError> {
        if new_deposit >= self.activation_deposit || new_deposit < ACTIVATION_DEPOSIT_FLOOR {
            return Err(StateError::InvalidDepositChange);
        }
        self.activation_deposit = new_deposit;
        Ok(())
    }

    // ---- Internal helpers ---------------------------------------------------

    fn account_mut(&mut self, address: &Address) -> Result<&mut Account, StateError> {
        self.accounts
            .get_mut(address)
            .ok_or(StateError::UnknownAccount(*address))
    }

    /// Free balance of `address` after debiting `amount` (nothing changes).
    fn free_after_debit(&self, address: &Address, amount: Amount) -> Result<Amount, StateError> {
        let account = self
            .accounts
            .get(address)
            .ok_or(StateError::UnknownAccount(*address))?;
        account
            .free
            .checked_sub(amount)
            .map_err(|_| StateError::InsufficientFunds {
                needed: amount,
                available: account.free,
            })
    }

    /// Checks that [`Self::credit`] of `amount` to `to` would succeed,
    /// without changing anything.
    fn check_credit(&self, to: &Address, amount: Amount) -> Result<(), StateError> {
        if let Some(account) = self.accounts.get(to) {
            account.free.checked_add(amount)?;
            return Ok(());
        }
        amount.checked_sub(self.activation_deposit).map_err(|_| {
            StateError::BelowActivationDeposit {
                amount,
                required: self.activation_deposit,
            }
        })?;
        self.nonce_base()?;
        Ok(())
    }

    /// Credits `amount` to the free balance of `to`, activating a new
    /// account if needed. Private: callers must have debited `amount` from
    /// another pot and checked the credit with [`Self::check_credit`].
    fn credit(&mut self, to: Address, amount: Amount) -> Result<CreditOutcome, StateError> {
        if let Some(account) = self.accounts.get_mut(&to) {
            account.free = account.free.checked_add(amount)?;
            return Ok(CreditOutcome { activated: false });
        }
        let deposit = self.activation_deposit;
        let free = amount
            .checked_sub(deposit)
            .map_err(|_| StateError::BelowActivationDeposit {
                amount,
                required: deposit,
            })?;
        self.create_account(to, deposit, free)?;
        Ok(CreditOutcome { activated: true })
    }

    /// Nonce of an account created in the current block (spec 6.1:
    /// `height × 2^16`).
    ///
    /// Why an old signed transaction can never be valid again: a block
    /// holds at most 128 KB, so far fewer than 2^16 transactions, and an
    /// account created at height h1 and closed before height h2 therefore
    /// ends with a nonce below h2 × 2^16, the base of any later account at
    /// the same address.
    fn nonce_base(&self) -> Result<u64, StateError> {
        self.height
            .checked_shl(NONCE_BASE_SHIFT)
            .filter(|n| n >> NONCE_BASE_SHIFT == self.height)
            .ok_or(StateError::NonceExhausted)
    }

    fn create_account(
        &mut self,
        address: Address,
        deposit: Amount,
        free: Amount,
    ) -> Result<(), StateError> {
        let nonce = self.nonce_base()?;
        let penalties = self
            .detached
            .remove(&address)
            .map(|d| d.record)
            .unwrap_or_default();
        self.accounts.insert(
            address,
            Account::new(free, deposit, self.height, nonce, penalties),
        );
        Ok(())
    }

    /// Test helper: changes a balance without a matching debit, to check
    /// that rule I0 catches it.
    #[cfg(test)]
    pub(crate) fn tamper_free_balance(&mut self, address: &Address, amount: Amount) {
        if let Some(a) = self.accounts.get_mut(address) {
            a.free = amount;
        }
    }
}
