// src\jobs\bronze_ref_job.rs
use crate::core::errors::RsFoundryError;
use crate::core::types::RunId;
use crate::pipelines::bronze::ingest_ref::{run_bronze_ref_pipeline, BronzeRefIngestResult};

pub fn execute_with_run_id(run_id: RunId) -> Result<BronzeRefIngestResult, RsFoundryError> {
    run_bronze_ref_pipeline(run_id)
}

pub fn execute() -> Result<BronzeRefIngestResult, RsFoundryError> {
    execute_with_run_id(RunId::new())
}
