mod auth;
mod cmd;

use clap::Parser;
use crate::cmd::CmdArgs;
use crate::cmd::Commands::Auth;

fn main() {
    let cmd_args = CmdArgs::parse();
    // get AuthArgs if command is auth
    if let Auth(auth_args) = cmd_args.command {
        println!("Login with credentials for email {}...", auth_args.email);
    }
}
