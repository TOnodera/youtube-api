use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};

pub async fn index(_: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./frontend/build/index.html".parse()?;
    Ok(NamedFile::open(path)?)
}
