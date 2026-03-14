// tests\silver_tests.rs
use std::fs;

use chrono::Utc;
use rs_foundry::core::metadata::{SilverDailyRecord, SilverRefRecord};
use rs_foundry::core::types::RunId;
use rs_foundry::jobs::{
    bronze_daily_job, bronze_ref_job, silver_conformed_job, silver_daily_job, silver_ref_job,
};
use rs_foundry::pipelines::silver::build_conformed::build_conformed_records;
use rs_foundry::pipelines::silver::standardize_daily::{
    build_silver_daily_records, dedupe_silver_daily_records, read_bronze_daily_records,
};
use rs_foundry::pipelines::silver::standardize_ref::{
    build_silver_ref_records, dedupe_silver_ref_records, read_bronze_ref_records,
};


use rs_foundry::quality::contracts::{
    validate_silver_conformed_records, validate_silver_daily_records, validate_silver_ref_records,
};

#[test]
fn silver_ref_builder_normalizes_bronze_records() {
    let bronze_result = bronze_ref_job::execute_with_run_id(RunId::new()).unwrap();
    let bronze_records = read_bronze_ref_records(&bronze_result.bronze_path).unwrap();

    let silver_records = build_silver_ref_records(&bronze_records);

    assert_eq!(silver_records.len(), 3);
    assert_eq!(silver_records[0].ref_id, "R001");
    assert_eq!(silver_records[0].ref_name_normalized, "alpha");
    assert_eq!(silver_records[0].ref_type_normalized, "site");
}

#[test]
fn silver_ref_dedup_keeps_latest_record_per_ref_id() {
    let now = Utc::now();

    let records = vec![
        SilverRefRecord {
            ref_id: "R001".to_string(),
            ref_name: "Alpha Old".to_string(),
            ref_type: "site".to_string(),
            ref_name_normalized: "alpha old".to_string(),
            ref_type_normalized: "site".to_string(),
            is_active: true,
            updated_at: None,
            bronze_run_id: "run-1".to_string(),
            source_name: "ref_example".to_string(),
            source_file: None,
            bronze_ingested_at: now,
            silver_built_at: now,
            load_date: "2026-03-14".to_string(),
        },
        SilverRefRecord {
            ref_id: "R001".to_string(),
            ref_name: "Alpha New".to_string(),
            ref_type: "site".to_string(),
            ref_name_normalized: "alpha new".to_string(),
            ref_type_normalized: "site".to_string(),
            is_active: true,
            updated_at: Some(now),
            bronze_run_id: "run-2".to_string(),
            source_name: "ref_example".to_string(),
            source_file: None,
            bronze_ingested_at: now,
            silver_built_at: now,
            load_date: "2026-03-14".to_string(),
        },
    ];

    let deduped = dedupe_silver_ref_records(&records);

    assert_eq!(deduped.len(), 1);
    assert_eq!(deduped[0].ref_name, "Alpha New");
}

#[test]
fn silver_ref_pipeline_writes_silver_output() {
    let bronze_result = bronze_ref_job::execute_with_run_id(RunId::new()).unwrap();

    let silver_result = silver_ref_job::execute(&bronze_result.run_id).unwrap();

    assert_eq!(silver_result.source_name, "ref_example");
    assert_eq!(silver_result.record_count, 3);
    assert!(silver_result.bronze_path.exists());
    assert!(silver_result.silver_path.exists());
    assert!(silver_result.quality_report.passed);

    let silver_content = fs::read_to_string(&silver_result.silver_path).unwrap();
    assert!(silver_content.contains("ref_name_normalized"));
    assert!(silver_content.contains("bronze_run_id"));
}

#[test]
fn silver_daily_builder_normalizes_bronze_records() {
    let bronze_result = bronze_daily_job::execute_with_run_id(RunId::new()).unwrap();
    let bronze_records = read_bronze_daily_records(&bronze_result.bronze_path).unwrap();

    let silver_records = build_silver_daily_records(&bronze_records);

    assert_eq!(silver_records.len(), 3);
    assert_eq!(silver_records[0].event_id, "E001");
    assert_eq!(silver_records[0].status_normalized, "ok");
    assert!(silver_records[0].is_positive_metric);
}

