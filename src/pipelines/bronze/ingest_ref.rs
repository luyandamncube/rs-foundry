// src\pipelines\bronze\ingest_ref.rs
use std::path::{Path, PathBuf};

use chrono::Utc;
use tracing::{error, info};

use crate::core::errors::RsFoundryError;
use crate::core::metadata::BronzeRefRecord;
use crate::core::types::{RefSourceRecord, RunId};
use crate::io::files::{ensure_dir, write_text};
use crate::jobs::raw_landing::land_raw_text;
use crate::quality::checks::validate_bronze_ref_records;
use crate::quality::report::QualityReport;

#[derive(Debug, Clone)]
pub struct BronzeRefIngestResult {
    pub run_id: String,
    pub source_name: String,
    pub raw_path: PathBuf,
    pub bronze_path: PathBuf,
    pub record_count: usize,
    pub quality_report: QualityReport,
}

pub fn run_bronze_ref_pipeline(run_id: RunId) -> Result<BronzeRefIngestResult, RsFoundryError> {
    let data_root = Path::new("./data");
    let source_name = "ref_example";

    info!(
        run_id = %run_id.0,
        source_name = source_name,
        "starting bronze ref pipeline"
    );

    let source_records = sample_ref_source_records();

    info!(
        run_id = %run_id.0,
        source_name = source_name,
        source_record_count = source_records.len(),
        "generated bronze ref source records"
    );

    let raw_payload = serde_json::to_string_pretty(&source_records)
        .map_err(|e| RsFoundryError::Serialization(format!("failed to serialize raw payload: {e}")))?;

    let raw_result = land_raw_text(
        data_root,
        source_name,
        &run_id,
        Some("ref_source.json"),
        &raw_payload,
    )?;

    info!(
        run_id = %run_id.0,
        source_name = source_name,
        raw_path = %raw_result.file_path.display(),
        raw_bytes_written = raw_result.bytes_written,
        "landed raw bronze ref payload"
    );

    let bronze_records = build_bronze_ref_records(
        &source_records,
        &run_id,
        source_name,
        Some("ref_source.json".to_string()),
    );

    let quality_report = validate_bronze_ref_records(&bronze_records);

    if !quality_report.passed {
        error!(
            run_id = %run_id.0,
            source_name = source_name,
            error_count = quality_report.errors.len(),
            errors = ?quality_report.errors,
            "bronze ref quality checks failed"
        );

        return Err(RsFoundryError::Validation(format!(
            "bronze ref quality checks failed: {}",
            quality_report.errors.join("; ")
        )));
    }

    info!(
        run_id = %run_id.0,
        source_name = source_name,
        record_count = bronze_records.len(),
        warning_count = quality_report.warnings.len(),
        "bronze ref quality checks passed"
    );

    let bronze_path = bronze_output_path(data_root, source_name, &run_id);
    write_bronze_ref_output(&bronze_path, &bronze_records)?;

    info!(
        run_id = %run_id.0,
        source_name = source_name,
        bronze_path = %bronze_path.display(),
        record_count = bronze_records.len(),
        "wrote bronze ref output"
    );

    info!(
        run_id = %run_id.0,
        source_name = source_name,
        "completed bronze ref pipeline"
    );

    Ok(BronzeRefIngestResult {
        run_id: run_id.0.to_string(),
        source_name: source_name.to_string(),
        raw_path: raw_result.file_path,
        bronze_path,
        record_count: bronze_records.len(),
        quality_report,
    })
}

pub fn sample_ref_source_records() -> Vec<RefSourceRecord> {
    vec![
        RefSourceRecord {
            ref_id: "R001".to_string(),
            ref_name: "Alpha".to_string(),
            ref_type: "site".to_string(),
            is_active: true,
            updated_at: None,
        },
        RefSourceRecord {
            ref_id: "R002".to_string(),
            ref_name: "Beta".to_string(),
            ref_type: "site".to_string(),
            is_active: true,
            updated_at: None,
        },
        RefSourceRecord {
            ref_id: "R003".to_string(),
            ref_name: "Gamma".to_string(),
            ref_type: "region".to_string(),
            is_active: false,
            updated_at: None,
        },
    ]
}

pub fn build_bronze_ref_records(
    source_records: &[RefSourceRecord],
    run_id: &RunId,
    source_name: &str,
    source_file: Option<String>,
) -> Vec<BronzeRefRecord> {
    let ingested_at = Utc::now();
    let load_date = ingested_at.format("%Y-%m-%d").to_string();

    source_records
        .iter()
        .map(|record| BronzeRefRecord {
            ref_id: record.ref_id.clone(),
            ref_name: record.ref_name.clone(),
            ref_type: record.ref_type.clone(),
            is_active: record.is_active,
            updated_at: record.updated_at,

            run_id: run_id.0.to_string(),
            source_name: source_name.to_string(),
            source_file: source_file.clone(),
            ingested_at,
            load_date: load_date.clone(),
            record_hash: None,
        })
        .collect()
}

pub fn bronze_output_path(data_root: &Path, source_name: &str, run_id: &RunId) -> PathBuf {
    let load_date = Utc::now().format("%Y-%m-%d").to_string();

    data_root
        .join("bronze")
        .join(source_name)
        .join(format!("load_date={load_date}"))
        .join(format!("run_id={}", run_id.0))
        .join("bronze_ref.json")
}

pub fn write_bronze_ref_output(
    path: &Path,
    records: &[BronzeRefRecord],
) -> Result<(), RsFoundryError> {
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }

    let payload = serde_json::to_string_pretty(records)
        .map_err(|e| RsFoundryError::Serialization(format!("failed to serialize bronze output: {e}")))?;

    write_text(path, &payload)
}


