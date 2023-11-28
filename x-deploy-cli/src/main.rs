mod auth;
mod cmd;
mod executor;

use clap::Parser;
use crate::cmd::CmdArgs;
use crate::executor::execute;

#[tokio::main]
async fn main() {
    let cmd_args = CmdArgs::parse();
    execute(cmd_args);
}
