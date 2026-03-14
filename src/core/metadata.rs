// src\core\metadata.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionMetadata {
    pub source_name: String,
    pub source_file: Option<String>,
    pub ingested_at: DateTime<Utc>,
    pub load_date: String,
    pub record_hash: Option<String>,
    pub run_id: String,
}
