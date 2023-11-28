use crate::auth::AuthFile;
use std::process::exit;

pub fn logout() {
    if !AuthFile::is_authenticated() {
        println!("ERROR: Not authenticated, please login first");
        exit(1);
    }
    AuthFile::delete_file();
    println!("INFO: Successfully logged out")
}
