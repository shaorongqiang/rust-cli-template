use std::{
    fs::{File, create_dir_all},
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};
use clap::{
    ArgMatches, Args, CommandFactory, FromArgMatches, Parser, Subcommand, parser::ValueSource,
};
use clap_complete::{Shell, generate};

use application::{
    Application, CONFIG_FILE_NAME, Config, DatabaseConfig, LogConfig, ServerConfig, ShutdownSignal,
    TokenConfig, current_exe_name, init_tracing,
};

#[derive(Args, Debug)]
pub struct CommonConfigArgs {
    #[command(flatten)]
    pub log: LogConfig,
    #[command(flatten)]
    pub server: ServerConfig,
    #[command(flatten)]
    pub token: TokenConfig,
    #[command(flatten)]
    pub db: DatabaseConfig,
}
impl From<CommonConfigArgs> for Config {
    fn from(args: CommonConfigArgs) -> Self {
        Self {
            log: args.log,
            token: args.token,
            server: args.server,
            db: args.db,
        }
    }
}

#[derive(Parser, Debug)]
pub struct CmdArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Run {
        #[arg(long, short = 'c', default_value = CONFIG_FILE_NAME)]
        config: String,
        #[command(flatten)]
        common: Box<CommonConfigArgs>,
    },
    Configure {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    Completions {
        #[arg(value_enum)]
        shell: Shell,
        #[arg(long, short = 'o')]
        output: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    Generate {
        #[arg(long, short = 'o', default_value = CONFIG_FILE_NAME)]
        output: String,
        #[arg(long, short = 'f')]
        force: bool,
        #[command(flatten)]
        common: Box<CommonConfigArgs>,
    },
    GenerateKeys {
        #[arg(long, short = 'c', default_value = CONFIG_FILE_NAME)]
        config: String,
    },
    DatabaseInit {
        #[arg(long, short = 'c', default_value = CONFIG_FILE_NAME)]
        config: String,
    },
}

impl CmdArgs {
    pub async fn execute(self, matches: &ArgMatches) -> Result<()> {
        match self.command {
            Command::Run { config, common } => {
                let cfg_path = PathBuf::from(config);
                let run_matches = matches
                    .subcommand_matches("run")
                    .ok_or_else(|| anyhow::anyhow!("run command matches not found"))?;
                Self::run(cfg_path, *common, run_matches).await
            }

            Command::Configure { command } => match command {
                ConfigCommand::Generate {
                    output,
                    force,
                    common,
                } => Self::generate_config(PathBuf::from(output), force, *common),
                ConfigCommand::GenerateKeys { config } => {
                    Self::generate_keys(PathBuf::from(config))
                }
                ConfigCommand::DatabaseInit { config } => {
                    Self::init_database(PathBuf::from(config)).await
                }
            },

            Command::Completions { shell, output } => Self::write_completions(shell, &output),
        }
    }

    fn write_completions(shell: Shell, output: &Path) -> Result<()> {
        if let Some(parent) = output
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            create_dir_all(parent)?;
        }

        let mut file = File::create(output)?;
        let mut command = Self::command();
        let bin_name = command.get_name().to_string();
        generate(shell, &mut command, bin_name, &mut file);
        eprintln!("Completion script generated: {}", output.display());
        Ok(())
    }

    async fn run(
        cfg_path: PathBuf,
        common: CommonConfigArgs,
        run_matches: &ArgMatches,
    ) -> Result<()> {
        let exe_name =
            current_exe_name().ok_or_else(|| anyhow::anyhow!("current exe not found"))?;
        require_token_key_paths_without_config(run_matches)?;

        if !cfg_path.exists() {
            return Err(anyhow!(
                "config file not found: {} (run `{} config generate --output {}` first)",
                cfg_path.display(),
                exe_name,
                cfg_path.display()
            ));
        }

        let mut cfg = Config::load_from_file(&cfg_path)?;

        cfg.log.merge_with_args(run_matches, common.log);
        cfg.token.merge_with_args(run_matches, common.token);
        cfg.server.merge_with_args(run_matches, common.server);
        cfg.db.merge_with_args(run_matches, common.db);

        cfg.validate()?;
        let _tracing_guard = init_tracing(&cfg.log, &exe_name)?;
        Application::with_shutdown(ShutdownSignal::CtrlC)
            .run()
            .await
    }

    fn generate_config(cfg_path: PathBuf, force: bool, common: CommonConfigArgs) -> Result<()> {
        if cfg_path.exists() && !force {
            return Err(anyhow!(
                "config file already exists: {} (use --force to overwrite)",
                cfg_path.display()
            ));
        }

        let mut cfg = Config::from(common);
        cfg.create_file(&cfg_path)?;
        eprintln!("Config file generated: {}", cfg_path.display());
        Ok(())
    }

    fn generate_keys(cfg_path: PathBuf) -> Result<()> {
        let exe_name =
            current_exe_name().ok_or_else(|| anyhow::anyhow!("current exe not found"))?;
        if !cfg_path.exists() {
            return Err(anyhow!(
                "config file not found: {} (run `{} config generate --output {}` first)",
                cfg_path.display(),
                exe_name,
                cfg_path.display()
            ));
        }
        Config::load_from_file(&cfg_path).and_then(|cfg| cfg.token.generate_keys())?;

        eprintln!("Key files generated from config: {}", cfg_path.display());
        Ok(())
    }

    async fn init_database(cfg_path: PathBuf) -> Result<()> {
        let exe_name =
            current_exe_name().ok_or_else(|| anyhow::anyhow!("current exe not found"))?;
        if !cfg_path.exists() {
            return Err(anyhow!(
                "config file not found: {} (run `{} config generate --output {}` first)",
                cfg_path.display(),
                exe_name,
                cfg_path.display()
            ));
        }

        let cfg = Config::load_from_file(&cfg_path)?;
        let _tracing_guard = init_tracing(&cfg.log, &exe_name)?;
        Application::new().init_database().await?;
        tracing::info!(
            "Database initialization completed from config: {}",
            cfg_path.display()
        );
        Ok(())
    }
}

fn require_token_key_paths_without_config(matches: &ArgMatches) -> Result<()> {
    if matches.value_source("config") == Some(ValueSource::CommandLine) {
        return Ok(());
    }

    if matches.value_source("access_key_path") != Some(ValueSource::CommandLine) {
        return Err(anyhow!(
            "--token-access-key-path is required when --config is not provided"
        ));
    }

    if matches.value_source("refresh_key_path") != Some(ValueSource::CommandLine) {
        return Err(anyhow!(
            "--token-refresh-key-path is required when --config is not provided"
        ));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = CmdArgs::command().get_matches();
    let args = CmdArgs::from_arg_matches(&matches)?;
    args.execute(&matches).await
}
