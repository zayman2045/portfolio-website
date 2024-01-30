use chrono::{Duration, Utc};
use hyper::StatusCode;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

// Generate a JWT token.
pub fn new_jwt() -> Result<String, StatusCode> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    let exp = (now + expires_in).timestamp() as usize;
    let claims = Claims { exp, iat };
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in environment");
    let key = EncodingKey::from_secret(secret.as_bytes());
    let token = encode(&Header::default(), &claims, &key);
    token.map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn validate_jwt(token: &str) -> Result<bool, StatusCode> {
    todo!()
}