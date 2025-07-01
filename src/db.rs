use sqlx::{PgPool, postgres::PgQueryResult};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct ServerCheck {
    pub timestamp: String,
    pub response_time_ms: Option<i64>,    // <-- changed from i32 to i64
    pub status: Option<String>,
    pub cpu_usage: Option<f64>,           // <-- changed from f32 to f64
    pub memory_usage: Option<f64>,        // <-- changed from f32 to f64
}



pub async fn insert_server_check(
    db: &PgPool,
    response_time_ms: i32,
    status: &str,
    cpu_usage: f32,
    memory_usage: f32,
) -> Result<PgQueryResult, sqlx::Error> {
    let timestamp = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO server_checks (timestamp, response_time_ms, status, cpu_usage, memory_usage)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(timestamp)
    .bind(response_time_ms)
    .bind(status)
    .bind(cpu_usage)
    .bind(memory_usage)
    .execute(db)
    .await
}

