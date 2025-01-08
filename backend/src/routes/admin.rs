use crate::{
    auth::{hash_password, validate_jwt},
    errors::AppError,
};
use axum::{
    Json,
    extract::{Path, State},
};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Role {
    pub username: String,
    pub id: i32,
    pub role_name: String,
}

pub enum RoleId {
    User = (1),
    Admin = (2),
    Moderator = (3),
}

impl RoleId {
    const fn value(self) -> i32 {
        self as i32
    }
}

pub enum StatusId {
    Active = (1),
    Restricted = (2),
    Banned = (3),
}

impl StatusId {
    const fn value(self) -> i32 {
        self as i32
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub account_status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialUserProfile {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role_name: Option<String>,
    pub account_status: Option<String>,
}

pub async fn is_admin(
    State(pool): State<PgPool>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> bool {
    let role_response = get_role(State(pool), TypedHeader(Authorization(bearer)))
        .await
        .unwrap();

    role_response.0.role_name == "admin"
}

pub async fn get_role(
    State(pool): State<PgPool>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Role>, AppError> {
    let user_id =
        validate_jwt(&bearer).map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    let result = sqlx::query_as!(
        Role,
        "
        SELECT users.username, roles.id AS id, roles.role_name
        FROM users
        INNER JOIN roles ON users.role_id = roles.id
        WHERE users.id = $1
        LIMIT 1
        ",
        user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|err| AppError::InternalServerError(format!("An unexpected error occurred: {err}")))?;

    match result {
        Some(role) => Ok(Json(role)),
        None => Err(AppError::NotFound("Role not found".to_string())),
    }
}

pub async fn get_users(
    State(pool): State<PgPool>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<User>>, AppError> {
    if !(is_admin(State(pool.clone()), TypedHeader(Authorization(bearer))).await) {
        return Err(AppError::Unauthorized("Unauthorized".to_string()));
    }

    let users = sqlx::query_as!(
        User,
        "
        SELECT users.id, users.username, users.email, roles.role_name AS role, status.status_name AS account_status
        FROM users
        INNER JOIN roles ON users.role_id = roles.id
        INNER JOIN account_status AS status ON users.status_id = status.id
        ORDER BY users.id ASC
        LIMIT 5
        "
    ).fetch_all(&pool)
    .await
    .map_err(|err|  AppError::InternalServerError(format!("An unexpected error occurred: {err}")))?;

    if users.is_empty() {
        return Err(AppError::NotFound("Users not found".to_string()));
    }

    Ok(Json(users))
}

pub async fn patch_users(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<PartialUserProfile>,
) -> Result<String, AppError> {
    if !is_admin(State(pool.clone()), TypedHeader(Authorization(bearer))).await {
        return Err(AppError::Unauthorized("Unauthorized".to_string()));
    }

    let result = sqlx::query!(
        "
        SELECT * 
        FROM users 
        WHERE id = $1 
        LIMIT 1
        ",
        user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|err| AppError::InternalServerError(format!("An unexpected error occurred: {err}")))?;

    if result.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    let hashed_password = if payload.password.is_some() {
        Some(hash_password(&payload.password.clone().unwrap()))
    } else {
        None
    };

    let mut role_id = None;
    if let Some(role_name) = payload.role_name {
        role_id = Some(get_role_id(&role_name));
    }

    let mut status_id = None;
    if let Some(status_name) = payload.account_status {
        status_id = Some(get_status_id(&status_name));
    }

    let result = sqlx::query!(
        "
        UPDATE users 
        SET username = COALESCE($1, username), 
            email = COALESCE($2, email), 
            password = COALESCE($3, password),
            role_id = COALESCE($4, role_id),
            status_id = COALESCE($5, status_id)
        WHERE id = $6
        ",
        payload.username,
        payload.email,
        hashed_password,
        role_id,
        status_id,
        user_id
    )
    .execute(&pool)
    .await
    .map_err(|err| AppError::InternalServerError(format!("An unexpected error occurred: {err}")))?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalServerError(
            "User not updated".to_string(),
        ));
    }

    Ok("User updated".to_string())
}

pub fn get_role_id(role_name: &str) -> i32 {
    match role_name.trim().to_lowercase().as_str() {
        "admin" => RoleId::Admin.value(),
        "moderator" => RoleId::Moderator.value(),
        _ => RoleId::User.value(),
    }
}

pub fn get_status_id(account_status: &str) -> i32 {
    match account_status.trim().to_lowercase().as_str() {
        "restricted" => StatusId::Restricted.value(),
        "banned" => StatusId::Banned.value(),
        _ => StatusId::Active.value(),
    }
}
