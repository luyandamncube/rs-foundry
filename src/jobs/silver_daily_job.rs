// src\jobs\silver_daily_job.rs
use crate::core::errors::RsFoundryError;
use crate::pipelines::silver::standardize_daily::{
    run_silver_daily_pipeline, SilverDailyBuildResult,
};

pub fn execute(bronze_run_id: &str) -> Result<SilverDailyBuildResult, RsFoundryError> {
    run_silver_daily_pipeline(bronze_run_id)
}