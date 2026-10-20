//! Test network: the real emission engine (M1) drives the block executor
//! (M3). Blocks come every 60 seconds, so an epoch has 1,440 blocks.

// Test helpers: panicking on unexpected errors is the point here.
#![allow(
    dead_code,
    clippy::arithmetic_side_effects,
    clippy::missing_panics_doc,
    clippy::unwrap_used,
    clippy::panic,
    clippy::wrong_self_convention
)]

use toy_crypto::eip155::ChainId;
use toy_crypto::hash::blake3;
use toy_crypto::secp256k1::SecretKey;
use toy_crypto::work::ToyWorkHasher;
use toy_emission::{BlockInput, ConsensusPayout, EmissionEngine, Phase};
use toy_state::{ConsensusMint, GenesisConfig, Mint, State};
use toy_tx::ticket::sign_ticket;
use toy_tx::{
    Action, Block, BlockContext, BlockError, BlockReceipt, TxFields, execute_block,
    sign_transaction,
};
use toy_types::{Address, Amount, Hash};

pub const GENESIS_TIME: u64 = 1_800_000_000;
pub const CHAIN: ChainId = ChainId::TOYCHAIN_DEVNET;

pub fn key(n: u8) -> SecretKey {
    SecretKey::from_bytes(&[n; 32]).unwrap()
}

pub fn toy(n: u64) -> Amount {
    Amount::from_toy(n)
}

/// A generous fee cap for paid transactions (0.1 TOY).
pub const FEE: Amount = Amount::from_units(100_000_000_000_000_000);

pub fn to_state_mint(m: &toy_emission::BlockMint) -> Mint {
    Mint {
        total: m.total,
        consensus: match m.consensus {
            ConsensusPayout::StartValidators { each, .. } => {
                ConsensusMint::StartValidators { each }
            }
            ConsensusPayout::Stakers(a) => ConsensusMint::Stakers(a),
        },
        work_lottery: m.work_lottery,
        refund_pool: m.refund_pool,
    }
}

pub struct Net {
    pub state: State,
    pub engine: EmissionEngine,
    pub hasher: ToyWorkHasher,
    pub proposer: Address,
    pub phase: Phase,
    pub timestamp: u64,
    pub start_keys: Vec<SecretKey>,
}

impl Net {
    pub fn new(phase: Phase) -> Self {
        let start_keys: Vec<SecretKey> = (200..221).map(key).collect();
        let config = GenesisConfig {
            start_validators: start_keys.iter().map(SecretKey::address).collect(),
            genesis_randomness: Hash::default(),
        };
        Self {
            state: State::genesis(&config).unwrap(),
            engine: EmissionEngine::genesis(GENESIS_TIME),
            hasher: ToyWorkHasher::new().unwrap(),
            proposer: key(250).address(),
            phase,
            timestamp: GENESIS_TIME,
            start_keys,
        }
    }

    fn input(&self, counted_tx: u64, free_tx: u64) -> BlockInput {
        BlockInput {
            timestamp: self.timestamp + 60,
            counted_tx,
            free_tx,
            signatures_received: 21,
            signatures_expected: 21,
            phase: self.phase,
        }
    }

    /// Context of the next block (emission from a copy of the engine).
    pub fn next_ctx(&self) -> BlockContext {
        let mut engine = self.engine.clone();
        let outcome = engine.apply_block(&self.input(0, 0)).unwrap();
        let height = outcome.height;
        BlockContext {
            height,
            epoch: outcome.epoch,
            chain: CHAIN,
            proposer: self.proposer,
            randomness: blake3(&height.to_be_bytes()),
            mint: to_state_mint(&outcome.mint),
            minted_total: engine.minted(),
        }
    }

    /// Executes a block with a possibly modified context. Commits only on
    /// success.
    pub fn try_block_with(
        &mut self,
        block: &Block,
        change: impl FnOnce(&mut BlockContext),
    ) -> Result<BlockReceipt, BlockError> {
        let mut ctx = self.next_ctx();
        change(&mut ctx);
        let (state, receipt) = execute_block(&self.state, &ctx, block, &mut self.hasher)?;
        let mut engine = self.engine.clone();
        let outcome = engine
            .apply_block(&self.input(receipt.counted_tx, receipt.free_tx))
            .unwrap();
        assert_eq!(to_state_mint(&outcome.mint), ctx.mint);
        self.engine = engine;
        self.state = state;
        self.timestamp += 60;
        Ok(receipt)
    }

    pub fn try_block(&mut self, block: &Block) -> Result<BlockReceipt, BlockError> {
        self.try_block_with(block, |_| {})
    }

    pub fn block(&mut self, block: Block) -> BlockReceipt {
        self.try_block(&block).unwrap()
    }

    pub fn txs(&mut self, transactions: Vec<Vec<u8>>) -> BlockReceipt {
        self.block(Block {
            transactions,
            tickets: vec![],
        })
    }

    pub fn try_txs(&mut self, transactions: Vec<Vec<u8>>) -> Result<BlockReceipt, BlockError> {
        self.try_block(&Block {
            transactions,
            tickets: vec![],
        })
    }

