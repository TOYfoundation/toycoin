//! Ethereum transaction format (spec 7.2, decision 106): legacy
//! transactions with EIP-155 chain ID and EIP-1559 transactions (type 2),
//! so MetaMask, hardware wallets and scripts can send TOY.
//!
//! Parsing follows Ethereum's rules ([`parse_ethereum`]); ToyChain then
//! accepts only plain transfers ([`EthereumTx::into_transfer`]): a
//! recipient, no call data, no access list, amounts within 128 bits.
//! There is no contract execution on ToyChain (decision 36).
//!
//! Mapping of Ethereum fees to ToyChain fees: the gas limit has no meaning
//! on ToyChain except as part of the fee cap. A transaction is willing to
//! pay at most `gas_limit × max_fee_per_gas` in total (legacy:
//! `gas_limit × gas_price`) and at most `gas_limit × max_priority_fee_per_gas`
//! as tip (legacy: everything it offers, as on Ethereum). A fee cap of 0
//! requests the free quota.

use toy_crypto::eip155::{ChainId, Eip155Signature, recover_eip155};
use toy_crypto::hash::keccak256;
use toy_crypto::secp256k1::{RecoverableSignature, SecretKey};
use toy_types::{Address, Amount, Hash};

use crate::error::TxError;
use crate::rlp::{self, Item};

/// Type byte of EIP-1559 transactions.
pub const EIP1559_TYPE: u8 = 0x02;

/// Gas of a plain Ethereum transfer. ToyChain requires at least this gas
/// limit, as Ethereum does for a transfer without data.
pub const TRANSFER_GAS: u64 = 21_000;

/// Which Ethereum format a transaction uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EthereumKind {
    /// Legacy transaction with EIP-155 chain ID in `v`.
    Legacy,
    /// EIP-1559 transaction (type 2).
    Eip1559,
}

/// An Ethereum transaction after Ethereum's own validity checks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EthereumTx {
    /// Format.
    pub kind: EthereumKind,
    /// Nonce.
    pub nonce: u64,
    /// Gas limit.
    pub gas_limit: u64,
    /// Legacy gas price or EIP-1559 `max_fee_per_gas` (big-endian, up to
    /// 32 bytes).
    pub max_fee_per_gas: Vec<u8>,
    /// EIP-1559 `max_priority_fee_per_gas`; for legacy the gas price.
    pub max_priority_fee_per_gas: Vec<u8>,
    /// Recipient, `None` for contract creation.
    pub to: Option<Address>,
    /// Value (big-endian, up to 32 bytes).
    pub value: Vec<u8>,
    /// Length of the call data.
    pub data_len: usize,
    /// Number of access list entries.
    pub access_list_len: usize,
    /// Recovered sender.
    pub sender: Address,
    /// Transaction hash as on Ethereum: Keccak-256 of the raw bytes.
    pub hash: Hash,
}

/// A plain transfer accepted by ToyChain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EthereumTransfer {
    /// Format.
    pub kind: EthereumKind,
    /// Nonce.
    pub nonce: u64,
    /// Recipient.
    pub to: Address,
    /// Amount.
    pub value: Amount,
    /// Highest total fee the sender accepts.
    pub max_fee: Amount,
    /// Highest tip the sender accepts.
    pub max_tip: Amount,
    /// Sender.
    pub sender: Address,
    /// Transaction hash.
    pub hash: Hash,
}

