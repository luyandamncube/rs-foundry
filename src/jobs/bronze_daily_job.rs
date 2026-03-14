// src\jobs\bronze_daily_job.rs
use crate::core::errors::RsFoundryError;
use crate::pipelines::bronze::ingest_daily::{run_bronze_daily_pipeline, BronzeDailyIngestResult};

pub fn execute() -> Result<BronzeDailyIngestResult, RsFoundryError> {
    run_bronze_daily_pipeline()
}
