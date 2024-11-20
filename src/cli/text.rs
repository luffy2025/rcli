use crate::cli::verify_input_file;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

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

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct TextGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
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

fn verify_path(value: &str) -> Result<PathBuf, &'static str> {
    let path = PathBuf::from(value);
    if path.exists() && path.is_dir() {
        Ok(path)
    } else {
        Err("Invalid path")
    }
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
