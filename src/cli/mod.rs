use crate::{CmdExecutor, SubCommand};
use std::path::{Path, PathBuf};

pub(crate) mod b64;
pub(crate) mod csv;
pub(crate) mod gen_pass;
pub(crate) mod http;
pub(crate) mod opts;
pub(crate) mod text;

impl CmdExecutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::GenPass(opts) => opts.execute().await,
            SubCommand::Base64(subcmd) => subcmd.execute().await,
            SubCommand::Text(subcmd) => subcmd.execute().await,
            SubCommand::Http(subcmd) => subcmd.execute().await,
        }
    }
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
