mod db;

use axum::{Router, routing::get, extract::State, Json};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use db::{AppState, insert_server_check, ServerCheck};
use sysinfo::{System, SystemExt, CpuExt};
use sqlx::postgres::PgPoolOptions;

async fn health_handler(State(state): State<AppState>) -> Json<String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
    let memory_usage = sys.used_memory() as f32 / sys.total_memory() as f32 * 100.0;

    // Simulate ping (real response time can be measured on frontend)
    let response_time_ms = 100; // Dummy value
    let status = "online";

    let result = insert_server_check(&state.db, response_time_ms, status, cpu_usage, memory_usage).await;

    match result {
        Ok(_) => tracing::info!("✅ Inserted health check into DB"),
        Err(e) => tracing::error!("❌ Failed to insert health check: {}", e),
    }

    Json(format!("System OK. CPU: {:.1}%, RAM: {:.1}%", cpu_usage, memory_usage))
}

pub async fn get_stats(State(state): State<AppState>) -> Json<Vec<ServerCheck>> {
    match sqlx::query_as!(
        ServerCheck,
        "SELECT timestamp, response_time_ms, status, cpu_usage, memory_usage FROM server_checks ORDER BY timestamp DESC"
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(rows) => {
            tracing::info!("✅ Fetched {} rows from DB", rows.len());
            Json(rows)
        }
        Err(e) => {
            tracing::error!("❌ Failed to fetch stats: {}", e);
            Json(vec![]) // Return empty array on failure
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // 🔧 Initialize logger
    tracing_subscriber::fmt()
        .with_env_filter("info") // or "debug", or use RUST_LOG env var
        .init();


    let current_dir = std::env::current_dir().unwrap();
    tracing::info!("Current working directory: {:?}", current_dir);


    let db_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    let db = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("DB connection failed");

    // Run migrations (recommended)
    sqlx::migrate!("./migrations").run(&db).await.unwrap();
    tracing::info!("✅ Database connected and migrations ran successfully");

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/stats", get(get_stats))
        .with_state(AppState { db })
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("🚀 Listening on http://{}", addr);

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
