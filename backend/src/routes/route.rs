use super::{
    login::login,
    profile::{get_profile, patch_profile},
    signup::sign_up,
    validate::validate_token,
};
use crate::db::{self, migration};
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

pub async fn create_routes() -> Router {
    let pool = db::establish_connection().await;
    migration(&pool).await;

    let origins = [
        "http://localhost:3000".parse().unwrap(),
        "http://frontend:3000".parse().unwrap(),
        "http://localhost:5173".parse().unwrap(),
        "http://frontend:5173".parse().unwrap(),
    ];

    Router::new()
        .route("/signup", post(sign_up))
        .route("/login", post(login))
        .route("/users/{user_id}", get(get_profile).patch(patch_profile))
        .route("/validate", get(validate_token))
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_headers(Any)
                .allow_methods(Any),
        )
}
