use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{ArgMatches, Args, parser::ValueSource};
use jwt_simple::prelude::Ed25519KeyPair;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Args, Serialize, Deserialize)]
#[command(next_help_heading = "Token Options")]
pub struct TokenConfig {
    #[arg(
        env = "TOKEN_ACCESS_KEY_PATH",
        long = "token-access-key-path",
        default_value = "access_key.pem",
        required_unless_present = "config"
    )]
    pub access_key_path: PathBuf,
    #[arg(
        env = "TOKEN_ACCESS_TOKEN_EXPIRED_MINUTES",
        long = "token-access-token-expired-minutes",
        default_value = "10"
    )]
    pub access_token_expired_minutes: u64,

    #[arg(
        env = "TOKEN_REFRESH_KEY_PATH",
        long = "token-refresh-key-path",
        default_value = "refresh_key.pem",
        required_unless_present = "config"
    )]
    pub refresh_key_path: PathBuf,
    #[arg(
        env = "TOKEN_REFRESH_TOKEN_EXPIRED_MINUTES",
        long = "token-refresh-token-expired-minutes",
        default_value = "120"
    )]
    pub refresh_token_expired_minutes: u64,
    #[arg(
        env = "TOKEN_REFRESH_TOKEN_MAX_EXPIRED_MINUTES",
        long = "token-refresh-token-max-expired-minutes",
        default_value = "1440"
    )]
    pub refresh_token_max_expired_minutes: u64,
}

impl TokenConfig {
    pub fn generate_keys(&mut self, cfg_dir: &Path) -> Result<()> {
        self.access_key_path = cfg_dir.join(&self.access_key_path);
        self.refresh_key_path = cfg_dir.join(&self.refresh_key_path);
        fs::write(&self.access_key_path, Ed25519KeyPair::generate().to_pem())?;
        fs::write(&self.refresh_key_path, Ed25519KeyPair::generate().to_pem())?;
        Ok(())
    }

    pub fn merge_with_args(&mut self, matches: &ArgMatches, arg: Self) {
        match matches.value_source("access_key_path") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.access_key_path = arg.access_key_path;
            }
            _ => {}
        }
        match matches.value_source("access_token_expired_minutes") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.access_token_expired_minutes = arg.access_token_expired_minutes;
            }
            _ => {}
        }
        match matches.value_source("refresh_key_path") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.refresh_key_path = arg.refresh_key_path;
            }
            _ => {}
        }
        match matches.value_source("refresh_token_expired_minutes") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.refresh_token_expired_minutes = arg.refresh_token_expired_minutes;
            }
            _ => {}
        }
        match matches.value_source("refresh_token_max_expired_minutes") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.refresh_token_max_expired_minutes = arg.refresh_token_max_expired_minutes;
            }
            _ => {}
        }
    }
}
