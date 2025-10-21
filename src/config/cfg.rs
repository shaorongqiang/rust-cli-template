use std::{fs, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{DatabaseConfig, LogConfig, ServerConfig, TokenConfig};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub log: LogConfig,
    pub token: TokenConfig,
    pub server: ServerConfig,
    pub db: DatabaseConfig,
}

impl Config {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        Ok(toml::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn create_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        if let Some(p) = path.as_ref().parent() {
            if !p.exists() {
                fs::create_dir_all(p)?;
            }
            self.token.generate_keys(p)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;

        Ok(())
    }
}
