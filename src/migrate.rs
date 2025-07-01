use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let db = PgPoolOptions::new()
        .connect("postgres://postgres:password@localhost:5432/monitor_db")
        .await
        .expect("Failed to connect to DB");

        println!("Running migrations from ./migrations...");

        sqlx::migrate!("./migrations")
            .run(&db)
            .await
            .unwrap();
        
        println!("✅ Migrations applied successfully.");

}
