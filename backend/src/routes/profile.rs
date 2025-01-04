use crate::auth::validate_jwt;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub email: String,
}

pub async fn profile(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<UserProfile>, (StatusCode, String)> {
    let auth_user_id = validate_jwt(&bearer).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "Invalid or expired token".to_string(),
        )
    })?;

    if auth_user_id != user_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let user = sqlx::query_as!(
        UserProfile,
        "SELECT id, username, email FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch user".to_string(),
        )
    })?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    }
}
