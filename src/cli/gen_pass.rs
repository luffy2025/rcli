use crate::CmdExecutor;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = false)]
    pub no_upper: bool,

    #[arg(long, default_value_t = false)]
    pub no_lower: bool,

    #[arg(long, default_value_t = false)]
    pub no_number: bool,

    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let pass = crate::process::process_genpass(
            self.length,
            self.no_upper,
            self.no_lower,
            self.no_number,
            self.no_symbol,
        )?;
        println!("{}", pass);
        eprintln!("Strength: {}", zxcvbn::zxcvbn(&pass, &[]).score());
        Ok(())
    }
}
