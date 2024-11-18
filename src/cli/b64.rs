use crate::cli::verify_input_file;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Parser, Serialize, Deserialize)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode base64")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Decode base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "standard" )]
    pub format: Base64Format,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Unsupported format {:?}", value)),
        }
    }
}

fn parse_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}
