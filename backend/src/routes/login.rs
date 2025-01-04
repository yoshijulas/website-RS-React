use crate::auth::{generate_jwt, verify_password};
use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct LoginInput {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    message: String,
    user_id: Option<i32>,
    token: Option<String>,
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginInput>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user = sqlx::query!(
        "SELECT * FROM users WHERE email = $1 LIMIT 1",
        &payload.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to check if email exists".to_string(),
        )
    })?;

    if user.is_none() {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    }

    let user = user.unwrap();
    if verify_password(&payload.password, &user.password) {
        let token = generate_jwt(user.id);
        return Ok(Json(LoginResponse {
            message: "Login successful".to_string(),
            user_id: Some(user.id),
            token: Some(token),
        }));
    }

    Err((StatusCode::UNAUTHORIZED, "Invalid password".to_string()))
}
