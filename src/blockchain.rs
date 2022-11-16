use std::convert::TryInto;

use crate::block;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<block::Block>,
}

impl Blockchain {
    pub fn new_blockchain() -> Blockchain {
        Blockchain{
            blocks: vec![block::Block::new_genesis_block()],
        }
    }
    
    pub fn add_new_block(&mut self, data: String) {
        let n = self.blocks.len();
        let l: u64 = self.blocks.len().try_into().expect("block length should be greater than 1");
        let prev_block = self.blocks.get(n-1).expect("previous block is empty");
        let prev_block_hash = &prev_block.hash;
        let new_block = block::Block::new(l, data, prev_block_hash.to_string());
        self.blocks.push(
            new_block,
        );
    }
}