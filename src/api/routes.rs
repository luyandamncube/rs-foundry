// src/api/routes.rs
use axum::{
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::docs::ApiDoc;
use crate::api::handlers;
use crate::api::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(handlers::health))
        .route("/ready", get(handlers::ready))
        .route("/jobs/bronze/ref", post(handlers::submit_bronze_ref_job))
        .route("/jobs/bronze/daily", post(handlers::submit_bronze_daily_job))
        .route("/jobs/silver/ref", post(handlers::submit_silver_ref_job))
        .route("/jobs/silver/daily", post(handlers::submit_silver_daily_job))
        .route("/jobs/silver/conformed", post(handlers::submit_silver_conformed_job))
        .route("/runs", get(handlers::list_runs))
        .route("/runs/{id}", get(handlers::get_run_by_id))
        .with_state(state)
}