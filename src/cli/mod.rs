// 向Rust模块系统注册mod
mod base64;
mod csv;
mod genpass;
mod text;
// 向外暴露数据结构
pub use self::csv::{CsvOpts, OutputFormat};
pub use base64::{Base64Format, Base64Subcommand};
pub use genpass::GenPassOpts;
pub use text::{TextSignFormat, TextSubcommand};

use clap::Parser;
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
    #[command(subcommand)]
    Text(TextSubcommand),
}
