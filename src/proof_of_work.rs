use crate::block;
use num::{BigInt, FromPrimitive};
use num_bigint::Sign;
use sha2::{Digest, Sha256};

const MAX_NONCE: i64 = i64::MAX;
const TARGET_BITS: i64 = 16;

pub struct ProofOfWork {
    pub block: block::Block,
    target: BigInt,
}

impl ProofOfWork {
    pub fn new(b: block::Block) -> ProofOfWork {
        let mut target = BigInt::from_i64(1).unwrap();
        target = target << (256 - TARGET_BITS);
        ProofOfWork { block: b, target }
    }

    pub fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let header: Vec<u8> = [
            self.block.prev_block_hash.as_bytes(),
            self.block.data.as_bytes(),
            self.block.block_number.to_string().as_bytes(),
            TARGET_BITS.to_string().as_bytes(),
            nonce.to_string().as_bytes(),
        ]
        .concat();
        header
    }

    pub fn run(&mut self) {
        let mut hashstr: String = "".to_string();
        let mut nonce: i64 = 0;

        print!("Mining the block containing \"{}\"\n", self.block.data);

        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            let mut hasher = Sha256::new();
            hasher.update(data);
            
            let h = hasher.finalize();
            let hash: BigInt = BigInt::from_bytes_be(Sign::Plus, &h);
            hashstr = format!("{:x}", h);
            print!("\r{}", hashstr);
            
            if hash < self.target {
                break;
            }
            nonce = nonce + 1;
        }
        self.block.nonce = nonce;
        self.block.hash = hashstr;
        print!("\n\n");
        
    }

    pub fn _validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hashstr = format!("{:x}", hasher.finalize());
        let hash: BigInt = hashstr.parse::<BigInt>().unwrap();
        hash < self.target
    }
}
