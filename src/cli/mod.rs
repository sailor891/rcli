mod base64;
mod csv;
mod genpass;

pub use self::csv::{CsvOpts, OutputFormat};
pub use base64::{Base64Format, Base64Subcommand};
use clap::Parser;
pub use genpass::GenPassOpts;

#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "CSV to Others")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate Password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64Subcommand),
}
