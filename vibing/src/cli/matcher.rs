use crate::{
    authentication::keycloak_auth::KeycloakRequest,
    cli::args::{AuthArgs, BaseCommands, ClubArgs, VibingCliParser},
    creds, env,
    error::Result,
    features::club::club_requests::get_club_info,
    utils::verbose_print,
};
use clap::Parser;

pub async fn handle_cli_args() -> Result<()> {
    let args: VibingCliParser = VibingCliParser::parse();
    let verbose_flag = args.verbose;

    match args.command {
        BaseCommands::Auth(auth_commands) => match_auth_args(&auth_commands, verbose_flag).await?,
        BaseCommands::Club(club_args) => match_club_args(&club_args, verbose_flag).await?,
    }
    Ok(())
}

async fn match_auth_args(command: &AuthArgs, verbose: bool) -> Result<()> {
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
        AuthArgs::Logout => {
            verbose_print(verbose, "Logging you out...");
            let credential_store_service = env::get_env("CREDENTIAL_STORE_AUTH_SERVICE");
            let credential_store_user = env::get_env("CREDENTIAL_STORE_AUTH_USER");
            let access_token = creds::get_cred(
                credential_store_service.as_str(),
                credential_store_user.as_str(),
            )?;
            auth_request
                .revoke_token(
                    env::get_env("VIBING_DEVICE_CODE_CLIENT_ID").as_str(),
                    env::get_env("VIBING_CLIENT_SECRET").as_str(),
                    access_token.as_str(),
                    verbose,
                )
                .await?;
            println!("Successfully logged out!");
            verbose_print(verbose, "Removing credential...");
            creds::remove_cred(
                &credential_store_service.as_str(),
                credential_store_user.as_str(),
            )?;
            verbose_print(verbose, "Done!");
        }
        AuthArgs::Check => {
            verbose_print(verbose, "Checking authentication status...");
            // TODO: Handle NoEntry Creds error
            let access_token = creds::get_cred(
                env::get_env("CREDENTIAL_STORE_AUTH_SERVICE").as_str(),
                env::get_env("CREDENTIAL_STORE_AUTH_USER").as_str(),
            )?;
            let is_authenticated = auth_request
                .check_auth(
                    env::get_env("VIBING_DEVICE_CODE_CLIENT_ID").as_str(),
                    env::get_env("VIBING_CLIENT_SECRET").as_str(),
                    access_token.as_str(),
                )
                .await?;
            if !is_authenticated {
                println!(
                    "You are not authenticated or your token is expired, please run 'vibing auth login' to login again."
                );
                return Ok(());
            }
            println!("You are authenticated and ready to make requests!");
        }
    }
    Ok(())
}

async fn match_club_args(club_args: &ClubArgs, verbose_flag: bool) -> Result<()> {
    match club_args {
        ClubArgs::Get(club_get_args) => {
            get_club_info(club_get_args.club_id, verbose_flag).await?;
        }
    }
    Ok(())
}
