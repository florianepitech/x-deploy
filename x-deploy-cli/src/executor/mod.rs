mod login;
mod logout;

use crate::cmd::{CmdArgs, Commands};
use crate::executor::login::login;
use crate::executor::logout::logout;

pub(crate) fn execute(cmd_args: CmdArgs) {
  match cmd_args.command {
    Commands::Login(args) => login(args),
    Commands::Logout => logout(),
    _ => println!("Not implemented yet"),
  }
}
