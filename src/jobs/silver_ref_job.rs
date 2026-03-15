// src/jobs/silver_ref_job.rs
use crate::core::errors::RsFoundryError;
use crate::core::types::RunId;
use crate::pipelines::silver::standardize_ref::{
    run_silver_ref_pipeline, SilverRefBuildResult,
};

pub fn execute(bronze_run_id: &str) -> Result<SilverRefBuildResult, RsFoundryError> {
    run_silver_ref_pipeline(bronze_run_id)
}

pub fn execute_with_run_id_and_upstream(
    _run_id: RunId,
    bronze_run_id: RunId,
) -> Result<SilverRefBuildResult, RsFoundryError> {
    run_silver_ref_pipeline(&bronze_run_id.0.to_string())
}