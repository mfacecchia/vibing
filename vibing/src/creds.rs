use keyring::Entry;

use crate::error::Result;

// TODO: Get serivce from function argument
pub fn store_cred(user: &str, password: &str) -> Result<()> {
    let entry = Entry::new("vibing", user)?;
    entry.set_password(password).unwrap();
    println!("Credential stored successfully");
    Ok(())
}

pub fn get_cred(service: &str, user: &str) -> Result<String> {
    let entry = Entry::new(service, user)?;
    Ok(entry.get_password().unwrap())
}

pub fn remove_cred(service: &str, user: &str) -> Result<()> {
    let entry = Entry::new(service, user)?;
    entry.delete_credential()?;
    Ok(())
}
