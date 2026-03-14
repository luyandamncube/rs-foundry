// src\jobs\bronze_ref_job.rs
use crate::core::errors::RsFoundryError;
use crate::pipelines::bronze::ingest_ref::{run_bronze_ref_pipeline, BronzeRefIngestResult};

pub fn execute() -> Result<BronzeRefIngestResult, RsFoundryError> {
    run_bronze_ref_pipeline()
}
