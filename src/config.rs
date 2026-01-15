use std::env;

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub steam_id: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, AppError> {
        Ok(Self {
            steam_id: get_env_var("STEAM_ID")?,
            database_url: get_env_var("DATABASE_URL")?,
        })
    }
}

fn get_env_var(name: &str) -> Result<String, AppError> {
    env::var(name).map_err(|_| AppError::Env(name.to_string()))
}
