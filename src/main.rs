use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_generate,
    process_text_sign, process_text_verify,
};
use rcli::{Base64Subcommand, Opts, SubCommand, TextSubcommand};
use std::fs;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
                opts.length,
            )?;
            println!("{}", password);
        }
        SubCommand::Base64(subcommand) => match subcommand {
            Base64Subcommand::Encode(encode) => {
                let encoded = process_encode(&encode.input, encode.format)?;
                println!("{}", encoded);
            }
            Base64Subcommand::Decode(decode) => {
                let decoded = process_decode(&decode.input, decode.format)?;
                println!("{}", decoded);
            }
        },
        SubCommand::Text(subcommand) => match subcommand {
            TextSubcommand::Sign(opts) => {
                let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubcommand::Verify(opts) => {
                match process_text_verify(&opts.input, &opts.key, &opts.sign, opts.format)? {
                    true => println!("✓ Signature verified"),
                    false => println!("✗ Signature verification failed"),
                }
            }
            TextSubcommand::Generate(opts) => {
                let keys = process_text_generate(opts.format)?;
                for (k, v) in keys {
                    fs::write(opts.output.join(k), v)?;
                }
            }
        },
    }
    Ok(())
}