    pub fn empty(&mut self, n: u64) {
        for _ in 0..n {
            self.block(Block::default());
        }
    }

    /// Runs empty blocks until the next block starts a new epoch, then that
    /// block too.
    pub fn to_next_epoch(&mut self) {
        let epoch = self.state.epoch();
        while self.state.epoch() == epoch {
            self.block(Block::default());
        }
    }

    pub fn minted(&self) -> Amount {
        self.engine.minted()
    }

    pub fn i0(&self) {
        self.state.check_i0(self.minted()).unwrap();
    }

    pub fn nonce(&self, a: &Address) -> u64 {
        self.state.account(a).unwrap().nonce()
    }

    pub fn free(&self, a: &Address) -> Amount {
        self.state
            .account(a)
            .map_or(Amount::ZERO, toy_state::Account::free)
    }

    /// Makes every ToyWork ticket valid (one hash per ticket).
    pub fn easy_work(&mut self) {
        self.state.chain_mut().ticket_threshold = u128::MAX;
    }

    /// Funds `who` through the work lottery until it has at least `amount`
    /// free. Lets the pot grow first so few draws are needed.
    pub fn fund(&mut self, who: &SecretKey, amount: Amount) {
        let address = who.address();
        let mut nonce = 1_000_000u64;
        while self.free(&address) < amount {
            let need = amount.checked_sub(self.free(&address)).unwrap();
            // Grow the pot to cover the need (about 14 TOY per block).
            while self.state.work_lottery() < need.checked_add(toy(1)).unwrap() {
                self.empty(1);
            }
            self.easy_work();
            let epoch = self.next_ctx().epoch;
            let ticket = sign_ticket(who, CHAIN, epoch, nonce);
            nonce += 1;
            self.block(Block {
                transactions: vec![],
                tickets: vec![ticket],
            });
            self.empty(2);
        }
    }

    pub fn fields(&self, who: &SecretKey) -> TxFields {
        let nonce = self
            .state
            .account(&who.address())
            .map_or(0, toy_state::Account::nonce);
        TxFields {
            nonce,
            max_fee: FEE,
            max_tip: Amount::ZERO,
            expiry: self.state.height() + 100,
            memo: vec![],
            work_nonce: None,
        }
    }

    pub fn sign(&self, who: &SecretKey, fields: &TxFields, action: &Action) -> Vec<u8> {
        sign_transaction(who, CHAIN, fields, action)
    }

    /// Paid transfer with the next nonce.
    pub fn transfer(&self, from: &SecretKey, to: &Address, amount: Amount) -> Vec<u8> {
        let fields = self.fields(from);
        self.sign(from, &fields, &Action::Transfer { to: *to, amount })
    }

    /// Paid transfer with an explicit nonce offset (several in one block).
    pub fn transfer_n(
        &self,
        from: &SecretKey,
        offset: u64,
        to: &Address,
        amount: Amount,
    ) -> Vec<u8> {
        let mut fields = self.fields(from);
        fields.nonce += offset;
        self.sign(from, &fields, &Action::Transfer { to: *to, amount })
    }

    /// Free-quota transfer (fee cap 0).
    pub fn free_transfer_n(
        &self,
        from: &SecretKey,
        offset: u64,
        to: &Address,
        amount: Amount,
    ) -> Vec<u8> {
        let mut fields = self.fields(from);
        fields.nonce += offset;
        fields.max_fee = Amount::ZERO;
        self.sign(from, &fields, &Action::Transfer { to: *to, amount })
    }
}

/// Unwraps the transaction error of a rejected block.
pub fn tx_error(result: Result<BlockReceipt, BlockError>) -> toy_tx::TxError {
    match result {
        Err(BlockError::Transaction { error, .. }) => error,
        other => panic!("expected a transaction error, got {other:?}"),
    }
}

/// Unwraps the ticket error of a rejected block.
pub fn ticket_error(result: Result<BlockReceipt, BlockError>) -> toy_tx::TxError {
    match result {
        Err(BlockError::Ticket { error, .. }) => error,
        other => panic!("expected a ticket error, got {other:?}"),
    }
}

impl Net {
    /// Builds a block greedily: every candidate that keeps the block valid
    /// is kept, the others are dropped (what a proposer does).
    pub fn greedy(
        &mut self,
        candidates: Vec<Vec<u8>>,
        tickets: Vec<Vec<u8>>,
    ) -> (BlockReceipt, Block) {
        let ctx = self.next_ctx();
        let mut block = Block::default();
        for t in tickets {
            block.tickets.push(t);
            if execute_block(&self.state, &ctx, &block, &mut self.hasher).is_err() {
                block.tickets.pop();
            }
        }
        for c in candidates {
            block.transactions.push(c);
            if execute_block(&self.state, &ctx, &block, &mut self.hasher).is_err() {
                block.transactions.pop();
            }
        }
        let receipt = self.try_block(&block).unwrap();
        (receipt, block)
    }
}

/// Small deterministic generator for test inputs (SplitMix64).
pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    pub fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }
}
