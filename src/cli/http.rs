use crate::{process_http_serve, CmdExcutor};

use super::super::verify_path;
// use anyhow::Result;
use clap::Parser;
// use std::fmt;
use std::path::PathBuf;
// use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum HttpSubcommand {
    #[command(about = "serve a http server")]
    Serve(HttpServeOpts),
}
#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short,long, default_value = ".",value_parser=verify_path)]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
impl CmdExcutor for HttpSubcommand {
    async fn excutor(self) -> anyhow::Result<()> {
        match self {
            HttpSubcommand::Serve(opts) => opts.excutor().await,
        }
    }
}
impl CmdExcutor for HttpServeOpts {
    async fn excutor(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await
    }
}
