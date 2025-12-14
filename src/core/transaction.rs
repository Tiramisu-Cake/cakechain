use super::basics::Address;
use super::state::State;

use ed25519_dalek::Signature as ed25519_signature;
use ed25519_dalek::SigningKey;
use ed25519_dalek::VerifyingKey;
use ed25519_dalek::ed25519::signature::SignerMut;

pub type ChainId = u64;

pub const CHAIN_ID: ChainId = 1;
pub const TX_DOMAIN_TAG: &[u8; 4] = b"TXv1";

//"TXv1" || chain_id(u64 LE) || from(32) || to(32) || amount(u64 LE) || nonce(u64 LE)
pub const TX_SIGNING_BYTES_LENGTH: usize = TX_DOMAIN_TAG.len() + 8 + 32 + 32 + 8 + 8;
//"TXv1" || from(32) || to(32) || amount(u64 LE) || nonce(u64 LE) || signature
pub const TX_CANONICAL_BYTES_LENGTH: usize = TX_DOMAIN_TAG.len() + 32 + 32 + 8 + 8 + 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Signature([u8; 64]);

impl TryFrom<&[u8]> for Signature {
    type Error = SignatureError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let arr: [u8; 64] = slice
            .try_into()
            .map_err(|_| SignatureError::InvalidLength {
                expected: 64,
                actual: slice.len(),
            })?;
        Ok(Signature(arr))
    }
}

impl Signature {
    fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureError {
    InvalidLength { expected: usize, actual: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxError {
    BadSignature,
    BadNonce { expected: u64, got: u64 },
    InsufficientFunds { balance: u64, needed: u64 },
    Overflow,
    SelfTransferForbidden,
    ZeroTransactionForbidden,
    // etc
}

#[derive(Copy, Clone)]
pub struct TxBody {
    from: Address,
    to: Address,
    amount: u64,
    nonce: u64,
}

impl TxBody {
    pub fn signing_bytes(&self, chain_id: ChainId) -> Vec<u8> {
        let mut out = Vec::with_capacity(TX_SIGNING_BYTES_LENGTH);

        out.extend_from_slice(TX_DOMAIN_TAG);
        out.extend_from_slice(&chain_id.to_le_bytes());
        out.extend_from_slice(self.from.as_bytes());
        out.extend_from_slice(self.to.as_bytes());
        out.extend_from_slice(&self.amount.to_le_bytes());
        out.extend_from_slice(&self.nonce.to_le_bytes());

        out
    }
}

pub struct UnsignedTransaction {
    body: TxBody,
}

impl UnsignedTransaction {
    pub fn sign(&self, signing_key: &mut SigningKey, chain_id: ChainId) -> Transaction {
        let msg = &self.body.signing_bytes(chain_id);
        let signature = signing_key.sign(msg).to_bytes();

        Transaction {
            body: self.body,
            signature: Signature(signature),
        }
    }
}

pub struct Transaction {
    body: TxBody,
    signature: Signature,
}

impl Transaction {
    pub fn verify_signature(
        &self,
        chain_id: ChainId,
        // verify_fn: impl Fn(&Address, &[u8], &Signature) -> bool,
    ) -> Result<(), TxError> {
        let addr_bytes = self.body.from.as_bytes();
        let ver_key = VerifyingKey::from_bytes(addr_bytes).map_err(|_| TxError::BadSignature)?;

        let sg = ed25519_signature::from_bytes(self.signature.as_bytes());

        let msg = &self.body.signing_bytes(chain_id);

        ver_key
            .verify_strict(&msg, &sg)
            .map_err(|_| TxError::BadSignature)?;
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(TX_CANONICAL_BYTES_LENGTH);

        let body = self.body;
        out.extend_from_slice(TX_DOMAIN_TAG);
        out.extend_from_slice(body.from.as_bytes());
        out.extend_from_slice(body.to.as_bytes());
        out.extend_from_slice(&body.amount.to_le_bytes());
        out.extend_from_slice(&body.nonce.to_le_bytes());
        out.extend_from_slice(self.signature.as_bytes());

        out
    }
}

pub fn validate_tx(tx: &Transaction, state: &State, chain_id: ChainId) -> Result<(), TxError> {
    let from = tx.body.from;
    let to = tx.body.to;
    let amount = tx.body.amount;
    let nonce = tx.body.nonce;

    if from == to {
        let err = TxError::SelfTransferForbidden;
        return Err(err);
    }

    if amount == 0 {
        return Err(TxError::ZeroTransactionForbidden);
    }

    let balance = *state.balances.get(&from).unwrap_or(&0u64);

    if balance < amount {
        let err = TxError::InsufficientFunds {
            balance,
            needed: amount,
        };
        return Err(err);
    }

    let addr_nonce = *state.nonces.get(&from).unwrap_or(&0u64);

    if nonce != addr_nonce {
        let err = TxError::BadNonce {
            expected: addr_nonce,
            got: nonce,
        };
        return Err(err);
    }
    tx.verify_signature(chain_id)
}

pub fn apply_tx(state: &mut State, tx: &Transaction, chain_id: ChainId) -> Result<(), TxError> {
    validate_tx(tx, state, chain_id)?;

    let from = tx.body.from;
    let to = tx.body.to;
    let amount = tx.body.amount;
    let nonce = tx.body.nonce;

    let from_balance = *state.balances.get(&from).unwrap_or(&0u64);
    let to_balance = *state.balances.get(&to).unwrap_or(&0u64);

    let new_from = from_balance - amount;
    let new_to = to_balance.checked_add(amount).ok_or(TxError::Overflow)?;
    let new_nonce = nonce.checked_add(1).ok_or(TxError::Overflow)?;

    state.nonces.insert(from, new_nonce);

    if new_from == 0 {
        state.balances.remove(&from);
    } else {
        state.balances.insert(from, new_from);
    }

    if new_to == 0 {
        state.balances.remove(&to);
    } else {
        state.balances.insert(to, new_to);
    }

    Ok(())
}
