mod cli;
mod process;
mod utils;

pub use cli::{Base64Subcommand, HttpSubcommand, Opts, SubCommand, TextSignFormat, TextSubcommand};
pub use process::*;
pub use utils::*;
#[allow(async_fn_in_trait)]
pub trait CmdExcutor {
    async fn excutor(self) -> anyhow::Result<()>;
}
