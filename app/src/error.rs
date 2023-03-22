use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "サーバーエラーが発生しました。")]
    InternalError,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub body: String,
}
impl ErrorResponse {
    pub fn new(status: u16, body: String) -> Self {
        Self { status, body }
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ErrorResponse::new(
                self.status_code().as_u16(),
                self.to_string(),
            ))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
