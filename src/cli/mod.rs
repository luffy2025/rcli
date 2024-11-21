use crate::cli::b64::Base64SubCommand;
use crate::cli::csv::CsvOpts;
use crate::cli::gen_pass::GenPassOpts;
use crate::cli::http::HttpSubCommand;
use crate::cli::text::TextSubCommand;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub(crate) mod b64;
pub(crate) mod csv;
pub(crate) mod gen_pass;
pub(crate) mod http;
pub(crate) mod text;

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[enum_dispatch(CmdExecutor)]
#[derive(Debug, Parser, Serialize, Deserialize)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, name = "base64", about = "Encode or decode base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, name = "crypto", about = "Text subcommand")]
    Text(TextSubCommand),
    #[command(subcommand, name = "http", about = "HTTP subcommand")]
    Http(HttpSubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // FQDN格式的方法调用 xx::yy::zz
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file does not exist")
    }
}

fn verify_path(value: &str) -> std::result::Result<PathBuf, &'static str> {
    let path = PathBuf::from(value);
    if path.exists() && path.is_dir() {
        Ok(path)
    } else {
        Err("Invalid path")
    }
}

// 编译宏，test表示只有在测试时才编译
#[cfg(test)]
// mod可以随意命名，一般规约为tests
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("Input file does not exist"));
        assert_eq!(
            verify_input_file("not-exist"),
            Err("Input file does not exist")
        );
    }
}
