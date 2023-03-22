use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::result::Result;

// Result型
pub type AppResult<T> = Result<T, AppError>;

// Response Type
#[derive(Debug, Serialize, Deserialize)]
pub struct Success<T> {
    pub status: u16,
    pub body: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub scope: String,
}
impl Success<AccessToken> {
    pub fn new(status: u16, body: AccessToken) -> Self {
        Self { status, body }
    }
}
