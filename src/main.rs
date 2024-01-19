use std::sync::Mutex;

use askama::Template;
use axum::{routing::get, Router};
use blockchain::chain::{Block, BlockChain};
mod blockchain;

static BLOCK_CHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::new());
#[derive(Template)]
#[template(path = "../templates/home.html")]
struct HomeTemplate {
    title: String,
    blocks: Vec<Block>,
}

async fn home() -> HomeTemplate {
    let blocks = BLOCK_CHAIN.lock().unwrap().all_blocks();
    HomeTemplate {
        title: "Hello Rust".to_string(),
        blocks,
    }
}

#[tokio::main]
async fn main() {
    {
        let mut chain = BLOCK_CHAIN.lock().unwrap();
        chain.init();
        chain.add_block("Second Block".to_string());
        chain.add_block("Third Block".to_string());
    }
    let app = Router::new().route("/", get(home));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
