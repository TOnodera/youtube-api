use crate::pages::route::config;
use actix_web::{App, HttpServer};
use util::make_ssl_builder;

mod env;
mod error;
mod pages;
mod types;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // SSL 設定
    let ssl_builder = make_ssl_builder()?;
    // アプリケーション設定
    HttpServer::new(|| App::new().configure(config))
        .bind_openssl(("127.0.0.1", 8080), ssl_builder)?
        .run()
        .await
}
