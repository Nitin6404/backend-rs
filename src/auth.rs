use axum::{
    routing::post,
    Json, Router,
};
use axum::extract::Json as AxumJson;
use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};
use crate::{utils::*, db::DB};

#[derive(Deserialize)]
struct RegisterInput {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginInput {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

async fn register(Json(input): AxumJson<RegisterInput>) -> Json<AuthResponse> {
    let hash = hash(&input.password, 10).unwrap();
    let user_id = sqlx::query("INSERT INTO users (email, password) VALUES (?, ?)")
        .bind(&input.email)
        .bind(&hash)
        .execute(DB.get().unwrap())
        .await
        .unwrap()
        .last_insert_rowid();

    let token = create_jwt(user_id);
    Json(AuthResponse { token })
}

async fn login(Json(input): AxumJson<LoginInput>) -> Result<Json<AuthResponse>, &'static str> {
    let row = sqlx::query!("SELECT id, password FROM users WHERE email = ?", input.email)
        .fetch_optional(DB.get().unwrap())
        .await.unwrap();

    match row {
        Some(user) if verify(&input.password, &user.password).unwrap() => {
            let token = create_jwt(user.id);
            Ok(Json(AuthResponse { token }))
        },
        _ => Err("Invalid credentials"),
    }
}
