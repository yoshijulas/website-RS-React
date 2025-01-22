use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use headers::authorization::Bearer;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: i64,
}

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password).unwrap()
}

pub fn generate_jwt(user_id: i32) -> String {
    let expiration_hours = 24;
    let expiration = Utc::now() + Duration::hours(expiration_hours);
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
    )
    .unwrap()
}

pub fn validate_jwt(token: &Bearer) -> Result<i32, StatusCode> {
    let token = token.token();
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
        &Validation::default(),
    ) {
        Ok(token_data) => token_data
            .claims
            .sub
            .parse()
            .map_err(|_| StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
