mod login;
mod logout;
mod organization;

use crate::cmd::{CmdArgs, Commands};
use crate::error::CliResult;
use crate::executor::login::login;
use crate::executor::logout::logout;
use crate::executor::organization::organization;

pub(crate) async fn execute(cmd_args: CmdArgs) -> CliResult<String> {
  match cmd_args.command {
    Commands::Login(args) => login(args).await,
    Commands::Logout => logout(),
    Commands::Organization => organization().await,
  }
}
