use std::sync::Mutex;

use blockchain::chain::BlockChain;

pub mod blockchain;
static BLOCK_CHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::new());

fn main() {}
