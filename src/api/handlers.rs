// src/api/handlers.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::api::models::{
    BronzeJobRequest, BronzeJobResponse, HealthResponse, ReadyResponse, RunListResponse,
    RunResponse, SilverConformedJobRequest, SilverConformedJobResponse, SilverDailyJobRequest,
    SilverDailyJobResponse, SilverRefJobRequest, SilverRefJobResponse,
};
use crate::api::state::AppState;
use crate::core::types::{RunId, RunStatus};
use crate::orchestration::run_registry;

/// Health check.
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health status", body = HealthResponse)
    ),
    tag = "runner"
)]
pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: state.settings.app_name.clone(),
        environment: state.settings.app_env.clone(),
    })
}

/// Readiness check.
#[utoipa::path(
    get,
    path = "/ready",
    responses(
        (status = 200, description = "Readiness status", body = ReadyResponse)
    ),
    tag = "runner"
)]
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

#[utoipa::path(
    post,
    path = "/jobs/bronze/ref",
    request_body = BronzeJobRequest,
    responses(
        (status = 200, description = "Bronze ref job submitted", body = BronzeJobResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "jobs"
)]
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

#[utoipa::path(
    post,
    path = "/jobs/bronze/daily",
    request_body = BronzeJobRequest,
    responses(
        (status = 200, description = "Bronze daily job submitted", body = BronzeJobResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "jobs"
)]
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

#[utoipa::path(
    post,
    path = "/jobs/silver/ref",
    request_body = SilverRefJobRequest,
    responses(
        (status = 200, description = "Silver ref job submitted", body = SilverRefJobResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "jobs"
)]
pub async fn submit_silver_ref_job(
    State(state): State<AppState>,
    Json(request): Json<SilverRefJobRequest>,
) -> Result<Json<SilverRefJobResponse>, (StatusCode, String)> {
    let upstream_bronze_run_id = Uuid::parse_str(&request.bronze_run_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("invalid bronze_run_id: {e}")))?;

    let run_id = RunId::new();

    run_registry::create_run(&state.db_pool, &run_id, "silver_ref")
        .await
        .map_err(internal_error)?;

    run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Running, None)
        .await
        .map_err(internal_error)?;

    match crate::jobs::silver_ref_job::execute_with_run_id_and_upstream(
        run_id.clone(),
        RunId(upstream_bronze_run_id),
    ) {
        Ok(_result) => {
            run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Succeeded, None)
                .await
                .map_err(internal_error)?;

            Ok(Json(SilverRefJobResponse {
                status: "submitted".to_string(),
                job_name: "silver_ref".to_string(),
                run_id: run_id.0.to_string(),
                bronze_run_id: request.bronze_run_id,
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

#[utoipa::path(
    post,
    path = "/jobs/silver/daily",
    request_body = SilverDailyJobRequest,
    responses(
        (status = 200, description = "Silver daily job submitted", body = SilverDailyJobResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "jobs"
)]
pub async fn submit_silver_daily_job(
    State(state): State<AppState>,
    Json(request): Json<SilverDailyJobRequest>,
) -> Result<Json<SilverDailyJobResponse>, (StatusCode, String)> {
    let upstream_bronze_run_id = Uuid::parse_str(&request.bronze_run_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("invalid bronze_run_id: {e}")))?;

    let run_id = RunId::new();

    run_registry::create_run(&state.db_pool, &run_id, "silver_daily")
        .await
        .map_err(internal_error)?;

    run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Running, None)
        .await
        .map_err(internal_error)?;

    match crate::jobs::silver_daily_job::execute_with_run_id_and_upstream(
        run_id.clone(),
        RunId(upstream_bronze_run_id),
    ) {
        Ok(_result) => {
            run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Succeeded, None)
                .await
                .map_err(internal_error)?;

            Ok(Json(SilverDailyJobResponse {
                status: "submitted".to_string(),
                job_name: "silver_daily".to_string(),
                run_id: run_id.0.to_string(),
                bronze_run_id: request.bronze_run_id,
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

#[utoipa::path(
    post,
    path = "/jobs/silver/conformed",
    request_body = SilverConformedJobRequest,
    responses(
        (status = 200, description = "Silver conformed job submitted", body = SilverConformedJobResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "jobs"
)]
pub async fn submit_silver_conformed_job(
    State(state): State<AppState>,
    Json(request): Json<SilverConformedJobRequest>,
) -> Result<Json<SilverConformedJobResponse>, (StatusCode, String)> {
    let upstream_ref_run_id = Uuid::parse_str(&request.ref_run_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("invalid ref_run_id: {e}")))?;

    let upstream_daily_run_id = Uuid::parse_str(&request.daily_run_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("invalid daily_run_id: {e}")))?;

    let run_id = RunId::new();

    run_registry::create_run(&state.db_pool, &run_id, "silver_conformed")
        .await
        .map_err(internal_error)?;

    run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Running, None)
        .await
        .map_err(internal_error)?;

    match crate::jobs::silver_conformed_job::execute_with_run_id_and_upstreams(
        run_id.clone(),
        RunId(upstream_ref_run_id),
        RunId(upstream_daily_run_id),
    ) {
        Ok(_result) => {
            run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Succeeded, None)
                .await
                .map_err(internal_error)?;

            Ok(Json(SilverConformedJobResponse {
                status: "submitted".to_string(),
                job_name: "silver_conformed".to_string(),
                run_id: run_id.0.to_string(),
                ref_run_id: request.ref_run_id,
                daily_run_id: request.daily_run_id,
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

#[utoipa::path(
    get,
    path = "/runs/{id}",
    params(
        ("id" = String, Path, description = "Run ID")
    ),
    responses(
        (status = 200, description = "Run by ID", body = RunResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "runs"
)]
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

#[utoipa::path(
    get,
    path = "/runs",
    responses(
        (status = 200, description = "List runs", body = RunListResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "runs"
)]
pub async fn list_runs(
    State(state): State<AppState>,
) -> Result<Json<RunListResponse>, (StatusCode, String)> {
    let runs = run_registry::list_runs(&state.db_pool, 50)
        .await
        .map_err(internal_error)?;

    Ok(Json(RunListResponse {
        runs: runs.into_iter().map(to_run_response).collect(),
    }))
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