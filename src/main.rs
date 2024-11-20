use anyhow::Result;
use clap::Parser;
use rcli::TextSignFormat::{Blake3, ChaCha20Poly1305, Ed25519};
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_generate,
    process_text_sign, process_text_verify, Base64SubCommand, Opts, SubCommand, TextSubCommand,
};
use std::path::Path;
use zxcvbn::zxcvbn;

fn main() -> Result<()> {
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
        SubCommand::GenPass(opts) => {
            let pass = process_genpass(
                opts.length,
                opts.no_upper,
                opts.no_lower,
                opts.no_number,
                opts.no_symbol,
            )?;
            println!("{}", pass);
            eprintln!("Strength: {}", zxcvbn(&pass, &[]).score());
        }

        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format);
                println!("{}", String::from_utf8(decoded?)?);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sig)
            }
            TextSubCommand::Verify(opts) => {
                let verified =
                    process_text_verify(&opts.input, &opts.key, &opts.signature, opts.format)?;
                print!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;

                match opts.format {
                    Blake3 => {
                        let path = Path::join(&opts.output, "blake3.key");
                        std::fs::write(path, &key[0])?
                    }
                    Ed25519 => {
                        let path = Path::join(&opts.output, "ed25519.sk");
                        std::fs::write(path, &key[0])?;
                        let path = Path::join(&opts.output, "ed25519.pk");
                        std::fs::write(path, &key[1])?
                    }
                    ChaCha20Poly1305 => {
                        let path = Path::join(&opts.output, "chacha20poly1305.key");
                        std::fs::write(path, &key[0])?
                    }
                }
            }
        },
    }
    Ok(())
}
