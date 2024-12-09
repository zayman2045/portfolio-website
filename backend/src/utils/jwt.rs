//! JWT utilities.

use chrono::{Duration, Utc};
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// The claims for the JWT token.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize, // Expiration time
    iat: usize, // Issued at time
}

/// Generate a JWT token.
pub fn new_jwt() -> Result<String, StatusCode> {
    // Set the expiration time
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::days(1);
    let exp = (now + expires_in).timestamp() as usize;
    let claims = Claims { exp, iat };

    // Generate the token using the secret
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in environment");
    let key = EncodingKey::from_secret(secret.as_bytes());
    let token = encode(&Header::default(), &claims, &key);

    // Return the token or an error
    token.map_err(|_e| {
        eprintln!("Failed to generate JWT token");
        StatusCode::INTERNAL_SERVER_ERROR})
}

/// Validate a JWT token.
pub fn validate_jwt(token: &str) -> Result<(), StatusCode> {
    // Get the secret key and validation settings
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in environment");
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    // Decode the token and return the result
    decode::<Claims>(token, &key, &validation)
        .map(|_| ())
        .map_err(|_| {
            eprintln!("Failed to validate JWT token");
            StatusCode::FORBIDDEN})
}
