// src\api\routes.rs
use axum::{
    routing::{get, post},
    Router,
};

use crate::api::handlers;
use crate::api::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/ready", get(handlers::ready))
        .route("/jobs/bronze/ref", post(handlers::submit_bronze_ref_job))
        .route("/jobs/bronze/daily", post(handlers::submit_bronze_daily_job))
        .route("/runs", get(handlers::list_runs))
        .route("/runs/:id", get(handlers::get_run_by_id))
        .with_state(state)
}
    