#[test]
fn silver_daily_dedup_keeps_latest_record_per_event_id() {
    let now = Utc::now();

    let records = vec![
        SilverDailyRecord {
            event_id: "E001".to_string(),
            ref_id: "R001".to_string(),
            event_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(),
            metric_value: 1.0,
            status: "old".to_string(),
            status_normalized: "old".to_string(),
            is_positive_metric: true,
            source_ts: None,
            bronze_run_id: "run-1".to_string(),
            source_name: "daily_example".to_string(),
            source_file: None,
            bronze_ingested_at: now,
            silver_built_at: now,
            load_date: "2026-03-14".to_string(),
        },
        SilverDailyRecord {
            event_id: "E001".to_string(),
            ref_id: "R001".to_string(),
            event_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(),
            metric_value: 2.0,
            status: "new".to_string(),
            status_normalized: "new".to_string(),
            is_positive_metric: true,
            source_ts: Some(now),
            bronze_run_id: "run-2".to_string(),
            source_name: "daily_example".to_string(),
            source_file: None,
            bronze_ingested_at: now,
            silver_built_at: now,
            load_date: "2026-03-14".to_string(),
        },
    ];

    let deduped = dedupe_silver_daily_records(&records);

    assert_eq!(deduped.len(), 1);
    assert_eq!(deduped[0].status, "new");
    assert_eq!(deduped[0].metric_value, 2.0);
}

#[test]
fn silver_daily_pipeline_writes_silver_output() {
    let bronze_result = bronze_daily_job::execute_with_run_id(RunId::new()).unwrap();

    let silver_result = silver_daily_job::execute(&bronze_result.run_id).unwrap();

    assert_eq!(silver_result.source_name, "daily_example");
    assert_eq!(silver_result.record_count, 3);
    assert!(silver_result.bronze_path.exists());
    assert!(silver_result.silver_path.exists());
    assert!(silver_result.quality_report.passed);

    let silver_content = fs::read_to_string(&silver_result.silver_path).unwrap();
    assert!(silver_content.contains("status_normalized"));
    assert!(silver_content.contains("is_positive_metric"));
    assert!(silver_content.contains("bronze_run_id"));
}

#[test]
fn conformed_builder_joins_ref_and_daily() {
    let now = Utc::now();

    let refs = vec![SilverRefRecord {
        ref_id: "R001".to_string(),
        ref_name: "Alpha".to_string(),
        ref_type: "site".to_string(),
        ref_name_normalized: "alpha".to_string(),
        ref_type_normalized: "site".to_string(),
        is_active: true,
        updated_at: None,
        bronze_run_id: "ref-run".to_string(),
        source_name: "ref_example".to_string(),
        source_file: None,
        bronze_ingested_at: now,
        silver_built_at: now,
        load_date: "2026-03-14".to_string(),
    }];

    let dailies = vec![SilverDailyRecord {
        event_id: "E001".to_string(),
        ref_id: "R001".to_string(),
        event_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(),
        metric_value: 10.0,
        status: "ok".to_string(),
        status_normalized: "ok".to_string(),
        is_positive_metric: true,
        source_ts: None,
        bronze_run_id: "daily-run".to_string(),
        source_name: "daily_example".to_string(),
        source_file: None,
        bronze_ingested_at: now,
        silver_built_at: now,
        load_date: "2026-03-14".to_string(),
    }];

    let conformed = build_conformed_records(&refs, &dailies, "ref-run", "daily-run");

    assert_eq!(conformed.len(), 1);
    assert_eq!(conformed[0].event_id, "E001");
    assert_eq!(conformed[0].ref_name, "Alpha");
    assert_eq!(conformed[0].status_normalized, "ok");
}

#[test]
fn silver_conformed_pipeline_writes_output() {
    let bronze_ref = bronze_ref_job::execute_with_run_id(RunId::new()).unwrap();
    let bronze_daily = bronze_daily_job::execute_with_run_id(RunId::new()).unwrap();

    let result = silver_conformed_job::execute(&bronze_ref.run_id, &bronze_daily.run_id).unwrap();

    assert_eq!(result.record_count, 3);
    assert!(result.conformed_path.exists());
    assert!(result.quality_report.passed);

    let content = fs::read_to_string(&result.conformed_path).unwrap();
    assert!(content.contains("ref_name"));
    assert!(content.contains("status_normalized"));
    assert!(content.contains("daily_bronze_run_id"));
}

