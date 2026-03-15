// src\pipelines\bronze\ingest_daily.rs
use std::path::{Path, PathBuf};

use chrono::{NaiveDate, Utc};
use tracing::{error, info};

use crate::core::errors::RsFoundryError;
use crate::core::metadata::BronzeDailyRecord;
use crate::core::types::{DailySourceRecord, RunId};
use crate::io::files::{ensure_dir, write_text};
use crate::jobs::raw_landing::land_raw_text;
use crate::quality::checks::validate_bronze_daily_records;
use crate::quality::report::QualityReport;

#[derive(Debug, Clone)]
pub struct BronzeDailyIngestResult {
    pub run_id: String,
    pub source_name: String,
    pub raw_path: PathBuf,
    pub bronze_path: PathBuf,
    pub record_count: usize,
    pub quality_report: QualityReport,
}

pub fn run_bronze_daily_pipeline(run_id: RunId) -> Result<BronzeDailyIngestResult, RsFoundryError> {
    let data_root = Path::new("./data");
    let source_name = "daily_example";

    info!(
        run_id = %run_id.0,
        source_name = source_name,
        "starting bronze daily pipeline"
    );

    let source_records = sample_daily_source_records();

    info!(
        run_id = %run_id.0,
        source_name = source_name,
        source_record_count = source_records.len(),
        "generated bronze daily source records"
    );

    let raw_payload = serde_json::to_string_pretty(&source_records).map_err(|e| {
        RsFoundryError::Serialization(format!("failed to serialize daily raw payload: {e}"))
    })?;

    let raw_result = land_raw_text(
        data_root,
        source_name,
        &run_id,
        Some("daily_source.json"),
        &raw_payload,
    )?;

    info!(
        run_id = %run_id.0,
        source_name = source_name,
        raw_path = %raw_result.file_path.display(),
        raw_bytes_written = raw_result.bytes_written,
        "landed raw bronze daily payload"
    );

    let bronze_records = build_bronze_daily_records(
        &source_records,
        &run_id,
        source_name,
        Some("daily_source.json".to_string()),
    );

    let quality_report = validate_bronze_daily_records(&bronze_records);
    if !quality_report.passed {
        error!(
            run_id = %run_id.0,
            source_name = source_name,
            error_count = quality_report.errors.len(),
            errors = ?quality_report.errors,
            "bronze daily quality checks failed"
        );

        return Err(RsFoundryError::Validation(format!(
            "bronze daily quality checks failed: {}",
            quality_report.errors.join("; ")
        )));
    }

    info!(
        run_id = %run_id.0,
        source_name = source_name,
        record_count = bronze_records.len(),
        warning_count = quality_report.warnings.len(),
        "bronze daily quality checks passed"
    );

    let bronze_path = bronze_output_path(data_root, source_name, &run_id);
    write_bronze_daily_output(&bronze_path, &bronze_records)?;

    info!(
        run_id = %run_id.0,
        source_name = source_name,
        bronze_path = %bronze_path.display(),
        record_count = bronze_records.len(),
        "wrote bronze daily output"
    );

    info!(
        run_id = %run_id.0,
        source_name = source_name,
        "completed bronze daily pipeline"
    );

    Ok(BronzeDailyIngestResult {
        run_id: run_id.0.to_string(),
        source_name: source_name.to_string(),
        raw_path: raw_result.file_path,
        bronze_path,
        record_count: bronze_records.len(),
        quality_report,
    })
}

pub fn sample_daily_source_records() -> Vec<DailySourceRecord> {
    vec![
        DailySourceRecord {
            event_id: "E001".to_string(),
            ref_id: "R001".to_string(),
            event_date: NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(),
            metric_value: 10.5,
            status: "ok".to_string(),
            source_ts: None,
        },
        DailySourceRecord {
            event_id: "E002".to_string(),
            ref_id: "R002".to_string(),
            event_date: NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(),
            metric_value: 25.0,
            status: "warning".to_string(),
            source_ts: None,
        },
        DailySourceRecord {
            event_id: "E003".to_string(),
            ref_id: "R003".to_string(),
            event_date: NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(),
            metric_value: 0.0,
            status: "inactive".to_string(),
            source_ts: None,
        },
    ]
}

pub fn build_bronze_daily_records(
    source_records: &[DailySourceRecord],
    run_id: &RunId,
    source_name: &str,
    source_file: Option<String>,
) -> Vec<BronzeDailyRecord> {
    let ingested_at = Utc::now();
    let load_date = ingested_at.format("%Y-%m-%d").to_string();

    source_records
        .iter()
        .map(|record| BronzeDailyRecord {
            event_id: record.event_id.clone(),
            ref_id: record.ref_id.clone(),
            event_date: record.event_date,
            metric_value: record.metric_value,
            status: record.status.clone(),
            source_ts: record.source_ts,

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
        .join("bronze_daily.json")
}

pub fn write_bronze_daily_output(
    path: &Path,
    records: &[BronzeDailyRecord],
) -> Result<(), RsFoundryError> {
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }

    let payload = serde_json::to_string_pretty(records).map_err(|e| {
        RsFoundryError::Serialization(format!("failed to serialize bronze daily output: {e}"))
    })?;

    write_text(path, &payload)
}
