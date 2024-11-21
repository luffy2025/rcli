use enum_dispatch::enum_dispatch;

mod cli;
mod process;
mod utils;

pub use cli::b64::*;
pub use cli::csv::*;
pub use cli::gen_pass::*;
pub use cli::http::*;
pub use cli::text::*;
pub use cli::Opts;
pub use cli::SubCommand;

pub use utils::*;

#[enum_dispatch]
#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
