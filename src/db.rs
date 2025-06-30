use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::sync::OnceLock;
use std::env;

pub static DB: OnceLock<SqlitePool> = OnceLock::new();

pub async fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    // Get database URL from environment or use default
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./data/database.sqlite".to_string());
    
    println!("Using database at: {}", db_url);
    
    // Ensure the data directory exists
    if let Some(db_path) = db_url.strip_prefix("sqlite:").and_then(|s| s.split('?').next()) {
        if let Some(parent) = std::path::Path::new(db_path).parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
    }

    // Create a new connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    
    // Run migrations
    println!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    // Store the connection pool
    DB.set(pool).expect("Failed to set database pool");
    
    println!("✅ Database initialized successfully!");
    Ok(())
}
