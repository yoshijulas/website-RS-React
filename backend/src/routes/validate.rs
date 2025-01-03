use crate::auth::validate_jwt;
use axum::{Json, http::StatusCode};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    user_id: Option<i32>,
    message: String,
}

pub async fn validate_token(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> (StatusCode, Json<ApiResponse>) {
    match validate_jwt(&bearer) {
        Ok(user_id) => (
            StatusCode::OK,
            Json(ApiResponse {
                user_id: Some(user_id),
                message: "Valid token".to_string(),
            }),
        ),
        Err(status) => (
            status,
            Json(ApiResponse {
                user_id: None,
                message: "Invalid token".to_string(),
            }),
        ),
    }
}
