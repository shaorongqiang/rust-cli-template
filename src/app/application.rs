use anyhow::{Context, Result};
use tokio::signal;

#[derive(Debug)]
pub struct Application {
    shutdown: ShutdownSignal,
}

#[derive(Debug, Clone, Copy)]
pub enum ShutdownSignal {
    Disabled,
    CtrlC,
}

impl Application {
    pub fn new() -> Self {
        Self::with_shutdown(ShutdownSignal::Disabled)
    }

    pub fn with_shutdown(shutdown: ShutdownSignal) -> Self {
        Self { shutdown }
    }

    pub async fn run(self) -> Result<()> {
        self.shutdown.wait().await
    }

    pub async fn init_database(self) -> Result<()> {
        Ok(())
    }
}

impl ShutdownSignal {
    async fn wait(self) -> Result<()> {
        match self {
            Self::Disabled => Ok(()),
            Self::CtrlC => {
                signal::ctrl_c()
                    .await
                    .context("failed to listen for Ctrl-C shutdown signal")?;
                tracing::info!("Shutdown signal received");
                Ok(())
            }
        }
    }
}
