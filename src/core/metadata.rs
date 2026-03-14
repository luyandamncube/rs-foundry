// src\core\metadata.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::types::{Layer, SourceKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionMetadata {
    pub run_id: String,
    pub source_name: String,
    pub source_kind: SourceKind,
    pub layer: Layer,
    pub source_file: Option<String>,
    pub ingested_at: DateTime<Utc>,
    pub load_date: String,
    pub record_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BronzeRefRecord {
    pub ref_id: String,
    pub ref_name: String,
    pub ref_type: String,
    pub is_active: bool,
    pub updated_at: Option<DateTime<Utc>>,

    pub run_id: String,
    pub source_name: String,
    pub source_file: Option<String>,
    pub ingested_at: DateTime<Utc>,
    pub load_date: String,
    pub record_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BronzeDailyRecord {
    pub event_id: String,
    pub ref_id: String,
    pub event_date: chrono::NaiveDate,
    pub metric_value: f64,
    pub status: String,
    pub source_ts: Option<DateTime<Utc>>,

    pub run_id: String,
    pub source_name: String,
    pub source_file: Option<String>,
    pub ingested_at: DateTime<Utc>,
    pub load_date: String,
    pub record_hash: Option<String>,
}
