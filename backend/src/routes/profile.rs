use super::admin::get_role;
use super::log::log_activity;
use crate::auth::{hash_password, validate_jwt};
use crate::errors::AppError;
use axum::{
    Json,
    extract::{Path, State},
};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub username: String,
    pub email: String,
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PartialUserProfile {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Role {
    id: i32,
    name: String,
}

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
        "
        SELECT username, email, password
        FROM users
        WHERE id = $1
        LIMIT 1
        ",
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
        "
        UPDATE users
        SET username = COALESCE($1, username),
            email = COALESCE($2, email),
            password = COALESCE($3, password)
        WHERE id = $4
        ",
        updated_user.username,
        updated_user.email,
        updated_user.password,
        auth_user_id,
    )
    .execute(&pool)
    .await
    .map_err(|err| AppError::InternalServerError(format!("An unexpected error occurred: {err}")))?;

    if result.rows_affected() > 0 {
        log_activity(&pool, user_id, "Profile Updated".to_string()).await?;
        Ok("User updated successfully".to_string())
    } else {
        Err(AppError::InternalServerError(
            "Failed to update user".to_string(),
        ))
    }
}

pub async fn get_profile(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<PartialUserProfile>, AppError> {
    let auth_user_id = validate_jwt(&bearer)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

    if auth_user_id != user_id {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let user = sqlx::query_as!(
        UserProfileResponse,
        "
        SELECT id, username, email
        FROM users
        WHERE id = $1
        ",
        user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| AppError::InternalServerError("An unexpected error occurred".to_string()))?;

    if user.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    let user = user.unwrap();
    let role_json = get_role(State(pool), TypedHeader(Authorization(bearer)))
        .await?
        .0
        .role_name;
    let user = PartialUserProfile {
        role: Some(role_json),
        username: Some(user.username),
        email: Some(user.email),
        id: Some(user.id),
        password: None,
    };

    Ok(Json(user))
}
