use crate::cli::verify_input_file;
use crate::cli::verify_path;
use crate::process::{process_generate, process_sign};
use crate::CmdExecutor;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[enum_dispatch(CmdExecutor)]
#[derive(Debug, Parser, Serialize, Deserialize)]
pub enum JwtSubCommand {
    Sign(JwtSignOpts),
    Verify(JwtVerifyOpts),
    Generate(JwtGenerateOpts),
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct JwtSignOpts {
    #[arg(long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long, value_parser = parse_exp_time)]
    pub exp: Duration,
    #[arg(long, value_parser = verify_input_file, default_value = "fixtures/jwt.secret")]
    pub secret_file: String,
}

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = process_sign(&self.sub, &self.aud, self.exp, &self.secret_file)?;
        println!("{}", token);
        Ok(())
    }
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct JwtVerifyOpts {
    #[arg(short, long, default_value = "-")]
    pub token: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long, value_parser = verify_input_file, default_value = "fixtures/jwt.secret")]
    pub secret: String,
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let legal = crate::process::process_verify(&self.token, &self.aud, &self.secret)?;
        println!("{:?}", legal);
        Ok(())
    }
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct JwtGenerateOpts {
    #[arg(short, long)]
    pub len: u8,
    #[arg(short, long, value_parser = verify_path, default_value = "fixtures/")]
    pub output: PathBuf,
}

impl CmdExecutor for JwtGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let secret = process_generate(self.len)?;
        let output = self.output.join("jwt.secret");
        fs::write(output, secret)?;
        Ok(())
    }
}

fn parse_exp_time(value: &str) -> Result<Duration, anyhow::Error> {
    Ok(humantime::parse_duration(value)?)
}
