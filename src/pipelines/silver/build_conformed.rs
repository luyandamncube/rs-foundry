// src\pipelines\silver\build_conformed.rs
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use chrono::Utc;
use tracing::{error, info, warn};

use crate::core::errors::RsFoundryError;
use crate::core::metadata::{SilverConformedRecord, SilverDailyRecord, SilverRefRecord};
use crate::io::files::{ensure_dir, write_text};
use crate::pipelines::silver::standardize_daily::read_bronze_daily_records;
use crate::pipelines::silver::standardize_ref::read_bronze_ref_records;
use crate::quality::contracts::validate_silver_conformed_records;
use crate::quality::report::QualityReport;

#[derive(Debug, Clone)]
pub struct SilverConformedBuildResult {
    pub ref_bronze_run_id: String,
    pub daily_bronze_run_id: String,
    pub conformed_path: PathBuf,
    pub record_count: usize,
    pub quality_report: QualityReport,
}

pub fn run_silver_conformed_pipeline(
    ref_bronze_run_id: &str,
    daily_bronze_run_id: &str,
) -> Result<SilverConformedBuildResult, RsFoundryError> {
    let data_root = Path::new("./data");

    info!(
        ref_bronze_run_id = ref_bronze_run_id,
        daily_bronze_run_id = daily_bronze_run_id,
        "starting silver conformed pipeline"
    );

    let ref_bronze_path = data_root
        .join("bronze")
        .join("ref_example")
        .join(format!("load_date={}", Utc::now().format("%Y-%m-%d")))
        .join(format!("run_id={ref_bronze_run_id}"))
        .join("bronze_ref.json");

    let daily_bronze_path = data_root
        .join("bronze")
        .join("daily_example")
        .join(format!("load_date={}", Utc::now().format("%Y-%m-%d")))
        .join(format!("run_id={daily_bronze_run_id}"))
        .join("bronze_daily.json");

    let ref_bronze = read_bronze_ref_records(&ref_bronze_path)?;
    let daily_bronze = read_bronze_daily_records(&daily_bronze_path)?;

    info!(
        ref_bronze_run_id = ref_bronze_run_id,
        daily_bronze_run_id = daily_bronze_run_id,
        ref_bronze_count = ref_bronze.len(),
        daily_bronze_count = daily_bronze.len(),
        "read bronze inputs for conformed build"
    );

    let silver_ref = crate::pipelines::silver::standardize_ref::dedupe_silver_ref_records(
        &crate::pipelines::silver::standardize_ref::build_silver_ref_records(&ref_bronze),
    );

    let silver_daily = crate::pipelines::silver::standardize_daily::dedupe_silver_daily_records(
        &crate::pipelines::silver::standardize_daily::build_silver_daily_records(&daily_bronze),
    );

    let conformed = build_conformed_records(&silver_ref, &silver_daily, ref_bronze_run_id, daily_bronze_run_id);

    info!(
        ref_bronze_run_id = ref_bronze_run_id,
        daily_bronze_run_id = daily_bronze_run_id,
        silver_ref_count = silver_ref.len(),
        silver_daily_count = silver_daily.len(),
        conformed_count = conformed.len(),
        "built conformed silver records"
    );

    if conformed.len() < silver_daily.len() {
        warn!(
            ref_bronze_run_id = ref_bronze_run_id,
            daily_bronze_run_id = daily_bronze_run_id,
            missing_join_count = silver_daily.len() - conformed.len(),
            "some silver daily records did not match a silver ref record"
        );
    }

    let quality_report = validate_silver_conformed_records(&conformed);
    if !quality_report.passed {
        error!(
            ref_bronze_run_id = ref_bronze_run_id,
            daily_bronze_run_id = daily_bronze_run_id,
            error_count = quality_report.errors.len(),
            errors = ?quality_report.errors,
            "silver conformed contract checks failed"
        );

        return Err(RsFoundryError::Validation(format!(
            "silver conformed contract checks failed: {}",
            quality_report.errors.join("; ")
        )));
    }

    let conformed_path = conformed_output_path(data_root, ref_bronze_run_id, daily_bronze_run_id);
    write_conformed_output(&conformed_path, &conformed)?;

    info!(
        ref_bronze_run_id = ref_bronze_run_id,
        daily_bronze_run_id = daily_bronze_run_id,
        conformed_path = %conformed_path.display(),
        record_count = conformed.len(),
        "wrote conformed silver output"
    );

    Ok(SilverConformedBuildResult {
        ref_bronze_run_id: ref_bronze_run_id.to_string(),
        daily_bronze_run_id: daily_bronze_run_id.to_string(),
        conformed_path,
        record_count: conformed.len(),
        quality_report,
    })
}

pub fn build_conformed_records(
    ref_records: &[SilverRefRecord],
    daily_records: &[SilverDailyRecord],
    ref_bronze_run_id: &str,
    daily_bronze_run_id: &str,
) -> Vec<SilverConformedRecord> {
    let ref_by_id: HashMap<String, &SilverRefRecord> =
        ref_records.iter().map(|r| (r.ref_id.clone(), r)).collect();

    let silver_built_at = Utc::now();

    let mut conformed = Vec::new();

    for daily in daily_records {
        if let Some(reference) = ref_by_id.get(&daily.ref_id) {
            conformed.push(SilverConformedRecord {
                event_id: daily.event_id.clone(),
                ref_id: daily.ref_id.clone(),
                ref_name: reference.ref_name.clone(),
                ref_type: reference.ref_type.clone(),
                ref_name_normalized: reference.ref_name_normalized.clone(),
                ref_type_normalized: reference.ref_type_normalized.clone(),
                ref_is_active: reference.is_active,

                event_date: daily.event_date,
                metric_value: daily.metric_value,
                status: daily.status.clone(),
                status_normalized: daily.status_normalized.clone(),
                is_positive_metric: daily.is_positive_metric,
                source_ts: daily.source_ts,

                ref_bronze_run_id: ref_bronze_run_id.to_string(),
                daily_bronze_run_id: daily_bronze_run_id.to_string(),
                silver_built_at,
                load_date: daily.load_date.clone(),
            });
        }
    }

    conformed.sort_by(|a, b| a.event_id.cmp(&b.event_id));
    conformed
}

pub fn conformed_output_path(
    data_root: &Path,
    ref_bronze_run_id: &str,
    daily_bronze_run_id: &str,
) -> PathBuf {
    let load_date = Utc::now().format("%Y-%m-%d").to_string();

    data_root
        .join("silver")
        .join("conformed")
        .join(format!("load_date={load_date}"))
        .join(format!("ref_bronze_run_id={ref_bronze_run_id}"))
        .join(format!("daily_bronze_run_id={daily_bronze_run_id}"))
        .join("silver_conformed.json")
}

pub fn write_conformed_output(
    path: &Path,
    records: &[SilverConformedRecord],
) -> Result<(), RsFoundryError> {
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }

    let payload = serde_json::to_string_pretty(records).map_err(|e| {
        RsFoundryError::Serialization(format!("failed to serialize silver conformed output: {e}"))
    })?;

    write_text(path, &payload)
}
