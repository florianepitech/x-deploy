use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CmdArgs {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Manage applications
    Application,
    /// Log in
    Auth(AuthArgs),
    /// Generate the autocompletion script for the specified shell
    Completion {
        // You can add parameters specific to each command here
    },
    /// Opens the application in Console in your browser
    Console,
    /// Manage containers
    Container,
    /// Manage CLI context
    Context,
    /// Manage cronjobs
    Cronjob,
    /// Manage databases
    Database,
    /// Manage Environment Variables and Secrets
    Env,
    /// Manage environments
    Environment,
    /// Manage lifecycle jobs
    Lifecycle,
    /// Print your application logs
    Log,
    /// Manage services
    Service,
    /// Connect to an application container
    Shell,
    /// Print the status of your application
    Status,
    /// Generate an API token
    Token,
    /// Upgrade CLI to latest version
    Upgrade,
    /// Print installed version of the CLI
    Version,
}

#[derive(Parser, Debug)]
pub(crate) struct AuthArgs {
    #[clap(short, long)]
    pub(crate) email: String,

    #[clap(short, long)]
    pub(crate) password: String,
}
