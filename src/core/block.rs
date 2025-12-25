use crate::core::transaction;

use super::basics::*;
use super::state::StateRoot;
use super::transaction::Transaction;
use super::transaction::TxError;

use sha2::{Digest, Sha256};
pub const BLOCK_DOMAIN_TAG: &[u8; 7] = b"BLOCKv1";

pub struct Block {
    parent_hash: BlockHash,
    height: BlockHeight,
    txs: Vec<Transaction>,
    state_root: StateRoot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockError {
    WrongParent {
        expected: BlockHash,
        got: BlockHash,
    },
    WrongHeight {
        expected: BlockHeight,
        got: BlockHeight,
    },
    InvalidTx {
        index: usize,
        err: TxError,
    },
    BadStateRoot {
        expected: StateRoot,
        got: StateRoot,
    },
}

impl Block {
    pub fn new(
        parent_hash: BlockHash,
        height: BlockHeight,
        txs: Vec<Transaction>,
        state_root: StateRoot,
    ) -> Self {
        Self {
            parent_hash,
            height,
            txs,
            state_root,
        }
    }

    pub fn parent_hash(&self) -> BlockHash {
        self.parent_hash
    }

    pub fn height(&self) -> BlockHeight {
        self.height
    }

    pub fn txs(&self) -> &[Transaction] {
        &self.txs
    }

    pub fn state_root(&self) -> StateRoot {
        self.state_root
    }

    pub fn hash(&self) -> BlockHash {
        // BLOCKv1 || parent_hash || height || tx_count || tx_0_bytes || tx_1_bytes || ... || state_root

        let mut out = Vec::with_capacity(
            BLOCK_DOMAIN_TAG.len() // tag
            + 32 // parent_hash
            + 8 // height
            + 8 // tx_count
            + self.txs.len() * transaction::TX_CANONICAL_BYTES_LENGTH  // tx canonical_bytes
            + 32, // StateRoot
        );

        out.extend_from_slice(BLOCK_DOMAIN_TAG);
        out.extend_from_slice(&self.parent_hash.0);
        out.extend_from_slice(&self.height.0.to_le_bytes());
        out.extend_from_slice(&self.txs.len().to_le_bytes());

        for tx in &self.txs {
            out.extend_from_slice(&tx.canonical_bytes());
        }
        out.extend_from_slice(&self.state_root.0);

        let mut hasher = Sha256::new();
        hasher.update(&out);
        let res = hasher.finalize();

        BlockHash(res.into())
    }
}
