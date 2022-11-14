mod block;

fn main() {
    let mut block = block::new(2, String::from("data"), String::from("PrevBlockHash"));
    block.sethash();
    println!("{}", block.hash);
}
