use dotenv::dotenv;
use std::env;

use crate::types::AppResult;

pub fn get_env(key: &str) -> AppResult<String> {
    dotenv().ok();
    match env::var(key.to_string()) {
        Ok(value) => Ok(value),
        Err(e) => Err(Box::new(e)),
    }
}
