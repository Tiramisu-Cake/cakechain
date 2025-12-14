use std::collections::HashMap;

use crate::core::{
    basics::{BlockHash, BlockHeight},
    block::BlockError,
    state::State,
    transaction::ChainId,
};

use super::block::Block;

pub struct Node {
    tip_hash: BlockHash,
    tip_height: BlockHeight,
    state: State,
    blocks: HashMap<BlockHash, Block>,
}

impl Node {
    pub fn get_tip(&self) -> (BlockHash, BlockHeight) {
        (self.tip_hash, self.tip_height)
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

    pub fn get_block(&self, hash: &BlockHash) -> Option<&Block> {
        self.blocks.get(hash)
    }

    pub fn apply_block(&self, block: &Block, chain_id: ChainId) -> Result<(), BlockError> {
        // if self.tip_hash != block.parent_hash {
        //     let err = BlockError::WrongParent {
        //         expected: self.tip_hash,
        //         got: block.parent_hash,
        //     };
        //     return Err(err);
        // }
        //
        // if self.tip_height + 1 != block.height {
        //     let err = BlockError::WrongParent {
        //         expected: self.tip_height + 1,
        //         got: block.height,
        //     };
        //     return Err(err);
        // }
        unimplemented!();

        Ok(())
    }
}
