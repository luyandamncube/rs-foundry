// src\jobs\bronze_daily_job.rs
use crate::core::errors::RsFoundryError;
use crate::core::types::RunId;
use crate::pipelines::bronze::ingest_daily::{run_bronze_daily_pipeline, BronzeDailyIngestResult};

pub fn execute_with_run_id(run_id: RunId) -> Result<BronzeDailyIngestResult, RsFoundryError> {
    run_bronze_daily_pipeline(run_id)
}

pub fn execute() -> Result<BronzeDailyIngestResult, RsFoundryError> {
    execute_with_run_id(RunId::new())
}
