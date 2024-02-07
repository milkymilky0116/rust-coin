use std::sync::Mutex;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use blockchain::chain::{Block, BlockChain};
//use explorer::serve::serve;
use serde::{Deserialize, Serialize};
mod blockchain;
mod explorer;
static BLOCK_CHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::new());

#[derive(Serialize)]
struct URLDescription {
    url: String,
    method: String,
    description: String,
    payload: Option<String>,
}

#[derive(Deserialize)]
struct BlockBody {
    data: String,
}

async fn documentation() -> (StatusCode, Json<Vec<URLDescription>>) {
    let data: Vec<URLDescription> = vec![
        URLDescription {
            url: "http://localhost:3000/".to_string(),
            method: "GET".to_string(),
            description: "See documentation".to_string(),
            payload: None,
        },
        URLDescription {
            url: "http://localhost:3000/blocks".to_string(),
            method: "GET".to_string(),
            description: "See All Blocks".to_string(),
            payload: None,
        },
        URLDescription {
            url: "http://localhost:3000/blocks".to_string(),
            method: "POST".to_string(),
            description: "Add a Block".to_string(),
            payload: Some("data:string".to_string()),
        },
        URLDescription {
            url: "http://localhost:3000/blocks/{id}".to_string(),
            method: "GET".to_string(),
            description: "See a Block".to_string(),
            payload: None,
        },
    ];
    (StatusCode::OK, Json(data))
}

async fn get_blocks() -> (StatusCode, Json<Vec<Block>>) {
    let chain = BLOCK_CHAIN.lock().unwrap();
    let blocks = chain.all_blocks();

    (StatusCode::OK, Json(blocks))
}

async fn add_block(Json(payload): Json<BlockBody>) -> StatusCode {
    let mut chain = BLOCK_CHAIN.lock().unwrap();
    chain.add_block(payload.data);
    StatusCode::CREATED
}

#[tokio::main]
async fn main() {
    {
        let mut chain = BLOCK_CHAIN.lock().unwrap();
        chain.init();
        chain.add_block("Second Block".to_string());
        chain.add_block("Third Block".to_string());
    }

    //serve(4000).await

    let app = Router::new()
        .route("/", get(documentation))
        .route("/blocks", get(get_blocks))
        .route("/blocks", post(add_block));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
