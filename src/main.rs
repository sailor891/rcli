use clap::Parser;
use rcli::{CmdExcutor, Opts};
// TODO cargo install --path . 可以将该rcli 安装到全局
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    opts.cmd.excutor().await
}
