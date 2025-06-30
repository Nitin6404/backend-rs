use axum::{
    Router,
    routing::{post, get, delete},
    extract::{Multipart, Path, RequestPartsExt, TypedHeader},
    http::StatusCode,
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use uuid::Uuid;
use std::{fs, env};
use serde_json::json;
use crate::{utils::verify_jwt, db::DB};

pub fn routes() -> Router {
    Router::new()
        .route("/files/upload", post(upload_file))
        .route("/files", get(list_files))
        .route("/files/:id", get(download_file).delete(delete_file))
}

async fn auth_user(headers: axum::http::HeaderMap) -> Option<i64> {
    let auth = headers.get("Authorization")?.to_str().ok()?;
    let token = auth.strip_prefix("Bearer ")?;
    verify_jwt(token)
}

async fn upload_file(
    headers: axum::http::HeaderMap,
    mut multipart: Multipart
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user_id = auth_user(headers).ok_or(StatusCode::UNAUTHORIZED)?;

    let upload_dir = env::var("UPLOAD_DIR").unwrap_or("uploads".to_string());
    fs::create_dir_all(&upload_dir).unwrap();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap_or("file").to_string();
        let data = field.bytes().await.unwrap();
        let id = Uuid::new_v4().to_string();
        let path = format!("{}/{}", upload_dir, &id);
        fs::write(&path, &data).unwrap();

        sqlx::query("INSERT INTO files (id, user_id, filename, path, uploaded_at) VALUES (?, ?, ?, ?, ?)")
            .bind(&id)
            .bind(user_id)
            .bind(&filename)
            .bind(&path)
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(DB.get().unwrap())
            .await.unwrap();

        return Ok(Json(json!({ "id": id, "filename": filename })));
    }

    Err(StatusCode::BAD_REQUEST)
}

async fn list_files(headers: axum::http::HeaderMap) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let user_id = auth_user(headers).ok_or(StatusCode::UNAUTHORIZED)?;

    let rows = sqlx::query!("SELECT id, filename, uploaded_at FROM files WHERE user_id = ?", user_id)
        .fetch_all(DB.get().unwrap())
        .await.unwrap();

    let files = rows
        .into_iter()
        .map(|f| json!({ "id": f.id, "filename": f.filename, "uploaded_at": f.uploaded_at }))
        .collect();

    Ok(Json(files))
}

async fn download_file(Path(id): Path<String>) -> Result<(axum::http::HeaderMap, Vec<u8>), StatusCode> {
    let row = sqlx::query!("SELECT path, filename FROM files WHERE id = ?", id)
        .fetch_optional(DB.get().unwrap())
        .await.unwrap();

    if let Some(file) = row {
        let data = fs::read(&file.path).map_err(|_| StatusCode::NOT_FOUND)?;
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("Content-Disposition", format!("attachment; filename=\"{}\"", file.filename).parse().unwrap());
        Ok((headers, data))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_file(Path(id): Path<String>, headers: axum::http::HeaderMap) -> StatusCode {
    let user_id = auth_user(headers).ok_or(StatusCode::UNAUTHORIZED).unwrap();

    let row = sqlx::query!("SELECT path FROM files WHERE id = ? AND user_id = ?", id, user_id)
        .fetch_optional(DB.get().unwrap())
        .await.unwrap();

    if let Some(file) = row {
        let _ = fs::remove_file(&file.path);
        let _ = sqlx::query!("DELETE FROM files WHERE id = ?", id)
            .execute(DB.get().unwrap())
            .await;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
