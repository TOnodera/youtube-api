use actix_web::{web::Redirect, HttpRequest, Result};

use crate::env;

pub async fn oauth(_: HttpRequest) -> Result<Redirect> {
    // URL
    let redirect_uri_env = env::get_env("YOUTUBE_API_REDIRECT_URI");
    let redirect_uri = match redirect_uri_env {
        Ok(redirect_uri) => redirect_uri,
        Err(e) => {
            return Err(e.into());
        }
    };

    // CLIENT_ID
    let client_id_env = env::get_env("YOUTUBE_API_CLIENT_ID");
    let client_id = match client_id_env {
        Ok(client_id) => client_id,
        Err(e) => {
            return Err(e.into());
        }
    };

    // RESPONSE_TYPE
    let response_type = "code";

    // SCOPE
    let scope = "https://www.googleapis.com/auth/youtube.readonly";

    // ACCESS_TYPE
    let access_type = "online";

    // STATE
    let state = "random_string";

    let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?redirect_uri={}&client_id={}&response_type={}&scope={}&access_type={}&state={}",
        redirect_uri, client_id, response_type, scope, access_type, state
    );

    Ok(Redirect::to(url).permanent())
}
