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
    pub refresh_token: String,
    pub scope: String,
}
impl AccessToken {
    pub fn new(
        access_token: String,
        token_type: String,
        expires_in: u32,
        refresh_token: String,
        scope: String,
    ) -> Self {
        Self {
            access_token,
            token_type,
            expires_in,
            refresh_token,
            scope,
        }
    }
}

impl Success<AccessToken> {
    pub fn new(status: u16, body: AccessToken) -> Self {
        Self { status, body }
    }
}
