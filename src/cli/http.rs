use crate::cli::verify_path;
use crate::CmdExecutor;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Parser, Serialize, Deserialize)]
pub enum HttpSubCommand {
    #[command(about = "ve a directory over HTTP")]
    Serve(HttpServeOpts),
}

impl CmdExecutor for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process::process_http_serve(self.dir.clone(), self.port).await
    }
}
