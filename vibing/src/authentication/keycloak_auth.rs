use reqwest::{
    self, Method,
    header::{HeaderMap, HeaderValue},
};
use std::time::Duration;
use tokio::time;

use crate::{
    authentication::{
        request::{self, KeycloakAuth, KeycloakDeviceCodeAuthCheck},
        response::{self, DeviceCodeAuth, KeycloakError, KeycloakSuccessfulAuthentication},
    },
    creds, env,
    error::{AppError, Result, build_generic_error},
    requests::{FetchOptions, fetch},
    utils::verbose_print,
};

// TODO: Add clientId and clientSecret here
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
        let res = self.generate_device_code(client_id, client_secret).await?;
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
                    // TODO: move this call out of here (separation of responsibility)
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
            return Err(AppError::KeycloakError(json_res));
        }
        let json_res = res.json::<KeycloakSuccessfulAuthentication>().await?;
        Ok(json_res)
    }

    pub async fn check_auth(
        &self,
        client_id: &str,
        client_secret: &str,
        token: &str,
    ) -> Result<bool> {
        let full_url = format!("{}/protocol/openid-connect/token/introspect", self.url);
        let req_body = request::KeycloakJwtIntrospect::new(client_id, client_secret, token);
        let client = reqwest::Client::new();
        let res = client
            .post(full_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&req_body)
            .send()
            .await?;
        let json_res = res.json::<response::KeycloakJwtActiveStatus>().await?;
        Ok(json_res.active)
    }

    pub async fn revoke_token(
        &self,
        client_id: &str,
        client_secret: &str,
        token: &str,
        verbose: bool,
    ) -> Result<()> {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::new();
        headers.append(
            "Content-type",
            HeaderValue::from_str("application/x-www-form-urlencoded")
                .map_err(|_| return AppError::Other(Box::new(build_generic_error(None))))?,
        );
        let req_body = request::KeycloakJwtIntrospect::new(client_id, client_secret, token);
        let mut fetch_options = FetchOptions::new(
            self.url.clone(),
            "/protocol/openid-connect/revoke".to_owned(),
            Method::POST,
            Some(headers),
            None,
            Some(req_body),
        );
        let _ = fetch::<request::KeycloakJwtIntrospect, ()>(&mut fetch_options, verbose).await?;
        Ok(())
    }
}
