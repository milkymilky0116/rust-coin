use std::sync::Mutex;

use blockchain::chain::BlockChain;
use lazy_static::lazy_static;
mod blockchain;
mod cli;
mod db;
mod explorer;
mod rest;
static BLOCK_CHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::new());

lazy_static! {
    static ref DB: Mutex<sled::Db> = Mutex::new(db::init::new());
}
#[tokio::main]
async fn main() {
    {
        let mut chain = BLOCK_CHAIN.lock().unwrap();
        chain.init();
        chain.add_block("Second Block".to_string());
        chain.add_block("Third Block".to_string());
    }

    //cli::serve::serve().await
}
