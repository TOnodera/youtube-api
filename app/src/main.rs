use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};

use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./dist/index.html".parse()?;
    Ok(NamedFile::open(path)?)
}

async fn return_url(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./dist/return.html".parse()?;
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/return", web::get().to(return_url))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
