use std::sync::Mutex;

use blockchain::chain::BlockChain;
use explorer::serve::serve;
mod blockchain;
mod explorer;
static BLOCK_CHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::new());

#[tokio::main]
async fn main() {
    {
        let mut chain = BLOCK_CHAIN.lock().unwrap();
        chain.init();
        chain.add_block("Second Block".to_string());
        chain.add_block("Third Block".to_string());
    }

    serve(4000).await
}
