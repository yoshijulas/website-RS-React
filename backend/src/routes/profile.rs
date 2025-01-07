use crate::auth::{hash_password, validate_jwt};
use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub username: String,
    pub email: String,
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PartialUserProfile {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[debug_handler]
pub async fn patch_profile(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<PartialUserProfile>,
) -> Result<String, AppError> {
    let auth_user_id = validate_jwt(&bearer)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

    if auth_user_id != user_id {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let user = sqlx::query_as!(
        UserProfile,
        "SELECT username, email, password FROM users WHERE id = $1 LIMIT 1",
        auth_user_id,
    )
    .fetch_optional(&pool)
    .await
    .map_err(|err| AppError::InternalServerError(format!("An unexpected error occurred {err}")))?;

    if user.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    let user = user.unwrap();
    let updated_user = UserProfile {
        username: payload.username.unwrap_or(user.username),
        email: payload.email.unwrap_or(user.email),
        password: hash_password(&payload.password.unwrap_or(user.password)),
    };

    let result = sqlx::query!(
        "UPDATE users SET username = $1, email = $2, password = $3 WHERE id = $4",
        updated_user.username,
        updated_user.email,
        updated_user.password,
        auth_user_id,
    )
    .execute(&pool)
    .await
    .map_err(|err| AppError::InternalServerError(format!("An unexpected error occurred: {err}")))?;

    match result {
        x if x.rows_affected() > 0 => Ok("User updated successfully".to_string()),
        _ => Err(AppError::InternalServerError(
            "Failed to update user".to_string(),
        )),
    }
}

pub async fn get_profile(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<UserProfileResponse>, AppError> {
    let auth_user_id = validate_jwt(&bearer)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

    if auth_user_id != user_id {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let user = sqlx::query_as!(
        UserProfileResponse,
        "SELECT id, username, email FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| AppError::InternalServerError("An unexpected error occurred".to_string()))?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(AppError::NotFound("User not found".to_string())),
    }
}