fn uint_to_u128(bytes: &[u8]) -> Result<u128, TxError> {
    if bytes.len() > 16 {
        return Err(TxError::ValueTooLarge);
    }
    let mut buf = [0u8; 16];
    let start = 16usize.saturating_sub(bytes.len());
    buf.get_mut(start..)
        .ok_or(TxError::ValueTooLarge)?
        .copy_from_slice(bytes);
    Ok(u128::from_be_bytes(buf))
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

fn parse_to(item: &Item<'_>) -> Result<Option<Address>, TxError> {
    let bytes = item.as_bytes()?;
    if bytes.is_empty() {
        return Ok(None);
    }
    let array: [u8; 20] = bytes
        .try_into()
        .map_err(|_| TxError::Rlp(rlp::RlpError::WrongLength))?;
    Ok(Some(Address::from_bytes(array)))
}

fn parse_access_list(item: &Item<'_>) -> Result<usize, TxError> {
    let entries = item.as_list()?;
    for entry in entries {
        match entry.as_list()? {
            [address, keys] => {
                address.as_array::<20>()?;
                for key in keys.as_list()? {
                    key.as_array::<32>()?;
                }
            }
            _ => return Err(TxError::WrongFieldCount),
        }
    }
    Ok(entries.len())
}

fn encode_items(items: &[Item<'_>]) -> Vec<u8> {
    let mut payload = Vec::new();
    for item in items {
        rlp::encode_item(&mut payload, item);
    }
    rlp::encode_list(&payload)
}

/// `true` if `raw` looks like an Ethereum transaction (legacy list or
/// type 2), not a ToyChain transaction.
#[must_use]
pub fn is_ethereum(raw: &[u8]) -> bool {
    matches!(raw.first(), Some(b) if *b >= 0xc0 || *b == EIP1559_TYPE)
}

/// Parses and checks an Ethereum transaction for `chain` and recovers its
/// sender.
///
/// # Errors
/// [`TxError::UnsupportedType`] for types other than legacy and 2,
/// [`TxError::Crypto`] for signatures without or with a foreign chain ID
/// or invalid signatures, [`TxError::WrongChainId`], [`TxError::Rlp`] and
/// [`TxError::WrongFieldCount`] for malformed data,
/// [`TxError::PriorityAboveMaxFee`].
pub fn parse_ethereum(raw: &[u8], chain: ChainId) -> Result<EthereumTx, TxError> {
    let (&first, rest) = raw.split_first().ok_or(TxError::Empty)?;
    if first >= 0xc0 {
        parse_legacy(raw, chain)
    } else if first == EIP1559_TYPE {
        parse_1559(raw, rest, chain)
    } else {
        Err(TxError::UnsupportedType(first))
    }
}

fn parse_legacy(raw: &[u8], chain: ChainId) -> Result<EthereumTx, TxError> {
    let item = rlp::decode(raw)?;
    let [nonce, price, gas, to, value, data, v, r, s] = item.as_list()? else {
        return Err(TxError::WrongFieldCount);
    };
    let nonce_value = nonce.as_u64()?;
    // EIP-2681: the nonce is below 2^64 - 1.
    if nonce_value == u64::MAX {
        return Err(TxError::ValueTooLarge);
    }
    let gas_price = price.as_uint_bytes(32)?;
    let gas_limit = gas.as_u64()?;
    let to_address = parse_to(to)?;
    let value_bytes = value.as_uint_bytes(32)?;
    let data_bytes = data.as_bytes()?;
    let v_value = v.as_u64()?;
    let signature = Eip155Signature {
        v: v_value,
        r: pad32(r.as_uint_bytes(32)?)?,
        s: pad32(s.as_uint_bytes(32)?)?,
    };
    // EIP-155 signing data: the six fields, then chain ID, 0, 0.
    let mut payload = Vec::new();
    for field in [nonce, price, gas, to, value, data] {
        rlp::encode_item(&mut payload, field);
    }
    rlp::encode_u64(&mut payload, chain.get());
    rlp::encode_u64(&mut payload, 0);
    rlp::encode_u64(&mut payload, 0);
    let signing_hash = keccak256(&rlp::encode_list(&payload));
    let sender = recover_eip155(&signing_hash, &signature, chain)?;
    Ok(EthereumTx {
        kind: EthereumKind::Legacy,
        nonce: nonce_value,
        gas_limit,
        max_fee_per_gas: gas_price.to_vec(),
        max_priority_fee_per_gas: gas_price.to_vec(),
        to: to_address,
        value: value_bytes.to_vec(),
        data_len: data_bytes.len(),
        access_list_len: 0,
        sender,
        hash: keccak256(raw),
    })
}

fn parse_1559(raw: &[u8], body: &[u8], chain: ChainId) -> Result<EthereumTx, TxError> {
    let item = rlp::decode(body)?;
    let fields = item.as_list()?;
    let [
        chain_id,
        nonce,
        priority,
        max_fee,
        gas,
        to,
        value,
        data,
        access_list,
        y_parity,
        r,
        s,
    ] = fields
    else {
        return Err(TxError::WrongFieldCount);
    };
    let chain_bytes = chain_id.as_uint_bytes(32)?;
    let nonce_value = nonce.as_u64()?;
    if nonce_value == u64::MAX {
        return Err(TxError::ValueTooLarge);
    }
    let priority_bytes = priority.as_uint_bytes(32)?;
    let max_fee_bytes = max_fee.as_uint_bytes(32)?;
    let gas_limit = gas.as_u64()?;
    let to_address = parse_to(to)?;
    let value_bytes = value.as_uint_bytes(32)?;
    let data_bytes = data.as_bytes()?;
    let access_list_len = parse_access_list(access_list)?;
    let parity = y_parity.as_u64()?;
    if parity > 1 {
        return Err(TxError::Crypto(toy_crypto::CryptoError::InvalidSignature));
    }
    let r_bytes = pad32(r.as_uint_bytes(32)?)?;
    let s_bytes = pad32(s.as_uint_bytes(32)?)?;
    // EIP-1559: the priority fee never exceeds the max fee. Compare as
    // big-endian numbers without leading zeros: length first.
    if (priority_bytes.len(), priority_bytes) > (max_fee_bytes.len(), max_fee_bytes) {
        return Err(TxError::PriorityAboveMaxFee);
    }
    if chain_bytes != chain.get().to_be_bytes().as_slice().strip_leading_zeros() {
        return Err(TxError::WrongChainId);
    }
    let mut signing_data = vec![EIP1559_TYPE];
    signing_data.extend_from_slice(&encode_items(fields.get(..9).unwrap_or_default()));
    let signing_hash = keccak256(&signing_data);
    let sender = RecoverableSignature::from_parts(r_bytes, s_bytes, parity == 1)?
        .recover_address(&signing_hash)?;
    Ok(EthereumTx {
        kind: EthereumKind::Eip1559,
        nonce: nonce_value,
        gas_limit,
        max_fee_per_gas: max_fee_bytes.to_vec(),
        max_priority_fee_per_gas: priority_bytes.to_vec(),
        to: to_address,
        value: value_bytes.to_vec(),
        data_len: data_bytes.len(),
        access_list_len,
        sender,
        hash: keccak256(raw),
    })
}

trait StripLeadingZeros {
    fn strip_leading_zeros(&self) -> &[u8];
}

impl StripLeadingZeros for [u8] {
    fn strip_leading_zeros(&self) -> &[u8] {
        let start = self.iter().position(|b| *b != 0).unwrap_or(self.len());
        self.get(start..).unwrap_or_default()
    }
}

impl EthereumTx {
    /// Applies ToyChain's restrictions: only plain transfers.
    ///
    /// # Errors
    /// [`TxError::NotATransfer`] for contract creation, call data or an
    /// access list; [`TxError::ValueTooLarge`] if value or fee cap does not
    /// fit 128 bits; [`TxError::GasLimitTooLow`] below 21,000 gas.
    pub fn into_transfer(self) -> Result<EthereumTransfer, TxError> {
        let to = self.to.ok_or(TxError::NotATransfer)?;
        if self.data_len != 0 || self.access_list_len != 0 {
            return Err(TxError::NotATransfer);
        }
        if self.gas_limit < TRANSFER_GAS {
            return Err(TxError::GasLimitTooLow);
        }
        let gas = u128::from(self.gas_limit);
        let max_fee = uint_to_u128(&self.max_fee_per_gas)?
            .checked_mul(gas)
            .ok_or(TxError::ValueTooLarge)?;
        let max_tip = uint_to_u128(&self.max_priority_fee_per_gas)?
            .checked_mul(gas)
            .ok_or(TxError::ValueTooLarge)?;
        Ok(EthereumTransfer {
            kind: self.kind,
            nonce: self.nonce,
            to,
            value: Amount::from_units(uint_to_u128(&self.value)?),
            max_fee: Amount::from_units(max_fee),
            max_tip: Amount::from_units(max_tip),
            sender: self.sender,
            hash: self.hash,
        })
    }
}

/// Builds and signs a legacy EIP-155 transfer (wallet, tests).
#[must_use]
pub fn sign_legacy_transfer(
    key: &SecretKey,
    chain: ChainId,
    nonce: u64,
    gas_price: u128,
    gas_limit: u64,
    to: &Address,
    value: Amount,
) -> Vec<u8> {
    let mut fields = Vec::new();
    rlp::encode_u64(&mut fields, nonce);
    rlp::encode_u128(&mut fields, gas_price);
    rlp::encode_u64(&mut fields, gas_limit);
    rlp::encode_bytes(&mut fields, to.as_bytes());
    rlp::encode_u128(&mut fields, value.units());
    rlp::encode_bytes(&mut fields, &[]);
    let mut signing = fields.clone();
    rlp::encode_u64(&mut signing, chain.get());
    rlp::encode_u64(&mut signing, 0);
    rlp::encode_u64(&mut signing, 0);
    let signature =
        toy_crypto::eip155::sign_eip155(key, chain, &keccak256(&rlp::encode_list(&signing)));
    rlp::encode_u64(&mut fields, signature.v);
    rlp::encode_bytes(&mut fields, signature.r.strip_leading_zeros());
    rlp::encode_bytes(&mut fields, signature.s.strip_leading_zeros());
    rlp::encode_list(&fields)
}

/// Builds and signs an EIP-1559 transfer (wallet, tests).
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn sign_1559_transfer(
    key: &SecretKey,
    chain: ChainId,
    nonce: u64,
    max_priority_fee_per_gas: u128,
    max_fee_per_gas: u128,
    gas_limit: u64,
    to: &Address,
    value: Amount,
) -> Vec<u8> {
    let mut fields = Vec::new();
    rlp::encode_u64(&mut fields, chain.get());
    rlp::encode_u64(&mut fields, nonce);
    rlp::encode_u128(&mut fields, max_priority_fee_per_gas);
    rlp::encode_u128(&mut fields, max_fee_per_gas);
    rlp::encode_u64(&mut fields, gas_limit);
    rlp::encode_bytes(&mut fields, to.as_bytes());
    rlp::encode_u128(&mut fields, value.units());
    rlp::encode_bytes(&mut fields, &[]);
    fields.extend_from_slice(&rlp::encode_list(&[]));
    let mut signing = vec![EIP1559_TYPE];
    signing.extend_from_slice(&rlp::encode_list(&fields));
    let signature = key.sign_prehash(&keccak256(&signing));
    rlp::encode_u64(&mut fields, u64::from(signature.y_parity()));
    rlp::encode_bytes(&mut fields, signature.r().strip_leading_zeros());
    rlp::encode_bytes(&mut fields, signature.s().strip_leading_zeros());
    let mut out = vec![EIP1559_TYPE];
    out.extend_from_slice(&rlp::encode_list(&fields));
    out
}
