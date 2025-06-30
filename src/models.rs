use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub filename: String,
    pub uploaded_at: String,
}
