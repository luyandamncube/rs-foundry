// src\core\schemas.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceholderSchema {
    pub name: String,
    pub version: String,
}
