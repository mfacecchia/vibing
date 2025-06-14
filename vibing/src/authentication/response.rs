use serde::{Deserialize};

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct DeviceCodeAuth {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: i32,
    pub interval: i32,
}

#[derive(Deserialize, Debug)]
pub struct KeycloakAccountRoles {
    pub roles: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct KeycloakJwtAccount {
    pub account: KeycloakAccountRoles,
}

#[derive(Deserialize, Debug)]
pub struct KeycloakJwtIntrospect {
    pub exp: i32,
    pub iat: i32,
    pub auth_time: i32,
    pub jti: String,
    pub iss: String,
    pub sub: String,
    pub typ: String,
    pub resource_access: KeycloakJwtAccount,
    pub scope: String,
    pub email_verified: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
    pub client_id: String,
    pub username: String,
    pub token_type: String,
    pub active: bool,
}

#[derive(Deserialize, Debug)]
pub struct KeycloakJwtActiveStatus {
    pub active: bool
}
