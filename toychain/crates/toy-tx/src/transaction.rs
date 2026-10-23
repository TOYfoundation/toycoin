//! Transactions in both formats (spec 7.1, 7.2, decisions 84, 106).
//!
//! **ToyChain format** (typed envelope after EIP-2718): one type byte per
//! kind, then
//!
//! ```text
//! rlp([chain_id, nonce, max_fee, max_tip, expiry, memo, work, <kind fields>, y_parity, r, s])
//! ```
//!
//! signed over `keccak256(type ‖ rlp([chain_id … <kind fields>]))`. The
//! type byte is part of the signed data, so a signature for one kind can
//! never count for another kind, and the chain ID is signed too (EIP-155).
//! `work` is `[]` or `[ticket_nonce]`: a ToyWork ticket of the sender that
//! makes the transaction free (decision 119). `max_fee = 0` without a
//! ticket requests the free quota (decision 121).
//!
//! **Ethereum format**: see [`crate::ethereum`]; only plain transfers.

use toy_crypto::bls::{BlsPublicKey, ProofOfPossession, SIGNATURE_LEN};
use toy_crypto::eip155::ChainId;
use toy_crypto::hash::keccak256;
use toy_crypto::secp256k1::{RecoverableSignature, SecretKey};
use toy_state::BLS_KEY_LEN;
use toy_types::params::MEMO_MAX_BYTES;
use toy_types::{Address, Amount, Hash, Height};

use crate::error::TxError;
use crate::ethereum::{self, EthereumKind};
use crate::evidence::DoubleSignEvidence;
use crate::rlp::{self, Item};

/// Type bytes of the ToyChain transaction kinds. They lie in the EIP-2718
/// range (below `0x80`) and do not collide with Ethereum's types
/// (`0x01` to `0x04`).
pub mod kind {
    /// Transfer.
    pub const TRANSFER: u8 = 0x70;
    /// Bond stake.
    pub const BOND: u8 = 0x71;
    /// Unbond stake (21 epochs).
    pub const UNBOND: u8 = 0x72;
    /// Register a validator key.
    pub const REGISTER_VALIDATOR: u8 = 0x73;
    /// Deregister the validator key.
    pub const DEREGISTER_VALIDATOR: u8 = 0x74;
    /// Donate to the refund pool.
    pub const DONATE: u8 = 0x75;
    /// Close the account.
    pub const CLOSE_ACCOUNT: u8 = 0x76;
    /// Report a double signature.
    pub const REPORT_DOUBLE_SIGN: u8 = 0x77;
}

/// What a transaction does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    /// Moves TOY to another address.
    Transfer {
        /// Recipient.
        to: Address,
        /// Amount.
        amount: Amount,
    },
    /// Bonds free balance as stake.
    Bond(Amount),
    /// Starts unbonding stake.
    Unbond(Amount),
    /// Registers a BLS validator key with proof of possession.
    RegisterValidator {
        /// Compressed BLS public key.
        bls_key: [u8; BLS_KEY_LEN],
        /// Proof of possession.
        proof: [u8; SIGNATURE_LEN],
    },
    /// Removes the validator registration.
    DeregisterValidator,
    /// Donates to the refund pool.
    Donate(Amount),
    /// Closes the account.
    CloseAccount {
        /// Recipient of the rest (`None` if everything is donated).
        target: Option<Address>,
        /// Part donated to the refund pool.
        donation: Amount,
    },
    /// Reports a double signature.
    ReportDoubleSign(DoubleSignEvidence),
}

impl Action {
    const fn type_byte(&self) -> u8 {
        match self {
            Self::Transfer { .. } => kind::TRANSFER,
            Self::Bond(_) => kind::BOND,
            Self::Unbond(_) => kind::UNBOND,
            Self::RegisterValidator { .. } => kind::REGISTER_VALIDATOR,
            Self::DeregisterValidator => kind::DEREGISTER_VALIDATOR,
            Self::Donate(_) => kind::DONATE,
            Self::CloseAccount { .. } => kind::CLOSE_ACCOUNT,
            Self::ReportDoubleSign(_) => kind::REPORT_DOUBLE_SIGN,
        }
    }

    fn encode_into(&self, out: &mut Vec<u8>) {
        match self {
            Self::Transfer { to, amount } => {
                rlp::encode_bytes(out, to.as_bytes());
                rlp::encode_u128(out, amount.units());
            }
            Self::Bond(a) | Self::Unbond(a) | Self::Donate(a) => rlp::encode_u128(out, a.units()),
            Self::RegisterValidator { bls_key, proof } => {
                rlp::encode_bytes(out, bls_key);
                rlp::encode_bytes(out, proof);
            }
            Self::DeregisterValidator => {}
            Self::CloseAccount { target, donation } => {
                rlp::encode_bytes(out, target.as_ref().map_or(&[][..], |t| t.as_bytes()));
                rlp::encode_u128(out, donation.units());
            }
            Self::ReportDoubleSign(evidence) => evidence.encode_into(out),
        }
    }

