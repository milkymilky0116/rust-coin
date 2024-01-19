use std::sync::Mutex;

use blockchain::chain::BlockChain;

pub mod blockchain;
static BLOCK_CHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::new());

fn main() {
    let mut chain = BLOCK_CHAIN.lock().unwrap();

    chain.init();
    chain.add_block("Second Block".to_string());
    chain.add_block("Third Block".to_string());

    chain.all_blocks().iter().for_each(|block| {
        println!("Data: {}", block.data);
        println!("Hash: {}", block.hash);
        println!("Prev Hash: {}\n\n", block.prev_hash);
    })
}
