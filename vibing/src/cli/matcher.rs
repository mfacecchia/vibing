use clap::Parser;
use crate::{authentication::auth::KeycloakRequest, cli::args::{AuthArgs, BaseCommands, VibingCliParser}, env};

pub async fn handle_cli_args() -> () {
    let args: VibingCliParser = VibingCliParser::parse();
    let verbose_flag = args.verbose;

    match args.command {
        BaseCommands::Auth(auth_commands) => match_auth_args(auth_commands, verbose_flag).await,
    }
}

async fn match_auth_args (command: AuthArgs, verbose: bool) -> () {
    match command {
        AuthArgs::Login => {
            todo!();
        }
        AuthArgs::Logout => todo!(),
        AuthArgs::Check => todo!(),
    }
}
