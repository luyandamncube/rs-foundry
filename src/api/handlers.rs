// src\api\handlers.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::api::models::{
    BronzeJobRequest, BronzeJobResponse, HealthResponse, ReadyResponse, RunListResponse,
    RunResponse,
};
use crate::api::state::AppState;
use crate::core::types::{RunId, RunStatus};
use crate::orchestration::run_registry;

pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: state.settings.app_name.clone(),
        environment: state.settings.app_env.clone(),
    })
}

pub async fn ready(State(state): State<AppState>) -> Json<ReadyResponse> {
    Json(ReadyResponse {
        status: "ready".to_string(),
        service: state.settings.app_name.clone(),
        checks: vec![
            "config_loaded".to_string(),
            "api_routes_registered".to_string(),
            "db_pool_initialized".to_string(),
        ],
    })
}

pub async fn submit_bronze_ref_job(
    State(state): State<AppState>,
    Json(_request): Json<BronzeJobRequest>,
) -> Result<Json<BronzeJobResponse>, (StatusCode, String)> {
    let run_id = RunId::new();

    run_registry::create_run(&state.db_pool, &run_id, "bronze_ref")
        .await
        .map_err(internal_error)?;

    run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Running, None)
        .await
        .map_err(internal_error)?;

    match crate::jobs::bronze_ref_job::execute_with_run_id(run_id.clone()) {
        Ok(result) => {
            run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Succeeded, None)
                .await
                .map_err(internal_error)?;

            Ok(Json(BronzeJobResponse {
                status: "submitted".to_string(),
                job_name: "bronze_ref".to_string(),
                run_id: run_id.0.to_string(),
                source_name: result.source_name,
                raw_path: result.raw_path.display().to_string(),
                bronze_path: result.bronze_path.display().to_string(),
                record_count: result.record_count,
            }))
        }
        Err(e) => {
            let message = e.to_string();

            let _ = run_registry::update_run_status(
                &state.db_pool,
                &run_id,
                RunStatus::Failed,
                Some(&message),
            )
            .await;

            Err((StatusCode::INTERNAL_SERVER_ERROR, message))
        }
    }
}

pub async fn submit_bronze_daily_job(
    State(state): State<AppState>,
    Json(_request): Json<BronzeJobRequest>,
) -> Result<Json<BronzeJobResponse>, (StatusCode, String)> {
    let run_id = RunId::new();

    run_registry::create_run(&state.db_pool, &run_id, "bronze_daily")
        .await
        .map_err(internal_error)?;

    run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Running, None)
        .await
        .map_err(internal_error)?;

    match crate::jobs::bronze_daily_job::execute_with_run_id(run_id.clone()) {
        Ok(result) => {
            run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Succeeded, None)
                .await
                .map_err(internal_error)?;

            Ok(Json(BronzeJobResponse {
                status: "submitted".to_string(),
                job_name: "bronze_daily".to_string(),
                run_id: run_id.0.to_string(),
                source_name: result.source_name,
                raw_path: result.raw_path.display().to_string(),
                bronze_path: result.bronze_path.display().to_string(),
                record_count: result.record_count,
            }))
        }
        Err(e) => {
            let message = e.to_string();

            let _ = run_registry::update_run_status(
                &state.db_pool,
                &run_id,
                RunStatus::Failed,
                Some(&message),
            )
            .await;

            Err((StatusCode::INTERNAL_SERVER_ERROR, message))
        }
    }
}

pub async fn get_run_by_id(
    State(state): State<AppState>,
    Path(run_id): Path<String>,
) -> Result<Json<RunResponse>, (StatusCode, String)> {
    let uuid = Uuid::parse_str(&run_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("invalid run id: {e}")))?;

    let run = run_registry::get_run(&state.db_pool, &RunId(uuid))
        .await
        .map_err(internal_error)?;

    Ok(Json(to_run_response(run)))
}

pub async fn list_runs(
    State(state): State<AppState>,
) -> Result<Json<RunListResponse>, (StatusCode, String)> {
    let runs = run_registry::list_runs(&state.db_pool, 50)
        .await
        .map_err(internal_error)?;

    let response = RunListResponse {
        runs: runs.into_iter().map(to_run_response).collect(),
    };

    Ok(Json(response))
}

fn to_run_response(run: crate::core::types::RunRecord) -> RunResponse {
    RunResponse {
        run_id: run.run_id.0.to_string(),
        job_name: run.job_name,
        status: match run.status {
            RunStatus::Created => "created".to_string(),
            RunStatus::Running => "running".to_string(),
            RunStatus::Succeeded => "succeeded".to_string(),
            RunStatus::Failed => "failed".to_string(),
        },
        created_at: run.created_at.to_rfc3339(),
        updated_at: run.updated_at.to_rfc3339(),
        error_message: run.error_message,
    }
}

fn internal_error<E: ToString>(error: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}
