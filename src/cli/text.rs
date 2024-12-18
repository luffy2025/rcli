use crate::cli::verify_input_file;
use crate::cli::verify_path;
use crate::CmdExecutor;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[enum_dispatch(CmdExecutor)]
#[derive(Debug, Parser, Serialize, Deserialize)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),

    #[command(name = "verify", about = "Verify a message")]
    Verify(TextVerifyOpts),

    #[command(name = "generate", about = "Generate a key pair")]
    Generate(TextGenerateOpts),
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_input_file)]
    pub key: String,

    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sig = crate::process::process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", sig);
        Ok(())
    }
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_input_file)]
    pub key: String,

    #[arg(short, long)]
    pub signature: String,

    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = crate::process::process_text_verify(
            &self.input,
            &self.key,
            &self.signature,
            self.format,
        )?;
        print!("{}", verified);
        Ok(())
    }
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct TextGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

impl CmdExecutor for TextGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = crate::process::process_text_generate(self.format)?;

        match self.format {
            TextSignFormat::Blake3 => {
                let path = Path::join(&self.output, "blake3.key");
                std::fs::write(path, key[0].as_slice())?;
            }
            TextSignFormat::Ed25519 => {
                let path = Path::join(&self.output, "ed25519.key");
                std::fs::write(path, key[0].as_slice())?;
            }
            TextSignFormat::ChaCha20Poly1305 => {
                let path = Path::join(&self.output, "chacha20poly1305.key");
                std::fs::write(path, key[0].as_slice())?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TextSignFormat {
    Blake3,
    ChaCha20Poly1305,
    Ed25519,
}

fn parse_format(value: &str) -> Result<TextSignFormat, &'static str> {
    value.parse().map_err(|_| "Invalid format")
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Ed25519 => "ed25519",
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::ChaCha20Poly1305 => "chacha20poly1305",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "ed25519" => Ok(TextSignFormat::Ed25519),
            "blake3" => Ok(TextSignFormat::Blake3),
            "chacha20poly1305" => Ok(TextSignFormat::ChaCha20Poly1305),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
