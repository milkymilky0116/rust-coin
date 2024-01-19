use axum::{routing::get, Router};

mod blockchain;

async fn home() -> String {
    "This is Home!".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
