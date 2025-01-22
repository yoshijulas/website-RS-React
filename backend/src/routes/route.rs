use std::time::Duration;

use super::{
    admin::{get_role, get_users, patch_users},
    login::login,
    profile::{get_profile, patch_profile},
    signup::sign_up,
    validate::validate_token,
};
use crate::db::{self, migration};
use axum::{
    Router,
    http::Method,
    routing::{get, patch, post},
};
use tower::limit::RateLimitLayer;
use tower_http::cors::{Any, CorsLayer};

pub async fn create_routes() -> Router {
    let rate_limit_layer = RateLimitLayer::new(100, Duration::from_secs(60));
    let pool = db::establish_connection().await;
    migration(&pool).await;

    let origins = [
        "http://localhost:3000".parse().unwrap(),
        "http://frontend:3000".parse().unwrap(),
        "http://localhost:5173".parse().unwrap(),
        "http://frontend:5173".parse().unwrap(),
    ];

    let methods: Vec<Method> = ["GET", "POST", "PATCH", "DELETE", "OPTIONS"]
        .iter()
        .map(|s| s.parse::<Method>().unwrap())
        .collect();

    Router::new()
        .route("/signup", post(sign_up))
        .route("/login", post(login))
        .route("/users/{user_id}", get(get_profile).patch(patch_profile))
        .route("/validate", get(validate_token))
        .route("/role", get(get_role))
        .route("/admin/users", get(get_users))
        .route("/admin/users/{user_id}", patch(patch_users))
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_headers(Any)
                .allow_methods(methods),
        )
}
