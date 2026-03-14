// src\jobs\silver_ref_job.rs
use crate::core::errors::RsFoundryError;
use crate::pipelines::silver::standardize_ref::{run_silver_ref_pipeline, SilverRefBuildResult};

pub fn execute(bronze_run_id: &str) -> Result<SilverRefBuildResult, RsFoundryError> {
    run_silver_ref_pipeline(bronze_run_id)
}