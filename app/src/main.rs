use crate::pages::route::config;
use actix_files::{self as fs};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod env;
mod pages;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    ssl_builder.set_private_key_file(
        env::get_env("OPEN_SSL_KEY_FILE_PATH").unwrap(),
        SslFiletype::PEM,
    )?;
    ssl_builder.set_certificate_chain_file(env::get_env("OPEN_SSL_CERT_FILE_PATH").unwrap())?;

    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(fs::Files::new("/", "./frontend/build").show_files_listing())
    })
    .bind_openssl(("127.0.0.1", 8080), ssl_builder)?
    .run()
    .await
}
