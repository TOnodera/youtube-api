use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use crate::{env, error::AppError, types::AppResult};

fn get_open_ssl_key_file_path() -> std::io::Result<String> {
    let open_ssl_key_file_path = match env::get_env("OPEN_SSL_KEY_FILE_PATH") {
        Ok(value) => value,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "OPEN_SSL_KEY_FILE_PATH is not set.",
            ));
        }
    };
    Ok(open_ssl_key_file_path)
}

fn get_open_ssl_cert_file_path() -> std::io::Result<String> {
    let open_ssl_cert_file_path = match env::get_env("OPEN_SSL_CERT_FILE_PATH") {
        Ok(value) => value,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "OPEN_SSL_CERT_FILE_PATH is not set.",
            ));
        }
    };
    Ok(open_ssl_cert_file_path)
}

pub fn make_ssl_builder() -> std::io::Result<SslAcceptorBuilder> {
    let open_ssl_key_file_path = get_open_ssl_key_file_path()?;
    let open_ssl_cert_file_path = get_open_ssl_cert_file_path()?;

    // SSL設定
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    ssl_builder.set_private_key_file(open_ssl_key_file_path, SslFiletype::PEM)?;
    ssl_builder.set_certificate_chain_file(open_ssl_cert_file_path)?;
    Ok(ssl_builder)
}

pub async fn get_redirect_uri() -> AppResult<String> {
    let redirect_uri_env = env::get_env("YOUTUBE_API_REDIRECT_URI");
    let redirect_uri = match redirect_uri_env {
        Ok(redirect_uri) => redirect_uri,
        Err(e) => {
            return Err(AppError::InternalError);
        }
    };
    Ok(redirect_uri)
}

pub async fn get_client_id() -> AppResult<String> {
    let client_id_env = env::get_env("YOUTUBE_API_CLIENT_ID");
    let client_id = match client_id_env {
        Ok(client_id) => client_id,
        Err(e) => {
            return Err(AppError::InternalError);
        }
    };
    Ok(client_id)
}

pub async fn get_client_secret() -> AppResult<String> {
    let client_secret_env = env::get_env("YOUTUBE_API_CLIENT_SECRET");
    let client_secret = match client_secret_env {
        Ok(client_secret) => client_secret,
        Err(e) => {
            return Err(AppError::InternalError);
        }
    };
    Ok(client_secret)
}
