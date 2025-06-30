// mod db;
// mod auth;
// mod files;
// mod models;
// mod utils;

use axum::{Router, routing::get};
// use sqlx::Any;
// use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // dotenv().ok();
    // db::init_db().await.expect("DB init failed");

    let cors = CorsLayer::new()
    .allow_methods(Any)
    .allow_headers(Any)
    .allow_origin(Any);


    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        // .merge(auth::routes())
        // .merge(files::routes())
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Listening on http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
