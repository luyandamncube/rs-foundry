// src/jobs/silver_conformed_job.rs
use crate::core::errors::RsFoundryError;
use crate::core::types::RunId;
use crate::pipelines::silver::build_conformed::{
    run_silver_conformed_pipeline, SilverConformedBuildResult,
};

pub fn execute(
    ref_bronze_run_id: &str,
    daily_bronze_run_id: &str,
) -> Result<SilverConformedBuildResult, RsFoundryError> {
    run_silver_conformed_pipeline(ref_bronze_run_id, daily_bronze_run_id)
}

pub fn execute_with_run_id_and_upstreams(
    _run_id: RunId,
    ref_run_id: RunId,
    daily_run_id: RunId,
) -> Result<SilverConformedBuildResult, RsFoundryError> {
    run_silver_conformed_pipeline(
        &ref_run_id.0.to_string(),
        &daily_run_id.0.to_string(),
    )
}