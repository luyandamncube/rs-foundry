// src\api\routes.rs
use axum::{routing::get, Router};

use crate::api::handlers;
use crate::api::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .with_state(state)
}
