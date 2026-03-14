// src\core\errors.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RsFoundryError {
    #[error("configuration error: {0}")]
    Config(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("schema error: {0}")]
    Schema(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("source error: {0}")]
    Source(String),

    #[error("pipeline error: {0}")]
    Pipeline(String),

    #[error("not implemented: {0}")]
    NotImplemented(String),
}
