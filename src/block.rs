use crate::proof_of_work;

#[derive(Debug)]
pub struct Block {
    pub block_number: u64,
    pub data: String,
    pub prev_block_hash: String,
    pub hash: String,
    pub nonce: i64,
}

impl Block {

    pub fn new(block_number: u64, data: String, prev_block_hash: String) -> Block {
        let b = Block {
            block_number,
            data,
            prev_block_hash,
            hash: String::from(""),
            nonce: 0,
        };
        let mut pow = proof_of_work::ProofOfWork::new(b);
        pow.run();
        pow.block
    }


    pub fn new_genesis_block() -> Block {
        Block::new(
            0,
            String::from("First block; Hurray!"),
            String::from("000000001")
        )
    }
}


