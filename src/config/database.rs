use clap::{ArgMatches, Args, parser::ValueSource};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Args, Serialize, Deserialize)]
#[command(next_help_heading = "Database Options")]
pub struct DatabaseConfig {
    #[arg(
        env = "DATABASE_URL",
        long = "database-url",
        default_value = "postgres://postgres:123456@127.0.0.1:5432/postgres"
    )]
    pub url: String,
    #[serde(skip)]
    #[arg(
        long = "database-init",
        default_value = "false",
        help = "Initialize database"
    )]
    pub init: bool,
    #[arg(env = "DATABASE_ENABLE_LOGGING", long = "database-enable-logging")]
    pub enable_logging: bool,
    #[arg(env = "DATABASE_MIN_CONNECTIONS", long = "database-min-connections")]
    pub min_connections: Option<u32>,
    #[arg(env = "DATABASE_MAX_CONNECTIONS", long = "database-max-connections")]
    pub max_connections: Option<u32>,
    #[arg(env = "DATABASE_CONNECT_TIMEOUT", long = "database-connect-timeout")]
    pub connect_timeout: Option<u64>,
    #[arg(env = "DATABASE_IDLE_TIMEOUT", long = "database-idle-timeout")]
    pub idle_timeout: Option<u64>,
}

impl DatabaseConfig {
    pub fn merge_with_args(&mut self, matches: &ArgMatches, arg: Self) {
        match matches.value_source("url") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.url = arg.url;
            }
            _ => {}
        }

        match matches.value_source("init") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.init = arg.init;
            }
            _ => {}
        }

        match matches.value_source("enable_logging") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.enable_logging = arg.enable_logging;
            }
            _ => {}
        }

        match matches.value_source("min_connections") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.min_connections = arg.min_connections;
            }
            _ => {}
        }

        match matches.value_source("max_connections") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.max_connections = arg.max_connections;
            }
            _ => {}
        }

        match matches.value_source("connect_timeout") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.connect_timeout = arg.connect_timeout;
            }
            _ => {}
        }

        match matches.value_source("idle_timeout") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.idle_timeout = arg.idle_timeout;
            }
            _ => {}
        }
    }
}
