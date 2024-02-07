use std::{process, sync::Mutex};

use blockchain::chain::BlockChain;
mod blockchain;
mod explorer;
mod rest;
static BLOCK_CHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::new());

fn usage() {
    print!("Welcome to rust-coin\n\n");

    print!("Please use the following commands:\n\n");

    print!("explorer:   Start the HTML Explorer\n\n");

    print!("rest:   Start the REST API\n\n");
    process::exit(1)
}
#[tokio::main]
async fn main() {
    {
        let mut chain = BLOCK_CHAIN.lock().unwrap();
        chain.init();
        chain.add_block("Second Block".to_string());
        chain.add_block("Third Block".to_string());
    }

    let args = std::env::args().nth(1);

    match args {
        Some(arg) => match arg.as_str() {
            "explorer" => println!("explorer"),
            _ => println!("rest"),
        },
        None => usage(),
    }

    //serve(3000).await
}
