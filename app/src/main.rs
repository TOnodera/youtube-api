use actix_files::{self as fs, NamedFile};
use actix_web::{web::Redirect, HttpRequest, Result};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use std::path::PathBuf;

mod env;
mod types;

async fn index(_: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./frontend/build/index.html".parse()?;
    Ok(NamedFile::open(path)?)
}

async fn oauth_authorization(_: HttpRequest) -> Result<Redirect> {
    let redirect_uri = env::get_env("YOUTUBE_API_REDIRECT_URI");
    let url = match redirect_uri {
        Ok(url) => url,
        Err(e) => {
            return Err(e.into());
        }
    };

    Ok(Redirect::to(url).permanent())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    ssl_builder.set_private_key_file("/workspaces/app/app/pem/key.pem", SslFiletype::PEM)?;
    ssl_builder.set_certificate_chain_file("/workspaces/app/app/pem/cert.pem")?;

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/oauth", web::get().to(oauth_authorization))
            .service(fs::Files::new("/", "./frontend/build").show_files_listing())
    })
    .bind_openssl(("127.0.0.1", 8080), ssl_builder)?
    .run()
    .await
}
