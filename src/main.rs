use axum::{
    routing::get,
    Router,
    Json,
    http::StatusCode,
};
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct Message {
    message: String,
}

async fn hello() -> Json<Message> {
    let message = "Hello from Rust 🚀".to_string();
    Json(Message {
        message,
    })
}

async fn not_found() -> (StatusCode, Json<Message>) {
    let message = "Not Found".to_string();
    (StatusCode::NOT_FOUND, Json(Message {
        message,
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/not-found", get(not_found))
        .fallback(not_found);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}
