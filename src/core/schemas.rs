// src\core\schemas.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSchema {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetSchema {
    pub dataset_name: String,
    pub version: String,
    pub fields: Vec<FieldSchema>,
}

pub fn bronze_ref_schema() -> DatasetSchema {
    DatasetSchema {
        dataset_name: "bronze_ref".to_string(),
        version: "v1".to_string(),
        fields: vec![
            FieldSchema {
                name: "ref_id".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "ref_name".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "ref_type".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "is_active".to_string(),
                data_type: "bool".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "updated_at".to_string(),
                data_type: "timestamp".to_string(),
                nullable: true,
            },
            FieldSchema {
                name: "run_id".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "source_name".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "source_file".to_string(),
                data_type: "string".to_string(),
                nullable: true,
            },
            FieldSchema {
                name: "ingested_at".to_string(),
                data_type: "timestamp".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "load_date".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "record_hash".to_string(),
                data_type: "string".to_string(),
                nullable: true,
            },
        ],
    }
}

pub fn bronze_daily_schema() -> DatasetSchema {
    DatasetSchema {
        dataset_name: "bronze_daily".to_string(),
        version: "v1".to_string(),
        fields: vec![
            FieldSchema {
                name: "event_id".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "ref_id".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "event_date".to_string(),
                data_type: "date".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "metric_value".to_string(),
                data_type: "float64".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "status".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "source_ts".to_string(),
                data_type: "timestamp".to_string(),
                nullable: true,
            },
            FieldSchema {
                name: "run_id".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "source_name".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "source_file".to_string(),
                data_type: "string".to_string(),
                nullable: true,
            },
            FieldSchema {
                name: "ingested_at".to_string(),
                data_type: "timestamp".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "load_date".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            FieldSchema {
                name: "record_hash".to_string(),
                data_type: "string".to_string(),
                nullable: true,
            },
        ],
    }
}
