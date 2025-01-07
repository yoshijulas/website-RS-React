use crate::{auth::validate_jwt, errors::AppError};
use axum::{Json, extract::State};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct ApiResponse {
    user_id: Option<i32>,
    message: String,
}

pub async fn validate_token(
    State(pool): State<PgPool>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<ApiResponse>, AppError> {
    let token_data = validate_jwt(&bearer);

    if token_data.is_err() {
        return Err(AppError::Unauthorized("Invalid token".to_string()));
    }

    let user_id = token_data.unwrap();
    let result = sqlx::query!("SELECT COUNT(1) FROM users WHERE id = $1 LIMIT 1", user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| AppError::InternalServerError("An unexpected error occurred".to_string()))?;

    if result.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    Ok(Json(ApiResponse {
        user_id: Some(user_id),
        message: "Token is valid".to_string(),
    }))
}
