mod auth;
mod cmd;
mod executor;

use crate::cmd::CmdArgs;
use crate::executor::execute;
use clap::Parser;

#[tokio::main]
async fn main() {
    let cmd_args = CmdArgs::parse();
    execute(cmd_args);
}
