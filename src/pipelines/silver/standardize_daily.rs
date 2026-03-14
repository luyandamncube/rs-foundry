// src\pipelines\silver\standardize_daily.rs
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;

use crate::core::errors::RsFoundryError;
use crate::core::metadata::{BronzeDailyRecord, SilverDailyRecord};
use crate::io::files::{ensure_dir, write_text};
use crate::quality::contracts::validate_silver_daily_records;
use crate::quality::report::QualityReport;

#[derive(Debug, Clone)]
pub struct SilverDailyBuildResult {
    pub bronze_run_id: String,
    pub source_name: String,
    pub bronze_path: PathBuf,
    pub silver_path: PathBuf,
    pub record_count: usize,
    pub quality_report: QualityReport,
}

pub fn run_silver_daily_pipeline(
    bronze_run_id: &str,
) -> Result<SilverDailyBuildResult, RsFoundryError> {
    let data_root = Path::new("./data");
    let source_name = "daily_example";

    let bronze_path = bronze_input_path(data_root, source_name, bronze_run_id);
    let bronze_records = read_bronze_daily_records(&bronze_path)?;

    let standardized_records = build_silver_daily_records(&bronze_records);
    let silver_records = dedupe_silver_daily_records(&standardized_records);

    let quality_report = validate_silver_daily_records(&silver_records);
    if !quality_report.passed {
        return Err(RsFoundryError::Validation(format!(
            "silver daily contract checks failed: {}",
            quality_report.errors.join("; ")
        )));
    }

    let silver_path = silver_output_path(data_root, source_name, bronze_run_id);
    write_silver_daily_output(&silver_path, &silver_records)?;

    Ok(SilverDailyBuildResult {
        bronze_run_id: bronze_run_id.to_string(),
        source_name: source_name.to_string(),
        bronze_path,
        silver_path,
        record_count: silver_records.len(),
        quality_report,
    })
}

pub fn bronze_input_path(data_root: &Path, source_name: &str, bronze_run_id: &str) -> PathBuf {
    let load_date = Utc::now().format("%Y-%m-%d").to_string();

    data_root
        .join("bronze")
        .join(source_name)
        .join(format!("load_date={load_date}"))
        .join(format!("run_id={bronze_run_id}"))
        .join("bronze_daily.json")
}

pub fn silver_output_path(data_root: &Path, source_name: &str, bronze_run_id: &str) -> PathBuf {
    let load_date = Utc::now().format("%Y-%m-%d").to_string();

    data_root
        .join("silver")
        .join(source_name)
        .join(format!("load_date={load_date}"))
        .join(format!("bronze_run_id={bronze_run_id}"))
        .join("silver_daily.json")
}

pub fn read_bronze_daily_records(path: &Path) -> Result<Vec<BronzeDailyRecord>, RsFoundryError> {
    let content = fs::read_to_string(path).map_err(|e| {
        RsFoundryError::Io(format!(
            "failed to read bronze daily input {:?}: {e}",
            path
        ))
    })?;

    serde_json::from_str::<Vec<BronzeDailyRecord>>(&content).map_err(|e| {
        RsFoundryError::Serialization(format!(
            "failed to deserialize bronze daily records from {:?}: {e}",
            path
        ))
    })
}

pub fn build_silver_daily_records(bronze_records: &[BronzeDailyRecord]) -> Vec<SilverDailyRecord> {
    let silver_built_at = Utc::now();

    bronze_records
        .iter()
        .map(|record| SilverDailyRecord {
            event_id: record.event_id.trim().to_string(),
            ref_id: record.ref_id.trim().to_string(),
            event_date: record.event_date,
            metric_value: record.metric_value,
            status: record.status.trim().to_string(),
            status_normalized: record.status.trim().to_lowercase(),
            is_positive_metric: record.metric_value > 0.0,
            source_ts: record.source_ts,

            bronze_run_id: record.run_id.clone(),
            source_name: record.source_name.clone(),
            source_file: record.source_file.clone(),
            bronze_ingested_at: record.ingested_at,
            silver_built_at,
            load_date: record.load_date.clone(),
        })
        .collect()
}

pub fn dedupe_silver_daily_records(records: &[SilverDailyRecord]) -> Vec<SilverDailyRecord> {
    let mut by_key: HashMap<String, SilverDailyRecord> = HashMap::new();

    for record in records {
        match by_key.get(&record.event_id) {
            None => {
                by_key.insert(record.event_id.clone(), record.clone());
            }
            Some(existing) => {
                if should_replace_daily(existing, record) {
                    by_key.insert(record.event_id.clone(), record.clone());
                }
            }
        }
    }

    let mut deduped: Vec<SilverDailyRecord> = by_key.into_values().collect();
    deduped.sort_by(|a, b| a.event_id.cmp(&b.event_id));
    deduped
}

fn should_replace_daily(existing: &SilverDailyRecord, candidate: &SilverDailyRecord) -> bool {
    match (existing.source_ts, candidate.source_ts) {
        (Some(existing_ts), Some(candidate_ts)) => {
            if candidate_ts != existing_ts {
                return candidate_ts > existing_ts;
            }
        }
        (None, Some(_)) => return true,
        (Some(_), None) => return false,
        (None, None) => {}
    }

    candidate.bronze_ingested_at > existing.bronze_ingested_at
}

pub fn write_silver_daily_output(
    path: &Path,
    records: &[SilverDailyRecord],
) -> Result<(), RsFoundryError> {
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }

    let payload = serde_json::to_string_pretty(records).map_err(|e| {
        RsFoundryError::Serialization(format!("failed to serialize silver daily output: {e}"))
    })?;

    write_text(path, &payload)
}