use std::collections::{BTreeMap, HashMap};

use ed25519_dalek::SigningKey;

use super::block::Block;
use crate::core::{
    basics::{Address, BlockHash, BlockHeight},
    block::BlockError,
    state::{State, StateRoot},
    transaction::ChainId,
};

// Deterministic "treasury" key for genesis.
// Anyone who knows these bytes can sign as the treasury.
const GENESIS_TREASURY_SK_BYTES: [u8; 32] = [
    0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
    0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
];

pub fn genesis_treasury_signing_key() -> SigningKey {
    SigningKey::from_bytes(&GENESIS_TREASURY_SK_BYTES)
}

pub fn genesis_treasury_address() -> Address {
    let sk = genesis_treasury_signing_key();
    let vk = sk.verifying_key();
    Address::from_bytes(vk.as_bytes())
}

fn genesis_state() -> State {
    let mut balances = BTreeMap::new();

    let treasury = genesis_treasury_address();
    balances.insert(treasury, 1);

    State {
        balances,
        nonces: BTreeMap::new(),
    }
}

fn genesis_block() -> Block {
    let state_root = genesis_state().state_root();

    Block::new(BlockHash([0u8; 32]), BlockHeight(0), Vec::new(), state_root)
}

pub struct Node {
    tip_hash: BlockHash,
    tip_height: BlockHeight,
    state: State,
    blocks: HashMap<BlockHash, Block>,
}

impl Node {
    pub fn new() -> Node {
        let block = genesis_block();
        let tip_height = block.height();
        let mut blocks = HashMap::new();
        let tip_hash = block.hash();
        let state = genesis_state();
        blocks.insert(tip_hash, block);

        Node {
            tip_hash,
            tip_height,
            state,
            blocks,
        }
    }
    pub fn get_tip(&self) -> (BlockHash, BlockHeight) {
        (self.tip_hash, self.tip_height)
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

    pub fn get_block(&self, hash: &BlockHash) -> Option<&Block> {
        self.blocks.get(hash)
    }

    pub fn apply_block(&mut self, block: Block, chain_id: ChainId) -> Result<(), BlockError> {
        Self::validate_blockhash(self.tip_hash, block.parent_hash())?;
        Self::validate_blockheight(self.tip_height, block.height())?;

        let mut tmp = self.state.clone();

        for (index, tx) in block.txs().iter().enumerate() {
            super::transaction::apply_tx(&mut tmp, tx, chain_id)
                .map_err(|e| BlockError::InvalidTx { index, err: e })?;
        }

        Self::validate_stateroot(block.state_root(), tmp.state_root())?;
        self.commit_block(tmp, block);

        Ok(())
    }

    fn validate_blockhash(expected: BlockHash, got: BlockHash) -> Result<(), BlockError> {
        if expected != got {
            return Err(BlockError::WrongParent { expected, got });
        }
        Ok(())
    }

    fn validate_blockheight(expected: BlockHeight, got: BlockHeight) -> Result<(), BlockError> {
        let expected = BlockHeight(expected.0 + 1);
        if expected != got {
            return Err(BlockError::WrongHeight { expected, got });
        }
        Ok(())
    }

    fn validate_stateroot(expected: StateRoot, got: StateRoot) -> Result<(), BlockError> {
        if expected != got {
            return Err(BlockError::BadStateRoot { expected, got });
        }
        Ok(())
    }

    fn commit_block(&mut self, new_state: State, block: Block) {
        self.state = new_state;

        let block_hash = block.hash();
        self.tip_hash = block_hash;

        self.tip_height = block.height();
        self.blocks.insert(block_hash, block);
    }
}
