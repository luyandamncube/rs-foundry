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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BronzeJobRequest {
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BronzeJobResponse {
    pub status: String,
    pub job_name: String,
    pub run_id: String,
    pub source_name: String,
    pub raw_path: String,
    pub bronze_path: String,
    pub record_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SilverRefJobRequest {
    pub bronze_run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SilverRefJobResponse {
    pub status: String,
    pub job_name: String,
    pub run_id: String,
    pub bronze_run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SilverDailyJobRequest {
    pub bronze_run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SilverDailyJobResponse {
    pub status: String,
    pub job_name: String,
    pub run_id: String,
    pub bronze_run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SilverConformedJobRequest {
    pub ref_run_id: String,
    pub daily_run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SilverConformedJobResponse {
    pub status: String,
    pub job_name: String,
    pub run_id: String,
    pub ref_run_id: String,
    pub daily_run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RunResponse {
    pub run_id: String,
    pub job_name: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RunListResponse {
    pub runs: Vec<RunResponse>,
}