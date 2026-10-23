//! ToyWork tickets (spec section 9, decisions 106, 119).
//!
//! A ticket is an Argon2id result (16 MiB) below the difficulty threshold,
//! bound to
//!
//! - the owner's **address** (nobody can claim someone else's work),
//! - the **epoch** and the **randomness of the last block of the previous
//!   epoch** (it cannot be computed in advance),
//! - the **chain ID** (no reuse on another ToyChain network),
//!
//! and signed with the owner's key, so nobody can burden an address with
//! invalid tickets. A ticket is used either for the work lottery
//! (standalone, see [`SignedTicket`]) or as fee substitute attached to a
//! transaction of its owner (the transaction signature covers it). One
//! ticket makes one transaction free (decision 119).
//!
//! Only the nonce travels; the work hash is recomputed by every verifier.

use toy_crypto::eip155::ChainId;
use toy_crypto::hash::{blake3_derive, keccak256};
use toy_crypto::secp256k1::{RecoverableSignature, SecretKey};
use toy_crypto::work::{ToyWorkHasher, work_value};
use toy_types::{Address, EpochNumber, Hash};

use crate::error::TxError;
use crate::rlp;

/// Type byte of a standalone ticket. Its own signing domain: a ticket
/// signature can never be a transaction signature (EIP-2718 range,
/// unused by Ethereum and by ToyChain transactions).
pub const TICKET_TYPE: u8 = 0x7f;

const INPUT_CONTEXT: &str = "ToyChain 2026-10 ToyWork input v1";
const ID_CONTEXT: &str = "ToyChain 2026-10 ToyWork ticket id v1";

/// What a ticket claims: owner, epoch and nonce.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TicketClaim {
    /// Address that did the work.
    pub owner: Address,
    /// Epoch the ticket is for.
    pub epoch: EpochNumber,
    /// Nonce chosen by the worker.
    pub nonce: u64,
}

impl TicketClaim {
    /// Unique id of the ticket (no reuse within an epoch).
    #[must_use]
    pub fn id(&self) -> Hash {
        let mut data = Vec::with_capacity(36);
        data.extend_from_slice(self.owner.as_bytes());
        data.extend_from_slice(&self.epoch.to_be_bytes());
        data.extend_from_slice(&self.nonce.to_be_bytes());
        blake3_derive(ID_CONTEXT, &data)
    }

    /// The 32-byte Argon2id input, bound to chain, owner, epoch and seed.
    #[must_use]
    pub fn work_input(&self, chain: ChainId, epoch_seed: &Hash) -> Hash {
        let mut data = Vec::with_capacity(76);
        data.extend_from_slice(&chain.get().to_be_bytes());
        data.extend_from_slice(self.owner.as_bytes());
        data.extend_from_slice(&self.epoch.to_be_bytes());
        data.extend_from_slice(epoch_seed.as_bytes());
        data.extend_from_slice(&self.nonce.to_be_bytes());
        blake3_derive(INPUT_CONTEXT, &data)
    }

    /// `true` if the work of this ticket is below `threshold`.
    ///
    /// # Errors
    /// [`TxError::Crypto`] if Argon2id fails (not expected).
    pub fn work_is_valid(
        &self,
        hasher: &mut ToyWorkHasher,
        chain: ChainId,
        epoch_seed: &Hash,
        threshold: u128,
    ) -> Result<bool, TxError> {
        let hash = hasher.hash(&self.work_input(chain, epoch_seed))?;
        Ok(work_value(&hash) < threshold)
    }
}

/// A standalone ticket for the work lottery.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SignedTicket {
    /// The claim; `owner` is recovered from the signature.
    pub claim: TicketClaim,
}

fn signing_hash(chain: ChainId, epoch: EpochNumber, nonce: u64) -> Hash {
    let mut fields = Vec::new();
    rlp::encode_u64(&mut fields, chain.get());
    rlp::encode_u64(&mut fields, epoch);
    rlp::encode_u64(&mut fields, nonce);
    let mut data = vec![TICKET_TYPE];
    data.extend_from_slice(&rlp::encode_list(&fields));
    keccak256(&data)
}

