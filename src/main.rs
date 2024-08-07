use clap::Parser;
use rcli::opts::{Opts, SubCommand};
use rcli::process::{csv_convert::process_csv, gen_pass::process_genpass};
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
    }
    Ok(())
}
