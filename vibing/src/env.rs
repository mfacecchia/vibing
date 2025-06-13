use dotenv::dotenv;
use std::{env};

pub fn load() -> bool {
    dotenv().is_ok()
}

pub fn get_env(key: &str) -> String {
   let value = env::var(key);
   // FIXME: Don't panic, gracefully return a Result<>
   match value {
    Ok(value) => value.to_owned(),
    Err(err) => panic!("Could not retrieve environment variable.\nReason: {err}")
   }
}
