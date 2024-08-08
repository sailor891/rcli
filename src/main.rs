use clap::Parser;
use rcli::cli::{Base64Subcommand, Opts, SubCommand};
use rcli::process::{process_csv, process_decode, process_encode, process_genpass};
// rcli csv -i input.csv -o output.json --header -d ','
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
    }
    Ok(())
}
