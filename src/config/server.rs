use clap::{ArgMatches, Args, parser::ValueSource};
use serde::{Deserialize, Serialize};

#[derive(Debug, Args, Serialize, Deserialize)]
#[command(next_help_heading = "Server Options")]
pub struct ServerConfig {
    #[arg(
        env = "SERVER_LISTEN",
        short = 'l',
        long = "server-listen",
        default_value = "0.0.0.0:8580"
    )]
    pub listen: String,
    #[arg(
        env = "SERVER_ACCESS_URL",
        long = "server-access-url",
        default_value = "http://127.0.0.1:8580"
    )]
    pub access_url: String,
    #[arg(env = "SERVER_ALLOWED_ORIGINS", long = "server-allowed-origins")]
    pub allowed_origins: Vec<String>,
}

impl ServerConfig {
    pub fn merge_with_args(&mut self, matches: &ArgMatches, arg: Self) {
        match matches.value_source("listen") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.listen = arg.listen;
            }
            _ => {}
        }

        match matches.value_source("access_url") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.access_url = arg.access_url;
            }
            _ => {}
        }
        match matches.value_source("allowed_origins") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.allowed_origins = arg.allowed_origins;
            }
            _ => {}
        }
    }
}
