use crate::{
    authentication::keycloak_auth::KeycloakRequest,
    cli::args::{AuthArgs, BaseCommands, VibingCliParser},
    env,
    error::Result,
};
use clap::Parser;

pub async fn handle_cli_args() -> Result<()> {
    let args: VibingCliParser = VibingCliParser::parse();
    let verbose_flag = args.verbose;

    match args.command {
        BaseCommands::Auth(auth_commands) => match_auth_args(auth_commands, verbose_flag).await?,
    }
    Ok(())
}

async fn match_auth_args(command: AuthArgs, verbose: bool) -> Result<()> {
    let auth_request = KeycloakRequest::new();
    match command {
        AuthArgs::Login => {
            auth_request
                .authenticate(
                    env::get_env("VIBING_DEVICE_CODE_CLIENT_ID").as_str(),
                    env::get_env("VIBING_CLIENT_SECRET").as_str(),
                    verbose,
                )
                .await?;
        }
        AuthArgs::Logout => todo!(),
        AuthArgs::Check => todo!(),
    }
    Ok(())
}
