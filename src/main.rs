// mod db;
// mod auth;
// mod files;
// mod models;
// mod utils;

use axum::{Router, routing::get};
// use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
// use tower_http::cors::{CorsLayer};

#[tokio::main]
async fn main() {
    // dotenv().ok();
    // db::init_db().await.expect("DB init failed");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));
        // .merge(auth::routes())
        // .merge(files::routes())
        // .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Listening on http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
