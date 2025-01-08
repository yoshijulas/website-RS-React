use crate::auth::hash_password;
use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct SignUpInput {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
    created: bool,
}

pub async fn sign_up(
    State(pool): State<PgPool>,
    Json(payload): Json<SignUpInput>,
) -> Result<Json<ApiResponse>, (StatusCode, String)> {
    let hashed_password = hash_password(&payload.password);

    let existing_user = sqlx::query!(
        "
        SELECT *
        FROM users
        WHERE email = $1
        LIMIT 1
        ",
        &payload.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create user".to_string(),
        )
    })?;

    if existing_user.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username is already in use".to_string(),
        ));
    }

    sqlx::query!(
        "
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        ",
        &payload.username,
        &payload.email,
        &hashed_password,
    )
    .execute(&pool)
    .await
    .map_err(|x| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create user: {x:?}"),
        )
    })?;

    Ok(Json(ApiResponse {
        message: "User created successfully".to_string(),
        created: true,
    }))
}
