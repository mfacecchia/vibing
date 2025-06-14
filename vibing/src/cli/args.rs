use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct VibingCliParser {
    #[command(subcommand)]
    pub command: BaseCommands,

    /// Print out the trace from all the methods called
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum BaseCommands {
    /// Authenticate to the application (required for most of the operations)
    #[command(subcommand)]
    Auth(AuthArgs),
}

#[derive(Subcommand, Debug)]
pub enum AuthArgs {
    /// Initiate the login process
    Login,
    /// Logout from the current logger in account
    Logout,
    /// Check your current authentication status
    Check,
}
