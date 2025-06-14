use reqwest;
use std::{
    io,
    time::Duration,
};
use tokio::time;

use crate::{
    authentication::{
        request::{KeycloakAuth, KeycloakDeviceCodeAuthCheck},
        response::{DeviceCodeAuth, KeycloakError, KeycloakSuccessfulAuthentication},
    },
    creds, env,
    error::Result,
    utils::verbose_print,
};

pub struct KeycloakRequest {
    url: String,
}

impl KeycloakRequest {
    pub fn new() -> KeycloakRequest {
        let keycloak_host = env::get_env("KEYCLOAK_URL");
        let keycloak_realm = env::get_env("KEYCLOAK_REALM");
        return KeycloakRequest {
            url: format!("{keycloak_host}/realms/{keycloak_realm}"),
        };
    }

    pub async fn authenticate(
        &self,
        client_id: &str,
        client_secret: &str,
        verbose: bool,
    ) -> Result<()> {
        verbose_print(
            verbose,
            "Generating authentication _magic_ link... Please wait.",
        );
        let res = self
            .generate_device_code(client_id, client_secret)
            .await?;
        println!("Device code generated!");
        println!(
            "Please, open your browser on any of your devices and go to this link to continue."
        );
        println!("\t- {}", res.verification_uri_complete);
        println!("\t- {} minutes remaining", res.expires_in / 60);
        loop {
            time::sleep(Duration::from_secs(5)).await;
            let is_authenticated = self
                .check_authentication_status(res.device_code.as_str(), client_id, client_secret)
                .await;
            match is_authenticated {
                Ok(authentication) => {
                    verbose_print(verbose, "Authenticated!");
                    verbose_print(verbose, "Storing credential");
                    creds::store_cred("Vibing X Vibes - by Feis._.", &authentication.access_token)?;
                    break;
                }
                Err(_) => (),
            }
        }
        println!("Successfully authenticated! Now you can vibe ~");
        Ok(())
    }

    pub async fn generate_device_code(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<DeviceCodeAuth> {
        let full_url = format!("{}/protocol/openid-connect/auth/device", self.url);
        let keycloak_auth_req = KeycloakAuth::new(client_id, client_secret);
        let client = reqwest::Client::new();
        let res = client
            .post(full_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&keycloak_auth_req)
            .send()
            .await?;

        let json_res = res.json::<DeviceCodeAuth>().await?;
        Ok(json_res)
    }

    async fn check_authentication_status(
        &self,
        device_code: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Result<KeycloakSuccessfulAuthentication> {
        let full_url = format!("{}/protocol/openid-connect/token", self.url);
        let req_body = KeycloakDeviceCodeAuthCheck::new(
            device_code,
            "urn:ietf:params:oauth:grant-type:device_code",
            client_id,
            client_secret,
        );
        let client = reqwest::Client::new();
        let res = client
            .post(full_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&req_body)
            .send()
            .await?;
        if res.status().is_client_error() {
            let json_res = res.json::<KeycloakError>().await?;
            let error = Box::new(io::Error::new(
                std::io::ErrorKind::Other,
                format!("{}: {}", json_res.error, json_res.error_description),
            ));
            return Err(error);
        }
        let json_res = res.json::<KeycloakSuccessfulAuthentication>().await?;
        Ok(json_res)
    }
}

fn check_auth() -> bool {
    todo!("Get credential from storage and fetch backend");
}
