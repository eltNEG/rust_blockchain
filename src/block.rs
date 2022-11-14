use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub block_number: u64,
    pub data: String,
    pub prev_block_hash: String,
    pub hash: String,
}

impl Block {
    pub fn sethash(&mut self) {
        let mut hasher = Sha256::new();
        let header: Vec<u8> = [
            self.prev_block_hash.as_bytes(),
            self.data.as_bytes(),
            self.block_number.to_string().as_bytes(),
        ]
        .concat();
        hasher.update(header);
        self.hash = format!("{:x}", hasher.finalize());
    }

    pub fn new(block_number: u64, data: String, prev_block_hash: String) -> Block {
        Block {
            block_number,
            data,
            prev_block_hash,
            hash: String::from(""),
        }
    }

    pub fn new_genesis_block() -> Block {
        let mut b = Block::new(
            0,
            String::from("First block; Hurray!"),
            String::from("000000001")
        );
        b.sethash();
        b
    }
}


