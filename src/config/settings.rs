// src\config\settings.rs
use serde::Deserialize;
use std::env;

use crate::core::errors::RsFoundryError;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub app_name: String,
    pub app_host: String,
    pub app_port: u16,
    pub app_env: String,
    pub data_root: String,
    pub conf_root: String,
    pub postgres_host: String,
    pub postgres_port: u16,
    pub postgres_db: String,
    pub postgres_user: String,
    pub postgres_password: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            app_name: "rs-foundry".to_string(),
            app_host: "0.0.0.0".to_string(),
            app_port: 8080,
            app_env: "dev".to_string(),
            data_root: "./data".to_string(),
            conf_root: "./conf".to_string(),
            postgres_host: "localhost".to_string(),
            postgres_port: 5432,
            postgres_db: "rs_foundry".to_string(),
            postgres_user: "rs_foundry".to_string(),
            postgres_password: "rs_foundry".to_string(),
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, RsFoundryError> {
        let _ = dotenvy::dotenv();

        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
        let conf_root = env::var("CONF_ROOT").unwrap_or_else(|_| "./conf".to_string());

        let builder = config::Config::builder()
            .add_source(config::File::with_name(&format!("{conf_root}/base/app")).required(true))
            .add_source(
                config::File::with_name(&format!("{conf_root}/{app_env}/app")).required(false),
            )
            .add_source(config::Environment::default().separator("_"));

        let cfg = builder
            .build()
            .map_err(|e| RsFoundryError::Config(format!("failed to build config: {e}")))?;

        cfg.try_deserialize::<Settings>()
            .map_err(|e| RsFoundryError::Config(format!("failed to deserialize config: {e}")))
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_db
        )
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.app_host, self.app_port)
    }
}
