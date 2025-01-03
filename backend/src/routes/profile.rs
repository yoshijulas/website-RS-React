use crate::auth::validate_jwt;
use crate::entities::users::Entity as Users;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use axum_extra::TypedHeader;
use diesel_async::AsyncPgConnection;
use headers::{Authorization, authorization::Bearer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub email: String,
}

pub async fn profile(
    State(pool): State<&mut AsyncPgConnection>,
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

    let user = Users::find_by_id(user_id)
        .one(&pool)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    match user {
        Some(user) => Ok(Json(UserProfile {
            id: user.id,
            username: user.username,
            email: user.email,
        })),
        None => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    }
}