#[test]
fn silver_ref_contract_fails_on_duplicate_ref_id() {
    let now = Utc::now();

    let records = vec![
        SilverRefRecord {
            ref_id: "R001".to_string(),
            ref_name: "Alpha".to_string(),
            ref_type: "site".to_string(),
            ref_name_normalized: "alpha".to_string(),
            ref_type_normalized: "site".to_string(),
            is_active: true,
            updated_at: None,
            bronze_run_id: "run-1".to_string(),
            source_name: "ref_example".to_string(),
            source_file: None,
            bronze_ingested_at: now,
            silver_built_at: now,
            load_date: "2026-03-14".to_string(),
        },
        SilverRefRecord {
            ref_id: "R001".to_string(),
            ref_name: "Alpha Duplicate".to_string(),
            ref_type: "site".to_string(),
            ref_name_normalized: "alpha duplicate".to_string(),
            ref_type_normalized: "site".to_string(),
            is_active: true,
            updated_at: None,
            bronze_run_id: "run-2".to_string(),
            source_name: "ref_example".to_string(),
            source_file: None,
            bronze_ingested_at: now,
            silver_built_at: now,
            load_date: "2026-03-14".to_string(),
        },
    ];

    let report = validate_silver_ref_records(&records);

    assert!(!report.passed);
    assert!(report.errors.iter().any(|e| e.contains("duplicate ref_id")));
}

#[test]
fn silver_daily_contract_fails_on_duplicate_event_id() {
    let now = Utc::now();

    let records = vec![
        SilverDailyRecord {
            event_id: "E001".to_string(),
            ref_id: "R001".to_string(),
            event_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(),
            metric_value: 1.0,
            status: "ok".to_string(),
            status_normalized: "ok".to_string(),
            is_positive_metric: true,
            source_ts: None,
            bronze_run_id: "run-1".to_string(),
            source_name: "daily_example".to_string(),
            source_file: None,
            bronze_ingested_at: now,
            silver_built_at: now,
            load_date: "2026-03-14".to_string(),
        },
        SilverDailyRecord {
            event_id: "E001".to_string(),
            ref_id: "R002".to_string(),
            event_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(),
            metric_value: 2.0,
            status: "ok".to_string(),
            status_normalized: "ok".to_string(),
            is_positive_metric: true,
            source_ts: None,
            bronze_run_id: "run-2".to_string(),
            source_name: "daily_example".to_string(),
            source_file: None,
            bronze_ingested_at: now,
            silver_built_at: now,
            load_date: "2026-03-14".to_string(),
        },
    ];

    let report = validate_silver_daily_records(&records);

    assert!(!report.passed);
    assert!(report.errors.iter().any(|e| e.contains("duplicate event_id")));
}

#[test]
fn silver_conformed_contract_fails_on_duplicate_event_id() {
    let now = Utc::now();

    let records = vec![
        rs_foundry::core::metadata::SilverConformedRecord {
            event_id: "E001".to_string(),
            ref_id: "R001".to_string(),
            ref_name: "Alpha".to_string(),
            ref_type: "site".to_string(),
            ref_name_normalized: "alpha".to_string(),
            ref_type_normalized: "site".to_string(),
            ref_is_active: true,
            event_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(),
            metric_value: 10.0,
            status: "ok".to_string(),
            status_normalized: "ok".to_string(),
            is_positive_metric: true,
            source_ts: None,
            ref_bronze_run_id: "ref-run".to_string(),
            daily_bronze_run_id: "daily-run".to_string(),
            silver_built_at: now,
            load_date: "2026-03-14".to_string(),
        },
        rs_foundry::core::metadata::SilverConformedRecord {
            event_id: "E001".to_string(),
            ref_id: "R001".to_string(),
            ref_name: "Alpha".to_string(),
            ref_type: "site".to_string(),
            ref_name_normalized: "alpha".to_string(),
            ref_type_normalized: "site".to_string(),
            ref_is_active: true,
            event_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(),
            metric_value: 12.0,
            status: "ok".to_string(),
            status_normalized: "ok".to_string(),
            is_positive_metric: true,
            source_ts: None,
            ref_bronze_run_id: "ref-run".to_string(),
            daily_bronze_run_id: "daily-run".to_string(),
            silver_built_at: now,
            load_date: "2026-03-14".to_string(),
        },
    ];

    let report = validate_silver_conformed_records(&records);

    assert!(!report.passed);
    assert!(report.errors.iter().any(|e| e.contains("duplicate event_id")));
}
