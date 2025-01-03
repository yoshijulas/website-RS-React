use super::{login::login, profile::profile, signup::sign_up, validate::validate_token};
use crate::db;
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

pub async fn create_routes() -> Router {
    let mut pool = db::establish_connection().await;

    let origins = ["http://localhost:4000".parse().unwrap()];

    Router::new()
        .route("/signup", post(sign_up))
        .route("/login", post(login))
        .route("/profile/{user_id}", get(profile))
        .route("/validate_token", get(validate_token))
        .with_state(&mut pool)
        .layer(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_headers(Any)
                .allow_methods(Any),
        )
}
