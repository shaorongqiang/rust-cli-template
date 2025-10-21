use std::path::PathBuf;

use clap::{ArgMatches, Args, parser::ValueSource};
use serde::{Deserialize, Serialize};

#[derive(Debug, Args, Serialize, Deserialize)]
#[command(next_help_heading = "Logging Options")]
pub struct LogConfig {
    #[arg(env = "LOG_DIR", short = 'D', long = "log-dir")]
    pub dir: Option<PathBuf>,
    #[arg(
        env = "LOG_LEVEL",
        short = 'L',
        long = "log-level",
        default_value = "info"
    )]
    pub level: String,
}

impl LogConfig {
    pub fn merge_with_args(&mut self, matches: &ArgMatches, arg: Self) {
        match matches.value_source("dir") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.dir = arg.dir;
            }
            _ => {}
        }

        match matches.value_source("level") {
            Some(ValueSource::CommandLine) | Some(ValueSource::EnvVariable) => {
                self.level = arg.level;
            }
            _ => {}
        }
    }
}
