#![deny(
    warnings,
    unused_crate_dependencies,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic
)]
mod config;

use std::{env, path::PathBuf};

use anyhow::Result;
use clap::{CommandFactory, Parser};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::{Config, DatabaseConfig, LogConfig, ServerConfig, TokenConfig};

#[derive(Parser)]
pub struct Args {
    #[arg(env = "CONFIG_PATH", short = 'c', long = "config")]
    pub config: Option<PathBuf>,

    #[command(flatten)]
    pub log: LogConfig,
    #[command(flatten)]
    pub server: ServerConfig,
    #[command(flatten)]
    pub token: TokenConfig,
    #[command(flatten)]
    pub db: DatabaseConfig,
}

impl Args {
    pub async fn execute(self) -> Result<()> {
        let cfg = if let Some(cfg_path) = self.config {
            if cfg_path.exists() {
                let mut cfg = Config::load_from_file(cfg_path)?;

                let matches = Self::command().get_matches();
                cfg.log.merge_with_args(&matches, self.log);
                cfg.token.merge_with_args(&matches, self.token);
                cfg.server.merge_with_args(&matches, self.server);
                cfg.db.merge_with_args(&matches, self.db);

                cfg
            } else {
                let mut cfg = Config {
                    log: self.log,
                    token: self.token,
                    server: self.server,
                    db: self.db,
                };
                println!();
                println!("{}", "=".repeat(60));
                println!("⚠️  配置文件未找到，正在生成默认配置文件");
                println!("⚠️  Config file not found, generating default config file");
                println!("{}", "=".repeat(60));
                println!("📁 配置文件路径: {}", cfg_path.display());
                println!("📁 Config file path: {}", cfg_path.display());
                println!("💡 提示: 您可以编辑此配置文件来自定义设置");
                println!("💡 Tip: You can edit this config file to customize settings");
                println!("{}", "=".repeat(60));
                println!();
                cfg.create_file(cfg_path)?;
                println!("✅ 默认配置文件已生成，请编辑后重新运行程序");
                println!(
                    "✅ Default config file has been generated, please edit and rerun the program"
                );
                println!();
                return Ok(());
            }
        } else {
            Config {
                log: self.log,
                token: self.token,
                server: self.server,
                db: self.db,
            }
        };

        let log_builder = tracing_subscriber::registry().with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| cfg.log.level.into()),
        );

        if let Some(log_dir) = cfg.log.dir {
            let file_appender = RollingFileAppender::builder()
                .rotation(Rotation::DAILY)
                .filename_prefix(
                    current_exe_name().ok_or_else(|| anyhow::anyhow!("current exe not found"))?,
                )
                .filename_suffix("log")
                .build(log_dir)?;

            log_builder
                .with(
                    tracing_subscriber::fmt::layer()
                        .with_file(true)
                        .with_line_number(true)
                        .with_thread_names(true)
                        .with_thread_ids(true)
                        .with_writer(file_appender)
                        .with_ansi(false),
                )
                .with(
                    tracing_subscriber::fmt::layer()
                        .with_file(true)
                        .with_line_number(true)
                        .with_thread_names(true)
                        .with_thread_ids(true)
                        .with_ansi(true),
                )
                .init();
        } else {
            log_builder
                .with(
                    tracing_subscriber::fmt::layer()
                        .with_file(true)
                        .with_line_number(true)
                        .with_thread_names(true)
                        .with_thread_ids(true)
                        .with_ansi(true),
                )
                .init();
        }
        tracing::info!("Starting service gateway");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    unsafe {
        if env::var_os("RUST_BACKTRACE").is_none() {
            env::set_var("RUST_BACKTRACE", "full");
        }
    }
    Args::parse().execute().await
}

fn current_exe_name() -> Option<String> {
    env::current_exe().ok().and_then(|exe| {
        exe.file_stem()
            .map(|file_name| file_name.to_string_lossy().to_string())
    })
}
