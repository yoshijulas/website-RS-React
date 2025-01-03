use crate::{
    auth::{generate_jwt, verify_password},
    entities::{prelude::*, users},
};
use axum::{Json, extract::State, http::StatusCode};
use diesel_async::AsyncPgConnection;
use serde::{Deserialize, Serialize};

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
    State(pool): State<&mut AsyncPgConnection>,
    Json(payload): Json<LoginInput>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user = Users::find()
        .filter(users::Column::Email.eq(&payload.email))
        .one(&pool)
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
