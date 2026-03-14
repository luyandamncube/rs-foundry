// src\jobs\silver_conformed_job.rs
use crate::core::errors::RsFoundryError;
use crate::pipelines::silver::build_conformed::{
    run_silver_conformed_pipeline, SilverConformedBuildResult,
};

pub fn execute(
    ref_bronze_run_id: &str,
    daily_bronze_run_id: &str,
) -> Result<SilverConformedBuildResult, RsFoundryError> {
    run_silver_conformed_pipeline(ref_bronze_run_id, daily_bronze_run_id)
}