/// Encodes and signs a standalone ticket:
/// `0x7f ‖ rlp([chain_id, epoch, nonce, y_parity, r, s])`.
#[must_use]
pub fn sign_ticket(key: &SecretKey, chain: ChainId, epoch: EpochNumber, nonce: u64) -> Vec<u8> {
    let signature = key.sign_prehash(&signing_hash(chain, epoch, nonce));
    let mut fields = Vec::new();
    rlp::encode_u64(&mut fields, chain.get());
    rlp::encode_u64(&mut fields, epoch);
    rlp::encode_u64(&mut fields, nonce);
    rlp::encode_u64(&mut fields, u64::from(signature.y_parity()));
    rlp::encode_bytes(&mut fields, strip(signature.r()));
    rlp::encode_bytes(&mut fields, strip(signature.s()));
    let mut out = vec![TICKET_TYPE];
    out.extend_from_slice(&rlp::encode_list(&fields));
    out
}

fn strip(bytes: &[u8; 32]) -> &[u8] {
    let start = bytes.iter().position(|b| *b != 0).unwrap_or(32);
    bytes.get(start..).unwrap_or_default()
}

fn pad32(bytes: &[u8]) -> Result<[u8; 32], TxError> {
    let mut out = [0u8; 32];
    let start = 32usize
        .checked_sub(bytes.len())
        .ok_or(TxError::Rlp(rlp::RlpError::IntegerTooLarge))?;
    out.get_mut(start..)
        .ok_or(TxError::Rlp(rlp::RlpError::IntegerTooLarge))?
        .copy_from_slice(bytes);
    Ok(out)
}

/// Decodes a standalone ticket for `chain` and recovers its owner.
///
/// # Errors
/// [`TxError::UnsupportedType`], [`TxError::Rlp`],
/// [`TxError::WrongFieldCount`], [`TxError::WrongChainId`],
/// [`TxError::Crypto`] for an invalid signature.
pub fn decode_ticket(raw: &[u8], chain: ChainId) -> Result<SignedTicket, TxError> {
    let (&first, body) = raw.split_first().ok_or(TxError::Empty)?;
    if first != TICKET_TYPE {
        return Err(TxError::UnsupportedType(first));
    }
    let item = rlp::decode(body)?;
    let [chain_id, epoch, nonce, y_parity, r, s] = item.as_list()? else {
        return Err(TxError::WrongFieldCount);
    };
    if chain_id.as_u64()? != chain.get() {
        return Err(TxError::WrongChainId);
    }
    let epoch = epoch.as_u64()?;
    let nonce = nonce.as_u64()?;
    let parity = y_parity.as_u64()?;
    if parity > 1 {
        return Err(TxError::Crypto(toy_crypto::CryptoError::InvalidSignature));
    }
    let signature = RecoverableSignature::from_parts(
        pad32(r.as_uint_bytes(32)?)?,
        pad32(s.as_uint_bytes(32)?)?,
        parity == 1,
    )?;
    let owner = signature.recover_address(&signing_hash(chain, epoch, nonce))?;
    Ok(SignedTicket {
        claim: TicketClaim {
            owner,
            epoch,
            nonce,
        },
    })
}

/// Searches a valid ticket nonce, starting at `start`, trying at most
/// `max_tries` nonces (wallet and tests).
///
/// # Errors
/// [`TxError::Crypto`] if Argon2id fails.
#[allow(clippy::too_many_arguments)]
pub fn mine(
    hasher: &mut ToyWorkHasher,
    chain: ChainId,
    owner: Address,
    epoch: EpochNumber,
    epoch_seed: &Hash,
    threshold: u128,
    start: u64,
    max_tries: u64,
) -> Result<Option<u64>, TxError> {
    for nonce in (start..).take(usize::try_from(max_tries).unwrap_or(usize::MAX)) {
        let claim = TicketClaim {
            owner,
            epoch,
            nonce,
        };
        if claim.work_is_valid(hasher, chain, epoch_seed, threshold)? {
            return Ok(Some(nonce));
        }
    }
    Ok(None)
}
