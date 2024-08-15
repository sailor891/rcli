use super::super::verify_path;
use crate::{process_http_serve, CmdExcutor};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcutor)]
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

impl CmdExcutor for HttpServeOpts {
    async fn excutor(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await
    }
}
