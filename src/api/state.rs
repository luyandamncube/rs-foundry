// src\api\state.rs
use crate::config::settings::Settings;

#[derive(Debug, Clone)]
pub struct AppState {
    pub settings: Settings,
}
