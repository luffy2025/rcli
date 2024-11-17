// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;

use rcli::{process_csv, process_genpass, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                format!("output/{}", output.clone())
            } else {
                format!("output/output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::GenPass(opts) => process_genpass(
            opts.length,
            opts.no_upper,
            opts.no_lower,
            opts.no_number,
            opts.no_symbol,
        )?,
    }
    Ok(())
}
