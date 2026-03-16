// src/api/models.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReadyResponse {
    pub status: String,
    pub service: String,
    pub checks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct OrchestrationMetadata {
    pub orchestrator: Option<String>,
    pub dag_id: Option<String>,
    pub dag_run_id: Option<String>,
    pub task_id: Option<String>,
    pub try_number: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct JobTriggerRequest {
    pub requested_by: Option<String>,
    #[serde(default)]
    pub upstream_run_ids: Vec<String>,
    pub orchestration: Option<OrchestrationMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JobTriggerResponse {
    pub status: String,
    pub job_name: String,
    pub run_id: String,
    pub upstream_run_ids: Vec<String>,
    pub source_name: Option<String>,
    pub raw_path: Option<String>,
    pub bronze_path: Option<String>,
    pub silver_path: Option<String>,
    pub record_count: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RunResponse {
    pub run_id: String,
    pub job_name: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub error_message: Option<String>,
    pub orchestration: OrchestrationMetadata,
    pub upstream_run_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RunListResponse {
    pub runs: Vec<RunResponse>,
}