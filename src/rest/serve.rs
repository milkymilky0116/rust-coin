use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{blockchain::chain::Block, BLOCK_CHAIN};

#[derive(Serialize)]
pub struct URLDescription {
    url: String,
    method: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    payload: Option<String>,
}

#[derive(Deserialize)]
pub struct BlockBody {
    data: String,
}
#[derive(Serialize)]
pub struct ErrMsg {
    msg: String,
}

pub async fn documentation() -> (StatusCode, Json<Vec<URLDescription>>) {
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

pub async fn get_blocks() -> (StatusCode, Json<Vec<Block>>) {
    let chain = BLOCK_CHAIN.lock().unwrap();
    let blocks = chain.all_blocks();

    (StatusCode::OK, Json(blocks))
}

pub async fn add_block(Json(payload): Json<BlockBody>) -> StatusCode {
    let mut chain = BLOCK_CHAIN.lock().unwrap();
    chain.add_block(payload.data);
    StatusCode::CREATED
}

pub async fn get_block(
    Path(id): Path<usize>,
) -> Result<(StatusCode, Json<Block>), (StatusCode, Json<ErrMsg>)> {
    let chain = BLOCK_CHAIN.lock().unwrap();
    let block = chain.get_block(id);
    match block {
        Some(block) => Ok((StatusCode::OK, Json(block.clone()))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrMsg {
                msg: "Block not found".to_string(),
            }),
        )),
    }
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(documentation))
        .route("/blocks", get(get_blocks))
        .route("/blocks", post(add_block))
        .route("/blocks/:id", get(get_block))
}

pub async fn serve(port: usize) {
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, routes()).await.unwrap();
}
