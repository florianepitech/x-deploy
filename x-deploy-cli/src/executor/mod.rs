use std::process::exit;
use crate::auth::AuthFile;
use crate::cmd::{AuthArgs, CmdArgs, Commands};

pub(crate) fn execute(cmd_args: CmdArgs) {
    match cmd_args.command {
        Commands::Auth(args) => auth(args),
        _ => println!("Not implemented yet"),
    }
}

fn auth(args: AuthArgs) {
    if AuthFile::is_authenticated() {
        println!("ERROR: Already authenticated, please logout first");
        exit(1);
    }
    let credentials_str = format!("{}:{}", args.email, args.password);
    let credentials_base64 = base64::encode(&credentials_str.as_bytes());
    let auth_file = AuthFile::new(credentials_base64);
    auth_file.save_to_file();
    println!("INFO: Successfully authenticated")
}

