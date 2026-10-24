//! Evidence of a double signature (spec 5.4 and 8, decision 106).
//!
//! A double signature is two **conflicting** votes of the same BLS key for
//! the same height, the same round and the same voting phase. A vote in a
//! new round after a timeout is **not** a double signature.
//!
//! [`Vote`] is the vote format of the consensus (M4 builds on it). Its
//! signed message carries a fixed prefix and the chain ID, so a vote can
//! never be replayed on another ToyChain network or mistaken for a VRF
//! proof (own BLS domain tag in `toy-crypto`).

use toy_crypto::bls::{BlsPublicKey, BlsSecretKey, BlsSignature, SIGNATURE_LEN};
use toy_crypto::eip155::ChainId;
use toy_crypto::hash::blake3_derive;
use toy_state::BLS_KEY_LEN;
use toy_types::{Address, Hash, Height};

use crate::error::TxError;
use crate::rlp::{self, Item};

const VOTE_PREFIX: &[u8] = b"ToyChain vote v1";
const EVIDENCE_CONTEXT: &str = "ToyChain 2026-10 double-sign evidence id v1";

/// Voting phase of the Tendermint-style consensus (two phases per round).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VotePhase {
    /// First phase.
    Prevote,
    /// Second phase.
    Precommit,
}

impl VotePhase {
    const fn code(self) -> u8 {
        match self {
            Self::Prevote => 1,
            Self::Precommit => 2,
        }
    }

    fn from_code(code: u64) -> Result<Self, TxError> {
        match code {
            1 => Ok(Self::Prevote),
            2 => Ok(Self::Precommit),
            _ => Err(TxError::InvalidEvidence),
        }
    }
}

/// A consensus vote.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vote {
    /// Block height.
    pub height: Height,
    /// Round within the height (a new round starts after a timeout).
    pub round: u32,
    /// Phase within the round.
    pub phase: VotePhase,
    /// Hash of the block voted for (all zero for "no block").
    pub block_hash: Hash,
}

impl Vote {
    /// The message that is signed with BLS.
    #[must_use]
    pub fn signing_message(&self, chain: ChainId) -> Vec<u8> {
        let mut msg = Vec::with_capacity(VOTE_PREFIX.len().saturating_add(61));
        msg.extend_from_slice(VOTE_PREFIX);
        msg.extend_from_slice(&chain.get().to_be_bytes());
        msg.extend_from_slice(&self.height.to_be_bytes());
        msg.extend_from_slice(&self.round.to_be_bytes());
        msg.push(self.phase.code());
        msg.extend_from_slice(self.block_hash.as_bytes());
        msg
    }

    /// Signs the vote.
    #[must_use]
    pub fn sign(&self, key: &BlsSecretKey, chain: ChainId) -> SignedVote {
        SignedVote {
            vote: *self,
            signature: key.sign(&self.signing_message(chain)).to_bytes(),
        }
    }
}

/// A vote with its BLS signature.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SignedVote {
    /// The vote.
    pub vote: Vote,
    /// BLS signature (96 bytes).
    pub signature: [u8; SIGNATURE_LEN],
}

/// Evidence that `offender` signed two conflicting votes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DoubleSignEvidence {
    /// Address whose registered BLS key signed both votes.
    pub offender: Address,
    /// First vote.
    pub first: SignedVote,
    /// Second vote.
    pub second: SignedVote,
}

impl DoubleSignEvidence {
    /// Id of the offence (offender, height, round, phase): one offence is
    /// punished once, however many conflicting votes exist.
    #[must_use]
    pub fn id(&self) -> Hash {
        let v = &self.first.vote;
        let mut data = Vec::with_capacity(41);
        data.extend_from_slice(self.offender.as_bytes());
        data.extend_from_slice(&v.height.to_be_bytes());
        data.extend_from_slice(&v.round.to_be_bytes());
        data.push(v.phase.code());
        blake3_derive(EVIDENCE_CONTEXT, &data)
    }

    /// Checks the evidence against the offender's registered key.
    ///
    /// # Errors
    /// [`TxError::InvalidEvidence`] unless both votes are for the same
    /// height, round and phase, name different blocks, and both
    /// signatures verify with `registered_key`.
    pub fn verify(
        &self,
        chain: ChainId,
        registered_key: &[u8; BLS_KEY_LEN],
    ) -> Result<(), TxError> {
        let (a, b) = (&self.first.vote, &self.second.vote);
        let conflicting = a.height == b.height
            && a.round == b.round
            && a.phase == b.phase
            && a.block_hash != b.block_hash;
        if !conflicting {
            return Err(TxError::InvalidEvidence);
        }
        let key = BlsPublicKey::from_bytes(registered_key).map_err(|_| TxError::InvalidEvidence)?;
        for signed in [&self.first, &self.second] {
            let signature = BlsSignature::from_bytes(&signed.signature)
                .map_err(|_| TxError::InvalidEvidence)?;
            key.verify(&signed.vote.signing_message(chain), &signature)
                .map_err(|_| TxError::InvalidEvidence)?;
        }
        Ok(())
    }

    pub(crate) fn encode_into(&self, out: &mut Vec<u8>) {
        rlp::encode_bytes(out, self.offender.as_bytes());
        for signed in [&self.first, &self.second] {
            let mut vote = Vec::new();
            rlp::encode_u64(&mut vote, signed.vote.height);
            rlp::encode_u64(&mut vote, u64::from(signed.vote.round));
            rlp::encode_u64(&mut vote, u64::from(signed.vote.phase.code()));
            rlp::encode_bytes(&mut vote, signed.vote.block_hash.as_bytes());
            rlp::encode_bytes(&mut vote, &signed.signature);
            out.extend_from_slice(&rlp::encode_list(&vote));
        }
    }

    pub(crate) fn decode(
        offender: &Item<'_>,
        first: &Item<'_>,
        second: &Item<'_>,
    ) -> Result<Self, TxError> {
        Ok(Self {
            offender: Address::from_bytes(offender.as_array::<20>()?),
            first: decode_vote(first)?,
            second: decode_vote(second)?,
        })
    }
}

fn decode_vote(item: &Item<'_>) -> Result<SignedVote, TxError> {
    let [height, round, phase, block_hash, signature] = item.as_list()? else {
        return Err(TxError::WrongFieldCount);
    };
    let round = u32::try_from(round.as_u64()?).map_err(|_| TxError::ValueTooLarge)?;
    Ok(SignedVote {
        vote: Vote {
            height: height.as_u64()?,
            round,
            phase: VotePhase::from_code(phase.as_u64()?)?,
            block_hash: Hash::from_bytes(block_hash.as_array::<32>()?),
        },
        signature: signature.as_array::<SIGNATURE_LEN>()?,
    })
}
