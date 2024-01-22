use std::sync::Mutex;

use askama::Template;
use axum::{
    response::Redirect,
    routing::{get, post},
    Form, Router,
};
use blockchain::chain::{Block, BlockChain};
use serde::Deserialize;
mod blockchain;

static BLOCK_CHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::new());
#[derive(Template)]
#[template(path = "../templates/home.html")]
struct HomeTemplate<'a> {
    title: &'a str,
    blocks: Vec<Block>,
    _parent: &'a Layout,
}

#[derive(Deserialize)]
struct BlockForm {
    block_data: String,
}

#[derive(Template)]
#[template(path = "../templates/add.html")]
struct AddTemplate<'a> {
    title: &'a str,
    _parent: &'a Layout,
}

#[derive(Template)]
#[template(path = "../templates/layout/layout.html")]
struct Layout {}

async fn home() -> HomeTemplate<'static> {
    let blocks = BLOCK_CHAIN.lock().unwrap().all_blocks();
    HomeTemplate {
        title: "Home",
        blocks,
        _parent: &Layout {},
    }
}

async fn get_add() -> AddTemplate<'static> {
    AddTemplate {
        title: "Add Block",
        _parent: &Layout {},
    }
}

async fn post_add(Form(form): Form<BlockForm>) -> Redirect {
    let mut chain = BLOCK_CHAIN.lock().unwrap();
    chain.add_block(form.block_data);
    Redirect::to("/")
}

#[tokio::main]
async fn main() {
    {
        let mut chain = BLOCK_CHAIN.lock().unwrap();
        chain.init();
        chain.add_block("Second Block".to_string());
        chain.add_block("Third Block".to_string());
    }
    let app = Router::new()
        .route("/", get(home))
        .route("/add", get(get_add))
        .route("/add", post(post_add));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
