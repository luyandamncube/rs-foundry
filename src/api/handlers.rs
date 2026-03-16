// src/api/handlers.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::api::models::{
    HealthResponse, JobTriggerRequest, JobTriggerResponse, OrchestrationMetadata, ReadyResponse,
    RunListResponse, RunResponse,
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
    request_body = JobTriggerRequest,
    responses(
        (status = 200, description = "Bronze ref job submitted", body = JobTriggerResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "jobs"
)]
pub async fn submit_bronze_ref_job(
    State(state): State<AppState>,
    Json(request): Json<JobTriggerRequest>,
) -> Result<Json<JobTriggerResponse>, (StatusCode, String)> {
    validate_upstream_count("bronze_ref", &request.upstream_run_ids, 0)?;

    let run_id = RunId::new();

    run_registry::create_run(
        &state.db_pool,
        &run_id,
        "bronze_ref",
        request.orchestration.as_ref(),
        &request.upstream_run_ids,
    )
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

            Ok(Json(JobTriggerResponse {
                status: "submitted".to_string(),
                job_name: "bronze_ref".to_string(),
                run_id: run_id.0.to_string(),
                upstream_run_ids: vec![],
                source_name: Some(result.source_name),
                raw_path: Some(result.raw_path.display().to_string()),
                bronze_path: Some(result.bronze_path.display().to_string()),
                silver_path: None,
                record_count: Some(result.record_count),
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
    request_body = JobTriggerRequest,
    responses(
        (status = 200, description = "Bronze daily job submitted", body = JobTriggerResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "jobs"
)]
pub async fn submit_bronze_daily_job(
    State(state): State<AppState>,
    Json(request): Json<JobTriggerRequest>,
) -> Result<Json<JobTriggerResponse>, (StatusCode, String)> {
    validate_upstream_count("bronze_daily", &request.upstream_run_ids, 0)?;

    let run_id = RunId::new();

    run_registry::create_run(
        &state.db_pool,
        &run_id,
        "bronze_daily",
        request.orchestration.as_ref(),
        &request.upstream_run_ids,
    )
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

            Ok(Json(JobTriggerResponse {
                status: "submitted".to_string(),
                job_name: "bronze_daily".to_string(),
                run_id: run_id.0.to_string(),
                upstream_run_ids: vec![],
                source_name: Some(result.source_name),
                raw_path: Some(result.raw_path.display().to_string()),
                bronze_path: Some(result.bronze_path.display().to_string()),
                silver_path: None,
                record_count: Some(result.record_count),
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
    request_body = JobTriggerRequest,
    responses(
        (status = 200, description = "Silver ref job submitted", body = JobTriggerResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "jobs"
)]
pub async fn submit_silver_ref_job(
    State(state): State<AppState>,
    Json(request): Json<JobTriggerRequest>,
) -> Result<Json<JobTriggerResponse>, (StatusCode, String)> {
    validate_upstream_count("silver_ref", &request.upstream_run_ids, 1)?;

    let upstream_bronze_run_id = parse_run_id(&request.upstream_run_ids[0], "upstream_run_ids[0]")?;
    let run_id = RunId::new();

    run_registry::create_run(
        &state.db_pool,
        &run_id,
        "silver_ref",
        request.orchestration.as_ref(),
        &request.upstream_run_ids,
    )
    .await
    .map_err(internal_error)?;

    run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Running, None)
        .await
        .map_err(internal_error)?;

    match crate::jobs::silver_ref_job::execute_with_run_id_and_upstream(
        run_id.clone(),
        upstream_bronze_run_id,
    ) {
        Ok(_result) => {
            run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Succeeded, None)
                .await
                .map_err(internal_error)?;

            Ok(Json(JobTriggerResponse {
                status: "submitted".to_string(),
                job_name: "silver_ref".to_string(),
                run_id: run_id.0.to_string(),
                upstream_run_ids: request.upstream_run_ids,
                source_name: None,
                raw_path: None,
                bronze_path: None,
                silver_path: None,
                record_count: None,
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
    request_body = JobTriggerRequest,
    responses(
        (status = 200, description = "Silver daily job submitted", body = JobTriggerResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "jobs"
)]
pub async fn submit_silver_daily_job(
    State(state): State<AppState>,
    Json(request): Json<JobTriggerRequest>,
) -> Result<Json<JobTriggerResponse>, (StatusCode, String)> {
    validate_upstream_count("silver_daily", &request.upstream_run_ids, 1)?;

    let upstream_bronze_run_id = parse_run_id(&request.upstream_run_ids[0], "upstream_run_ids[0]")?;
    let run_id = RunId::new();

    run_registry::create_run(
        &state.db_pool,
        &run_id,
        "silver_daily",
        request.orchestration.as_ref(),
        &request.upstream_run_ids,
    )
    .await
    .map_err(internal_error)?;

    run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Running, None)
        .await
        .map_err(internal_error)?;

    match crate::jobs::silver_daily_job::execute_with_run_id_and_upstream(
        run_id.clone(),
        upstream_bronze_run_id,
    ) {
        Ok(_result) => {
            run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Succeeded, None)
                .await
                .map_err(internal_error)?;

            Ok(Json(JobTriggerResponse {
                status: "submitted".to_string(),
                job_name: "silver_daily".to_string(),
                run_id: run_id.0.to_string(),
                upstream_run_ids: request.upstream_run_ids,
                source_name: None,
                raw_path: None,
                bronze_path: None,
                silver_path: None,
                record_count: None,
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
    request_body = JobTriggerRequest,
    responses(
        (status = 200, description = "Silver conformed job submitted", body = JobTriggerResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "jobs"
)]
pub async fn submit_silver_conformed_job(
    State(state): State<AppState>,
    Json(request): Json<JobTriggerRequest>,
) -> Result<Json<JobTriggerResponse>, (StatusCode, String)> {
    validate_upstream_count("silver_conformed", &request.upstream_run_ids, 2)?;

    let upstream_ref_run_id = parse_run_id(&request.upstream_run_ids[0], "upstream_run_ids[0]")?;
    let upstream_daily_run_id = parse_run_id(&request.upstream_run_ids[1], "upstream_run_ids[1]")?;
    let run_id = RunId::new();

    run_registry::create_run(
        &state.db_pool,
        &run_id,
        "silver_conformed",
        request.orchestration.as_ref(),
        &request.upstream_run_ids,
    )
    .await
    .map_err(internal_error)?;

    run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Running, None)
        .await
        .map_err(internal_error)?;

    match crate::jobs::silver_conformed_job::execute_with_run_id_and_upstreams(
        run_id.clone(),
        upstream_ref_run_id,
        upstream_daily_run_id,
    ) {
        Ok(_result) => {
            run_registry::update_run_status(&state.db_pool, &run_id, RunStatus::Succeeded, None)
                .await
                .map_err(internal_error)?;

            Ok(Json(JobTriggerResponse {
                status: "submitted".to_string(),
                job_name: "silver_conformed".to_string(),
                run_id: run_id.0.to_string(),
                upstream_run_ids: request.upstream_run_ids,
                source_name: None,
                raw_path: None,
                bronze_path: None,
                silver_path: None,
                record_count: None,
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
        orchestration: OrchestrationMetadata {
            orchestrator: run.orchestration.orchestrator,
            dag_id: run.orchestration.orchestrator_dag_id,
            dag_run_id: run.orchestration.orchestrator_dag_run_id,
            task_id: run.orchestration.orchestrator_task_id,
            try_number: run.orchestration.orchestrator_try_number,
        },
        upstream_run_ids: run.upstream_run_ids,
    }
}

fn parse_run_id(value: &str, field_name: &str) -> Result<RunId, (StatusCode, String)> {
    let uuid = Uuid::parse_str(value)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("invalid {field_name}: {e}")))?;

    Ok(RunId(uuid))
}

fn validate_upstream_count(
    job_name: &str,
    upstream_run_ids: &[String],
    expected: usize,
) -> Result<(), (StatusCode, String)> {
    if upstream_run_ids.len() != expected {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "{job_name} expects {expected} upstream_run_ids but received {}",
                upstream_run_ids.len()
            ),
        ));
    }

    Ok(())
}

fn internal_error<E: ToString>(error: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}