use crate::{error::AppError, types::AppResult};
use actix_files::NamedFile;
use actix_web::HttpRequest;
use std::path::PathBuf;

pub async fn index(_: HttpRequest) -> AppResult<NamedFile> {
    let path: PathBuf = match "./frontend/build/index.html".parse() {
        Ok(path) => path,
        Err(e) => {
            return Err(AppError::InternalError);
        }
    };
    match NamedFile::open(path) {
        Ok(file) => Ok(file),
        Err(e) => {
            // TODO ここだけはエラーページを返すべき
            return Err(AppError::InternalError);
        }
    }
}
