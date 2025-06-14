use serde::Deserialize;

#[derive(Deserialize)]
pub struct KeycloakError {
    pub error: String,
    pub error_description: String,
}

#[derive(Deserialize, Debug)]
pub struct KeycloakSuccessfulAuthentication {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
    pub session_state: String,
    pub scope: String,
}

#[derive(Deserialize)]
pub struct DeviceCodeAuth {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: i32,
    pub interval: i32,
}
