// src\orchestration\run_registry.rs
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::core::errors::RsFoundryError;
use crate::core::types::{RunId, RunRecord, RunStatus};

#[derive(Debug, Clone, FromRow)]
struct RunRow {
    run_id: Uuid,
    job_name: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    error_message: Option<String>,
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

fn map_row(row: RunRow) -> Result<RunRecord, RsFoundryError> {
    Ok(RunRecord {
        run_id: RunId(row.run_id),
        job_name: row.job_name,
        status: parse_status(&row.status)?,
        created_at: row.created_at,
        updated_at: row.updated_at,
        error_message: row.error_message,
    })
}

pub async fn create_run(
    pool: &PgPool,
    run_id: &RunId,
    job_name: &str,
) -> Result<RunRecord, RsFoundryError> {
    let row = sqlx::query_as::<_, RunRow>(
        r#"
        INSERT INTO runs (run_id, job_name, status, created_at, updated_at, error_message)
        VALUES ($1, $2, $3, NOW(), NOW(), NULL)
        RETURNING run_id, job_name, status, created_at, updated_at, error_message
        "#,
    )
    .bind(run_id.0)
    .bind(job_name)
    .bind(status_as_str(&RunStatus::Created))
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
        RETURNING run_id, job_name, status, created_at, updated_at, error_message
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
        SELECT run_id, job_name, status, created_at, updated_at, error_message
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
        SELECT run_id, job_name, status, created_at, updated_at, error_message
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
