// src\api\state.rs
use sqlx::PgPool;

use crate::config::settings::Settings;

#[derive(Debug, Clone)]
pub struct AppState {
    pub settings: Settings,
    pub db_pool: PgPool,
}
