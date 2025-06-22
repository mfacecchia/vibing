use keyring::Entry;

use crate::{env, error::Result};

// TODO: Get serivce from function argument
pub fn store_cred(user: &str, password: &str) -> Result<()> {
    let entry = Entry::new("vibing", user)?;
    entry.set_password(password).unwrap();
    println!("Credential stored successfully");
    Ok(())
}

pub fn get_cred(service: &str, user: &str) -> Result<String> {
    let entry = Entry::new(service, user)?;
    Ok(entry.get_password()?)
}

/// Uses default service and user to obtain authentication token from secure storage
pub fn get_cred_use_defaults() -> Result<String> {
    let service = env::get_env("CREDENTIAL_STORE_AUTH_SERVICE");
    let user = env::get_env("CREDENTIAL_STORE_AUTH_USER");
    get_cred(service.as_str(), user.as_str())
}

pub fn remove_cred(service: &str, user: &str) -> Result<()> {
    let entry = Entry::new(service, user)?;
    entry.delete_credential()?;
    Ok(())
}
