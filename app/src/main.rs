use crate::pages::route::config;
use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod env;
mod pages;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // SSL設定
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    ssl_builder.set_private_key_file(
        env::get_env("OPEN_SSL_KEY_FILE_PATH").unwrap(),
        SslFiletype::PEM,
    )?;
    ssl_builder.set_certificate_chain_file(env::get_env("OPEN_SSL_CERT_FILE_PATH").unwrap())?;

    // アプリケーション設定
    HttpServer::new(|| App::new().configure(config))
        .bind_openssl(("127.0.0.1", 8080), ssl_builder)?
        .run()
        .await
}
