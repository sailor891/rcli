mod cli;
mod process;
mod utils;

pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use process::*;
pub use utils::*;
#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExcutor {
    async fn excutor(self) -> anyhow::Result<()>;
}
