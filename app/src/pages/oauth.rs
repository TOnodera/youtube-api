use crate::{
    types::AppResult,
    util::{get_client_id, get_redirect_uri},
};
use actix_web::{web::Redirect, HttpRequest};

pub async fn oauth(_: HttpRequest) -> AppResult<Redirect> {
    // URL
    let redirect_uri = get_redirect_uri().await?;

    // CLIENT_ID
    let client_id = get_client_id().await?;

    // RESPONSE_TYPE
    let response_type = "code";

    // SCOPE
    let scope = "https://www.googleapis.com/auth/youtube.readonly";

    // ACCESS_TYPE
    let access_type = "online";

    // STATE
    let state = "random_string";

    // URL生成
    let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?redirect_uri={}&client_id={}&response_type={}&scope={}&access_type={}&state={}",
        redirect_uri, client_id, response_type, scope, access_type, state
    );

    Ok(Redirect::to(url).permanent())
}
