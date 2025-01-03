use crate::auth::hash_password;
use crate::models::schema::users::{self, dsl::*};
use axum::http::{StatusCode, status};
use axum::{Json, extract::State};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

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
    State(pool): State<&mut AsyncPgConnection>,
    Json(payload): Json<SignUpInput>,
) -> Result<Json<ApiResponse>, (StatusCode, Json<ApiResponse>)> {
    // Hash the password
    let hashed_password = hash_password(&payload.password);

    let existing_user = users
        .filter(email.eq(&payload.email))
        .limit(1)
        .select(id)
        .load::<id>(&mut pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "Failed to query the database".to_string(),
                    created: false,
                }),
            )
        })?;

    if existing_user.len() > 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                message: "Username is already in use".to_string(),
                created: false,
            }),
        ));
    }

    let new_user = users {
        username: payload.username,
        email: payload.email,
        password: hashed_password,
    };

    match diesel::insert_into(users)
        .values(new_user)
        .get_result(&mut pool)
        .await
    {
        Ok(_) => Ok(Json(ApiResponse {
            message: "User created successfully".to_string(),
            created: true,
        })),
        Err(err) => {
            eprintln!("Database error: {err:?}");
            Err((
                StatusCode::NOT_MODIFIED,
                Json(ApiResponse {
                    message: "Failed to create user".to_string(),
                    created: false,
                }),
            ))
        }
    }
}
