cargo build --bin x-deploy-cli
export RUST_BACKTRACE=1 && ./target/debug/x-deploy-cli logout
export RUST_BACKTRACE=1 && ./target/debug/x-deploy-cli login credentials --email bastien.cantet@gmail.com --password password