    fn decode(type_byte: u8, fields: &[Item<'_>]) -> Result<Self, TxError> {
        let amount = |item: &Item<'_>| -> Result<Amount, TxError> {
            Ok(Amount::from_units(item.as_u128()?))
        };
        match (type_byte, fields) {
            (kind::TRANSFER, [to, value]) => Ok(Self::Transfer {
                to: Address::from_bytes(to.as_array::<20>()?),
                amount: amount(value)?,
            }),
            (kind::BOND, [value]) => Ok(Self::Bond(amount(value)?)),
            (kind::UNBOND, [value]) => Ok(Self::Unbond(amount(value)?)),
            (kind::DONATE, [value]) => Ok(Self::Donate(amount(value)?)),
            (kind::REGISTER_VALIDATOR, [key, proof]) => Ok(Self::RegisterValidator {
                bls_key: key.as_array::<BLS_KEY_LEN>()?,
                proof: proof.as_array::<SIGNATURE_LEN>()?,
            }),
            (kind::DEREGISTER_VALIDATOR, []) => Ok(Self::DeregisterValidator),
            (kind::CLOSE_ACCOUNT, [target, donation]) => {
                let bytes = target.as_bytes()?;
                let target = if bytes.is_empty() {
                    None
                } else {
                    Some(Address::from_bytes(target.as_array::<20>()?))
                };
                Ok(Self::CloseAccount {
                    target,
                    donation: amount(donation)?,
                })
            }
            (kind::REPORT_DOUBLE_SIGN, [offender, first, second]) => Ok(Self::ReportDoubleSign(
                DoubleSignEvidence::decode(offender, first, second)?,
            )),
            (
                kind::TRANSFER
                | kind::BOND
                | kind::UNBOND
                | kind::DONATE
                | kind::REGISTER_VALIDATOR
                | kind::DEREGISTER_VALIDATOR
                | kind::CLOSE_ACCOUNT
                | kind::REPORT_DOUBLE_SIGN,
                _,
            ) => Err(TxError::WrongFieldCount),
            (other, _) => Err(TxError::UnsupportedType(other)),
        }
    }
}

/// Checks a proof of possession and returns the key bytes (validator
/// registration).
///
/// # Errors
/// [`TxError::Crypto`] if key or proof are invalid.
pub fn verify_possession(
    bls_key: &[u8; BLS_KEY_LEN],
    proof: &[u8; SIGNATURE_LEN],
) -> Result<(), TxError> {
    let key = BlsPublicKey::from_bytes(bls_key)?;
    let proof = ProofOfPossession::from_bytes(proof)?;
    key.verify_possession(&proof)?;
    Ok(())
}

/// Fields common to all ToyChain transactions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxFields {
    /// Account nonce.
    pub nonce: u64,
    /// Highest total fee (0: request the free quota).
    pub max_fee: Amount,
    /// Highest tip on top of the required fee.
    pub max_tip: Amount,
    /// Last block height at which the transaction is valid (decision 84).
    pub expiry: Height,
    /// Payment reference, at most 32 bytes (kept only in the block window).
    pub memo: Vec<u8>,
    /// Nonce of an attached ToyWork ticket of the sender (fee substitute).
    pub work_nonce: Option<u64>,
}

/// Format of a decoded transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxFormat {
    /// ToyChain format with this type byte.
    ToyChain(u8),
    /// Ethereum format.
    Ethereum(EthereumKind),
}

/// A decoded transaction with recovered sender.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    /// Format.
    pub format: TxFormat,
    /// Signer.
    pub sender: Address,
    /// Nonce.
    pub nonce: u64,
    /// Highest total fee.
    pub max_fee: Amount,
    /// Highest tip.
    pub max_tip: Amount,
    /// Last valid height (`None` for Ethereum transactions).
    pub expiry: Option<Height>,
    /// Payment reference.
    pub memo: Vec<u8>,
    /// Attached ToyWork ticket nonce.
    pub work_nonce: Option<u64>,
    /// What it does.
    pub action: Action,
    /// Encoded size in bytes (basis of the fee).
    pub size: u64,
    /// Keccak-256 of the encoded bytes.
    pub hash: Hash,
}

impl Transaction {
    /// `true` if the transaction requests the free quota.
    #[must_use]
    pub fn requests_free_quota(&self) -> bool {
        self.max_fee.is_zero() && self.work_nonce.is_none()
    }
}

fn encode_common(out: &mut Vec<u8>, chain: ChainId, fields: &TxFields) {
    rlp::encode_u64(out, chain.get());
    rlp::encode_u64(out, fields.nonce);
    rlp::encode_u128(out, fields.max_fee.units());
    rlp::encode_u128(out, fields.max_tip.units());
    rlp::encode_u64(out, fields.expiry);
    rlp::encode_bytes(out, &fields.memo);
    let mut work = Vec::new();
    if let Some(n) = fields.work_nonce {
        rlp::encode_u64(&mut work, n);
    }
    out.extend_from_slice(&rlp::encode_list(&work));
}

