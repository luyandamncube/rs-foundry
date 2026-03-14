// src\api\handlers.rs
use axum::{extract::State, Json};

use crate::api::models::HealthResponse;
use crate::api::state::AppState;

pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: state.settings.app_name.clone(),
    })
}
