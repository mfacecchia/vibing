use serde::Serialize;

#[derive(Serialize)]
pub struct KeycloakAuth {
    pub client_id: String,
    pub client_secret: String,
}

impl KeycloakAuth {
    pub fn new(client_id: &str, client_secret: &str) -> KeycloakAuth {
        KeycloakAuth {
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
        }
    }
}

#[derive(Serialize)]
pub struct KeycloakDeviceCodeAuthCheck {
    pub device_code: String,
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
}

impl KeycloakDeviceCodeAuthCheck {
    pub fn new(
        device_code: &str,
        grant_type: &str,
        client_id: &str,
        client_secret: &str,
    ) -> KeycloakDeviceCodeAuthCheck {
        KeycloakDeviceCodeAuthCheck {
            device_code: device_code.to_owned(),
            grant_type: grant_type.to_owned(),
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
        }
    }
}
