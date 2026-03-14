// src\quality\checks.rs
use crate::core::metadata::{BronzeDailyRecord, BronzeRefRecord};
use crate::quality::report::QualityReport;

pub fn check_non_empty(row_count: usize) -> QualityReport {
    let mut report = QualityReport::new(row_count);

    if row_count == 0 {
        report.push_error("dataset is empty");
    }

    report
}

pub fn validate_bronze_ref_records(records: &[BronzeRefRecord]) -> QualityReport {
    let mut report = check_non_empty(records.len());

    for (idx, record) in records.iter().enumerate() {
        if record.ref_id.trim().is_empty() {
            report.push_error(format!("row {idx}: ref_id is blank"));
        }
        if record.ref_name.trim().is_empty() {
            report.push_error(format!("row {idx}: ref_name is blank"));
        }
        if record.ref_type.trim().is_empty() {
            report.push_error(format!("row {idx}: ref_type is blank"));
        }
    }

    report
}

pub fn validate_bronze_daily_records(records: &[BronzeDailyRecord]) -> QualityReport {
    let mut report = check_non_empty(records.len());

    for (idx, record) in records.iter().enumerate() {
        if record.event_id.trim().is_empty() {
            report.push_error(format!("row {idx}: event_id is blank"));
        }
        if record.ref_id.trim().is_empty() {
            report.push_error(format!("row {idx}: ref_id is blank"));
        }
        if record.status.trim().is_empty() {
            report.push_error(format!("row {idx}: status is blank"));
        }
    }

    report
}
