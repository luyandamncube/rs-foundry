// src\core\types.rs
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RunId(pub Uuid);

impl RunId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for RunId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Created,
    Running,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRecord {
    pub run_id: RunId,
    pub job_name: String,
    pub status: RunStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    Reference,
    Daily,
    Event,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Layer {
    Raw,
    Bronze,
    Silver,
    Gold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefSourceRecord {
    pub ref_id: String,
    pub ref_name: String,
    pub ref_type: String,
    pub is_active: bool,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySourceRecord {
    pub event_id: String,
    pub ref_id: String,
    pub event_date: NaiveDate,
    pub metric_value: f64,
    pub status: String,
    pub source_ts: Option<DateTime<Utc>>,
}
