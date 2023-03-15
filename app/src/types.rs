use std::result::Result;

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;
