// src\quality\contracts.rs
use std::collections::HashSet;

use crate::core::metadata::{
    SilverConformedRecord, SilverDailyRecord, SilverRefRecord,
};
use crate::quality::report::QualityReport;

pub fn validate_silver_ref_records(records: &[SilverRefRecord]) -> QualityReport {
    let mut report = QualityReport::new(records.len());

    if records.is_empty() {
        report.push_error("silver ref dataset is empty");
        return report;
    }

    let mut seen_ref_ids: HashSet<&str> = HashSet::new();

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
        if record.ref_name_normalized.trim().is_empty() {
            report.push_error(format!("row {idx}: ref_name_normalized is blank"));
        }
        if record.ref_type_normalized.trim().is_empty() {
            report.push_error(format!("row {idx}: ref_type_normalized is blank"));
        }

        if !record.ref_id.trim().is_empty() && !seen_ref_ids.insert(record.ref_id.as_str()) {
            report.push_error(format!("row {idx}: duplicate ref_id '{}'", record.ref_id));
        }
    }

    report
}

pub fn validate_silver_daily_records(records: &[SilverDailyRecord]) -> QualityReport {
    let mut report = QualityReport::new(records.len());

    if records.is_empty() {
        report.push_error("silver daily dataset is empty");
        return report;
    }

    let mut seen_event_ids: HashSet<&str> = HashSet::new();

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
        if record.status_normalized.trim().is_empty() {
            report.push_error(format!("row {idx}: status_normalized is blank"));
        }
        if !record.metric_value.is_finite() {
            report.push_error(format!("row {idx}: metric_value is not finite"));
        }

        if !record.event_id.trim().is_empty() && !seen_event_ids.insert(record.event_id.as_str()) {
            report.push_error(format!(
                "row {idx}: duplicate event_id '{}'",
                record.event_id
            ));
        }
    }

    report
}

pub fn validate_silver_conformed_records(records: &[SilverConformedRecord]) -> QualityReport {
    let mut report = QualityReport::new(records.len());

    if records.is_empty() {
        report.push_error("silver conformed dataset is empty");
        return report;
    }

    let mut seen_event_ids: HashSet<&str> = HashSet::new();

    for (idx, record) in records.iter().enumerate() {
        if record.event_id.trim().is_empty() {
            report.push_error(format!("row {idx}: event_id is blank"));
        }
        if record.ref_id.trim().is_empty() {
            report.push_error(format!("row {idx}: ref_id is blank"));
        }
        if record.ref_name.trim().is_empty() {
            report.push_error(format!("row {idx}: ref_name is blank"));
        }
        if record.ref_type.trim().is_empty() {
            report.push_error(format!("row {idx}: ref_type is blank"));
        }
        if record.ref_name_normalized.trim().is_empty() {
            report.push_error(format!("row {idx}: ref_name_normalized is blank"));
        }
        if record.ref_type_normalized.trim().is_empty() {
            report.push_error(format!("row {idx}: ref_type_normalized is blank"));
        }
        if record.status.trim().is_empty() {
            report.push_error(format!("row {idx}: status is blank"));
        }
        if record.status_normalized.trim().is_empty() {
            report.push_error(format!("row {idx}: status_normalized is blank"));
        }
        if !record.metric_value.is_finite() {
            report.push_error(format!("row {idx}: metric_value is not finite"));
        }

        if !record.event_id.trim().is_empty() && !seen_event_ids.insert(record.event_id.as_str()) {
            report.push_error(format!(
                "row {idx}: duplicate event_id '{}'",
                record.event_id
            ));
        }
    }

    report
}
