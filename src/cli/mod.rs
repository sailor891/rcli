// 向Rust模块系统注册mod
mod base64;
mod csv;
mod genpass;
mod http;
mod text;

// 向外暴露数据结构
pub use self::{base64::*, csv::*, genpass::*, http::*, text::*};
use clap::Parser;
use enum_dispatch::enum_dispatch;
#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcutor)]
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
