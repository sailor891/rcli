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
