use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
}

pub fn create_jwt(user_id: i64) -> String {
    let key = env::var("JWT_SECRET").unwrap();
    let claims = Claims {
        sub: user_id,
        exp: (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(key.as_ref())).unwrap()
}

pub fn verify_jwt(token: &str) -> Option<i64> {
    let key = env::var("JWT_SECRET").unwrap();
    decode::<Claims>(token, &DecodingKey::from_secret(key.as_ref()), &Validation::default())
        .ok()
        .map(|data| data.claims.sub)
}
