//! Configuration loading from TOML files

use serde::{Deserialize, Serialize};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub opacity: f32,
}

pub fn load_config(_path: &str) -> anyhow::Result<Config> {
    Ok(Config { opacity: 1.0 })
}
