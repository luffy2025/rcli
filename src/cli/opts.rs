use crate::cli::b64::Base64SubCommand;
use crate::cli::csv::CsvOpts;
use crate::cli::gen_pass::GenPassOpts;
use crate::cli::text::TextSubCommand;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, name = "base64", about = "Encode or decode base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, name = "text", about = "Text subcommand")]
    Text(TextSubCommand),
}
