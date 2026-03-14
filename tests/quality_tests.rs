// tests\quality_tests.rs
use chrono::{NaiveDate, Utc};

use rs_foundry::core::metadata::{BronzeDailyRecord, BronzeRefRecord};
use rs_foundry::quality::checks::{
    check_non_empty, validate_bronze_daily_records, validate_bronze_ref_records,
};

#[test]
fn non_empty_check_fails_for_empty_dataset() {
    let report = check_non_empty(0);

    assert!(!report.passed);
    assert_eq!(report.row_count, 0);
    assert_eq!(report.errors.len(), 1);
}

#[test]
fn bronze_ref_quality_passes_for_valid_records() {
    let records = vec![BronzeRefRecord {
        ref_id: "R001".to_string(),
        ref_name: "Alpha".to_string(),
        ref_type: "site".to_string(),
        is_active: true,
        updated_at: None,
        run_id: "run-1".to_string(),
        source_name: "ref_example".to_string(),
        source_file: Some("ref_source.json".to_string()),
        ingested_at: Utc::now(),
        load_date: "2026-03-14".to_string(),
        record_hash: None,
    }];

    let report = validate_bronze_ref_records(&records);

    assert!(report.passed);
    assert_eq!(report.errors.len(), 0);
    assert_eq!(report.row_count, 1);
}

#[test]
fn bronze_ref_quality_fails_for_blank_required_fields() {
    let records = vec![BronzeRefRecord {
        ref_id: "".to_string(),
        ref_name: "".to_string(),
        ref_type: "".to_string(),
        is_active: true,
        updated_at: None,
        run_id: "run-1".to_string(),
        source_name: "ref_example".to_string(),
        source_file: Some("ref_source.json".to_string()),
        ingested_at: Utc::now(),
        load_date: "2026-03-14".to_string(),
        record_hash: None,
    }];

    let report = validate_bronze_ref_records(&records);

    assert!(!report.passed);
    assert_eq!(report.errors.len(), 3);
}

#[test]
fn bronze_daily_quality_passes_for_valid_records() {
    let records = vec![BronzeDailyRecord {
        event_id: "E001".to_string(),
        ref_id: "R001".to_string(),
        event_date: NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(),
        metric_value: 10.0,
        status: "ok".to_string(),
        source_ts: None,
        run_id: "run-1".to_string(),
        source_name: "daily_example".to_string(),
        source_file: Some("daily_source.json".to_string()),
        ingested_at: Utc::now(),
        load_date: "2026-03-14".to_string(),
        record_hash: None,
    }];

    let report = validate_bronze_daily_records(&records);

    assert!(report.passed);
    assert_eq!(report.errors.len(), 0);
}

#[test]
fn bronze_daily_quality_fails_for_blank_required_fields() {
    let records = vec![BronzeDailyRecord {
        event_id: "".to_string(),
        ref_id: "".to_string(),
        event_date: NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(),
        metric_value: 10.0,
        status: "".to_string(),
        source_ts: None,
        run_id: "run-1".to_string(),
        source_name: "daily_example".to_string(),
        source_file: Some("daily_source.json".to_string()),
        ingested_at: Utc::now(),
        load_date: "2026-03-14".to_string(),
        record_hash: None,
    }];

    let report = validate_bronze_daily_records(&records);

    assert!(!report.passed);
    assert_eq!(report.errors.len(), 3);
}
