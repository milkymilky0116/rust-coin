use askama::Template;
use axum::{
    response::Redirect,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

use crate::{blockchain::chain::Block, BLOCK_CHAIN};

#[derive(Template)]
#[template(path = "../templates/home.html")]
pub struct HomeTemplate<'a> {
    title: &'a str,
    blocks: Vec<Block>,
    _parent: &'a Layout,
}

#[derive(Deserialize)]
pub struct BlockForm {
    block_data: String,
}

#[derive(Template)]
#[template(path = "../templates/add.html")]
pub struct AddTemplate<'a> {
    title: &'a str,
    _parent: &'a Layout,
}

#[derive(Template)]
#[template(path = "../templates/layout/layout.html")]
pub struct Layout {}

pub async fn home() -> HomeTemplate<'static> {
    let blocks = BLOCK_CHAIN.lock().unwrap().all_blocks();
    HomeTemplate {
        title: "Home",
        blocks,
        _parent: &Layout {},
    }
}

pub async fn get_add() -> AddTemplate<'static> {
    AddTemplate {
        title: "Add Block",
        _parent: &Layout {},
    }
}

pub async fn post_add(Form(form): Form<BlockForm>) -> Redirect {
    let mut chain = BLOCK_CHAIN.lock().unwrap();
    chain.add_block(form.block_data);
    Redirect::to("/")
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/add", get(get_add))
        .route("/add", post(post_add))
}
pub async fn serve(port: usize) {
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes()).await.unwrap()
}
