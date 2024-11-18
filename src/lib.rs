mod cli;
mod process;

pub use cli::b64::Base64SubCommand;
pub use cli::opts::{Opts, SubCommand};
pub use process::{process_csv, process_genpass};
pub use process::{process_decode, process_encode};
