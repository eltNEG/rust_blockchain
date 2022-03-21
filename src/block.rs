use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    timestamp: u64,
    data: String,
    prev_block_hash: String,
    hash: String,
}

impl Block {
    pub fn sethash(&mut self) {
        let mut hasher = Sha256::new();
        let header: Vec<u8> = [
            self.prev_block_hash.as_bytes(),
            self.data.as_bytes(),
            self.timestamp.to_string().as_bytes(),
        ]
        .concat();
        hasher.update(header);
        self.hash = format!("{:x}", hasher.finalize());
    }
}

pub fn new(timestamp: u64, data: String, prev_block_hash: String) -> Block {
    Block {
        timestamp,
        data,
        prev_block_hash,
        hash: String::from(""),
    }
}
