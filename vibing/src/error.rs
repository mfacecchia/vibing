use std::io::ErrorKind;

use crate::authentication::response::KeycloakError;

pub type Error = Box<dyn std::error::Error>;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    CredentialsNotFound(keyring::Error),
    AuthorizationError(Option<reqwest::Error>),
    NetworkError(Option<reqwest::Error>),
    FetchError(reqwest::Error),
    KeycloakError(KeycloakError),
    Other(Error),
}

impl From<keyring::Error> for AppError {
    fn from(err: keyring::Error) -> Self {
        AppError::CredentialsNotFound(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.status().is_none() {
            return AppError::FetchError(err);
        }
        let res_status = err.status().unwrap();
        if res_status.is_client_error() {
            return AppError::AuthorizationError(Some(err));
        }
        if res_status.is_server_error() {
            return AppError::NetworkError(Some(err));
        }
        AppError::FetchError(err)
    }
}

impl From<KeycloakError> for AppError {
    fn from(err: KeycloakError) -> Self {
        AppError::KeycloakError(err)
    }
}

impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        AppError::Other(err)
    }
}

pub fn build_generic_error(msg: Option<&str>) -> std::io::Error {
    let mut error_msg = "An error occured while processing your request...";
    if let Some(msg) = msg {
        error_msg = msg;
    }
    std::io::Error::new(ErrorKind::Other, error_msg)
}

pub fn handle(err: AppError) -> () {
    match err {
        AppError::AuthorizationError(_) => {
            println!("You are not authorized to execute this operation")
        }
        AppError::NetworkError(_) => {
            println!("The server is experiencing some issues at the moment.");
            println!("Please, try again in a few minutes.");
        }
        AppError::FetchError(_) => {
            println!("An unknown error occured while fetching data.");
            println!("Please, try again.");
        }
        AppError::CredentialsNotFound(_) => {
            println!("Credentials not found in the device's secure storage...");
            println!("Try to authenticate again using");
            println!("`vibes auth login`");
        }
        AppError::KeycloakError(_) => {
            println!("An error occured while checking your authentication token.");
            println!("Please, try again or, if the problem persists, try logging in again using");
            println!("`vibing auth login`");
        }
        AppError::Other(_) => {
            println!("An unknown error occured while satisfying your request...");
            println!(
                "Please, try again. If the problem persists, try to contact the technical support."
            );
        }
    }
}