fn strip(bytes: &[u8; 32]) -> &[u8] {
    let start = bytes.iter().position(|b| *b != 0).unwrap_or(32);
    bytes.get(start..).unwrap_or_default()
}

/// Encodes and signs a ToyChain transaction.
#[must_use]
pub fn sign_transaction(
    key: &SecretKey,
    chain: ChainId,
    fields: &TxFields,
    action: &Action,
) -> Vec<u8> {
    let type_byte = action.type_byte();
    let mut body = Vec::new();
    encode_common(&mut body, chain, fields);
    action.encode_into(&mut body);
    let mut signing = vec![type_byte];
    signing.extend_from_slice(&rlp::encode_list(&body));
    let signature = key.sign_prehash(&keccak256(&signing));
    rlp::encode_u64(&mut body, u64::from(signature.y_parity()));
    rlp::encode_bytes(&mut body, strip(signature.r()));
    rlp::encode_bytes(&mut body, strip(signature.s()));
    let mut out = vec![type_byte];
    out.extend_from_slice(&rlp::encode_list(&body));
    out
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

/// Number of common fields before the kind fields.
const COMMON_FIELDS: usize = 7;

/// Decodes a transaction in either format for `chain` and recovers its
/// sender. Checks only the transaction itself; whether it fits the state
/// is decided when the block is executed.
///
/// # Errors
/// Any [`TxError`] of decoding, signature and format rules.
pub fn decode_transaction(raw: &[u8], chain: ChainId) -> Result<Transaction, TxError> {
    let size = u64::try_from(raw.len()).map_err(|_| TxError::ValueTooLarge)?;
    if ethereum::is_ethereum(raw) {
        let transfer = ethereum::parse_ethereum(raw, chain)?.into_transfer()?;
        return Ok(Transaction {
            format: TxFormat::Ethereum(transfer.kind),
            sender: transfer.sender,
            nonce: transfer.nonce,
            max_fee: transfer.max_fee,
            max_tip: transfer.max_tip,
            expiry: None,
            memo: Vec::new(),
            work_nonce: None,
            action: Action::Transfer {
                to: transfer.to,
                amount: transfer.value,
            },
            size,
            hash: transfer.hash,
        });
    }
    let (&type_byte, body) = raw.split_first().ok_or(TxError::Empty)?;
    if !(kind::TRANSFER..=kind::REPORT_DOUBLE_SIGN).contains(&type_byte) {
        return Err(TxError::UnsupportedType(type_byte));
    }
    let item = rlp::decode(body)?;
    let fields = item.as_list()?;
    let unsigned_len = fields
        .len()
        .checked_sub(3)
        .ok_or(TxError::WrongFieldCount)?;
    let (unsigned, signature) = fields.split_at(unsigned_len);
    let [y_parity, r, s] = signature else {
        return Err(TxError::WrongFieldCount);
    };
    if unsigned.len() < COMMON_FIELDS {
        return Err(TxError::WrongFieldCount);
    }
    let (common, kind_fields) = unsigned.split_at(COMMON_FIELDS);
    let [chain_id, nonce, max_fee, max_tip, expiry, memo, work] = common else {
        return Err(TxError::WrongFieldCount);
    };
    if chain_id.as_u64()? != chain.get() {
        return Err(TxError::WrongChainId);
    }
    let memo = memo.as_bytes()?;
    if memo.len() > MEMO_MAX_BYTES {
        return Err(TxError::MemoTooLong);
    }
    let work_nonce = match work.as_list()? {
        [] => None,
        [n] => Some(n.as_u64()?),
        _ => return Err(TxError::WrongFieldCount),
    };
    let action = Action::decode(type_byte, kind_fields)?;

    let parity = y_parity.as_u64()?;
    if parity > 1 {
        return Err(TxError::Crypto(toy_crypto::CryptoError::InvalidSignature));
    }
    let mut signing = vec![type_byte];
    let mut payload = Vec::new();
    for field in unsigned {
        rlp::encode_item(&mut payload, field);
    }
    signing.extend_from_slice(&rlp::encode_list(&payload));
    let sender = RecoverableSignature::from_parts(
        pad32(r.as_uint_bytes(32)?)?,
        pad32(s.as_uint_bytes(32)?)?,
        parity == 1,
    )?
    .recover_address(&keccak256(&signing))?;

    Ok(Transaction {
        format: TxFormat::ToyChain(type_byte),
        sender,
        nonce: nonce.as_u64()?,
        max_fee: Amount::from_units(max_fee.as_u128()?),
        max_tip: Amount::from_units(max_tip.as_u128()?),
        expiry: Some(expiry.as_u64()?),
        memo: memo.to_vec(),
        work_nonce,
        action,
        size,
        hash: keccak256(raw),
    })
}
