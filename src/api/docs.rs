// src/api/docs.rs
use utoipa::OpenApi;

use crate::api::models::{
    HealthResponse, JobTriggerRequest, JobTriggerResponse, OrchestrationMetadata, ReadyResponse,
    RunListResponse, RunResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::health,
        crate::api::handlers::ready,
        crate::api::handlers::submit_bronze_ref_job,
        crate::api::handlers::submit_bronze_daily_job,
        crate::api::handlers::submit_silver_ref_job,
        crate::api::handlers::submit_silver_daily_job,
        crate::api::handlers::submit_silver_conformed_job,
        crate::api::handlers::list_runs,
        crate::api::handlers::get_run_by_id
    ),
    components(
        schemas(
            HealthResponse,
            ReadyResponse,
            OrchestrationMetadata,
            JobTriggerRequest,
            JobTriggerResponse,
            RunResponse,
            RunListResponse
        )
    ),
    tags(
        (name = "runner", description = "Runner health and readiness endpoints"),
        (name = "jobs", description = "Job submission endpoints"),
        (name = "runs", description = "Run registry endpoints")
    )
)]
pub struct ApiDoc;