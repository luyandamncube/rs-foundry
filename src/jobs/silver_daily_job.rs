// src\jobs\silver_daily_job.rs
// src/jobs/silver_daily_job.rs
use crate::core::errors::RsFoundryError;
use crate::core::types::RunId;
use crate::pipelines::silver::standardize_daily::{
    run_silver_daily_pipeline, SilverDailyBuildResult,
};

pub fn execute(bronze_run_id: &str) -> Result<SilverDailyBuildResult, RsFoundryError> {
    run_silver_daily_pipeline(bronze_run_id)
}

pub fn execute_with_run_id_and_upstream(
    _run_id: RunId,
    bronze_run_id: RunId,
) -> Result<SilverDailyBuildResult, RsFoundryError> {
    run_silver_daily_pipeline(&bronze_run_id.0.to_string())
}