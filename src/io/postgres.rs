// src\io\postgres.rs
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::core::errors::RsFoundryError;

pub async fn create_pool(database_url: &str) -> Result<PgPool, RsFoundryError> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|e| RsFoundryError::Io(format!("failed to connect to postgres: {e}")))
}
