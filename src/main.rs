mod block;
mod blockchain;
mod proof_of_work;

fn main() {
    let mut bc = blockchain::Blockchain::new_blockchain();

    bc.add_new_block("send 1 BTC to eltneg".to_string());
    bc.add_new_block("send 2 BTC to eltneg".to_string());

    for block in bc.blocks {
        println!("Block number: {}", block.block_number);
        println!("Prev. hash: {}", block.prev_block_hash);
        println!("Data: {}", block.data);
        println!("Hash: {:?}", block.hash);
        println!("Nonce: {:?}", block.nonce);
        println!();
    }
}
