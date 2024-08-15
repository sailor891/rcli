// 向Rust模块系统注册mod
mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use crate::CmdExcutor;

// 向外暴露数据结构
pub use self::csv::{CsvOpts, OutputFormat};
pub use base64::{Base64Format, Base64Subcommand};
use clap::Parser;
pub use genpass::GenPassOpts;
pub use http::HttpSubcommand;
pub use text::{TextSignFormat, TextSubcommand};
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
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64Subcommand),
    #[command(subcommand, about = "Text Signature/Verify")]
    Text(TextSubcommand),
    #[command(subcommand, about = "HTTP Client")]
    Http(HttpSubcommand),
}
impl CmdExcutor for SubCommand {
    async fn excutor(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.excutor().await,
            SubCommand::GenPass(opts) => opts.excutor().await,
            SubCommand::Base64(cmd) => cmd.excutor().await,
            SubCommand::Text(cmd) => cmd.excutor().await,
            SubCommand::Http(cmd) => cmd.excutor().await,
        }
    }
}
