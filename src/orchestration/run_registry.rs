// src/orchestration/run_registry.rs
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::api::models::OrchestrationMetadata;
use crate::core::errors::RsFoundryError;
use crate::core::types::{RunId, RunOrchestrationMetadata, RunRecord, RunStatus};

#[derive(Debug, Clone, FromRow)]
struct RunRow {
    run_id: Uuid,
    job_name: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    error_message: Option<String>,
    orchestrator: Option<String>,
    orchestrator_dag_id: Option<String>,
    orchestrator_dag_run_id: Option<String>,
    orchestrator_task_id: Option<String>,
    orchestrator_try_number: Option<i32>,
    upstream_run_ids: Value,
}

fn parse_status(status: &str) -> Result<RunStatus, RsFoundryError> {
    match status {
        "created" => Ok(RunStatus::Created),
        "running" => Ok(RunStatus::Running),
        "succeeded" => Ok(RunStatus::Succeeded),
        "failed" => Ok(RunStatus::Failed),
        other => Err(RsFoundryError::Validation(format!(
            "unknown run status '{other}'"
        ))),
    }
}

fn status_as_str(status: &RunStatus) -> &'static str {
    match status {
        RunStatus::Created => "created",
        RunStatus::Running => "running",
        RunStatus::Succeeded => "succeeded",
        RunStatus::Failed => "failed",
    }
}

fn parse_upstream_run_ids(value: Value) -> Result<Vec<String>, RsFoundryError> {
    serde_json::from_value(value).map_err(|e| {
        RsFoundryError::Validation(format!("failed to parse upstream_run_ids from database: {e}"))
    })
}

fn map_row(row: RunRow) -> Result<RunRecord, RsFoundryError> {
    Ok(RunRecord {
        run_id: RunId(row.run_id),
        job_name: row.job_name,
        status: parse_status(&row.status)?,
        created_at: row.created_at,
        updated_at: row.updated_at,
        error_message: row.error_message,
        orchestration: RunOrchestrationMetadata {
            orchestrator: row.orchestrator,
            orchestrator_dag_id: row.orchestrator_dag_id,
            orchestrator_dag_run_id: row.orchestrator_dag_run_id,
            orchestrator_task_id: row.orchestrator_task_id,
            orchestrator_try_number: row.orchestrator_try_number,
        },
        upstream_run_ids: parse_upstream_run_ids(row.upstream_run_ids)?,
    })
}

pub async fn create_run(
    pool: &PgPool,
    run_id: &RunId,
    job_name: &str,
    orchestration: Option<&OrchestrationMetadata>,
    upstream_run_ids: &[String],
) -> Result<RunRecord, RsFoundryError> {
    let upstream_run_ids_json = serde_json::to_value(upstream_run_ids).map_err(|e| {
        RsFoundryError::Validation(format!("failed to serialize upstream_run_ids: {e}"))
    })?;

    let row = sqlx::query_as::<_, RunRow>(
        r#"
        INSERT INTO runs (
            run_id,
            job_name,
            status,
            created_at,
            updated_at,
            error_message,
            orchestrator,
            orchestrator_dag_id,
            orchestrator_dag_run_id,
            orchestrator_task_id,
            orchestrator_try_number,
            upstream_run_ids
        )
        VALUES (
            $1, $2, $3, NOW(), NOW(), NULL,
            $4, $5, $6, $7, $8, $9
        )
        RETURNING
            run_id,
            job_name,
            status,
            created_at,
            updated_at,
            error_message,
            orchestrator,
            orchestrator_dag_id,
            orchestrator_dag_run_id,
            orchestrator_task_id,
            orchestrator_try_number,
            upstream_run_ids
        "#,
    )
    .bind(run_id.0)
    .bind(job_name)
    .bind(status_as_str(&RunStatus::Created))
    .bind(orchestration.and_then(|o| o.orchestrator.clone()))
    .bind(orchestration.and_then(|o| o.dag_id.clone()))
    .bind(orchestration.and_then(|o| o.dag_run_id.clone()))
    .bind(orchestration.and_then(|o| o.task_id.clone()))
    .bind(orchestration.and_then(|o| o.try_number))
    .bind(upstream_run_ids_json)
    .fetch_one(pool)
    .await
    .map_err(|e| RsFoundryError::Io(format!("failed to create run: {e}")))?;

    map_row(row)
}

pub async fn update_run_status(
    pool: &PgPool,
    run_id: &RunId,
    status: RunStatus,
    error_message: Option<&str>,
) -> Result<RunRecord, RsFoundryError> {
    let row = sqlx::query_as::<_, RunRow>(
        r#"
        UPDATE runs
        SET status = $2,
            updated_at = NOW(),
            error_message = $3
        WHERE run_id = $1
        RETURNING
            run_id,
            job_name,
            status,
            created_at,
            updated_at,
            error_message,
            orchestrator,
            orchestrator_dag_id,
            orchestrator_dag_run_id,
            orchestrator_task_id,
            orchestrator_try_number,
            upstream_run_ids
        "#,
    )
    .bind(run_id.0)
    .bind(status_as_str(&status))
    .bind(error_message)
    .fetch_one(pool)
    .await
    .map_err(|e| RsFoundryError::Io(format!("failed to update run status: {e}")))?;

    map_row(row)
}

pub async fn get_run(pool: &PgPool, run_id: &RunId) -> Result<RunRecord, RsFoundryError> {
    let row = sqlx::query_as::<_, RunRow>(
        r#"
        SELECT
            run_id,
            job_name,
            status,
            created_at,
            updated_at,
            error_message,
            orchestrator,
            orchestrator_dag_id,
            orchestrator_dag_run_id,
            orchestrator_task_id,
            orchestrator_try_number,
            upstream_run_ids
        FROM runs
        WHERE run_id = $1
        "#,
    )
    .bind(run_id.0)
    .fetch_one(pool)
    .await
    .map_err(|e| RsFoundryError::Io(format!("failed to fetch run: {e}")))?;

    map_row(row)
}

pub async fn list_runs(pool: &PgPool, limit: i64) -> Result<Vec<RunRecord>, RsFoundryError> {
    let rows = sqlx::query_as::<_, RunRow>(
        r#"
        SELECT
            run_id,
            job_name,
            status,
            created_at,
            updated_at,
            error_message,
            orchestrator,
            orchestrator_dag_id,
            orchestrator_dag_run_id,
            orchestrator_task_id,
            orchestrator_try_number,
            upstream_run_ids
        FROM runs
        ORDER BY created_at DESC
        LIMIT $1
        "#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|e| RsFoundryError::Io(format!("failed to list runs: {e}")))?;

    rows.into_iter().map(map_row).collect()
}