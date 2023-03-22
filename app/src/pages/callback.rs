use std::collections::HashMap;

use actix_web::{
    web::{self, Json},
    HttpRequest,
};
use percent_encoding::percent_decode_str;
use regex::Regex;
use reqwest::StatusCode;

use crate::{
    env,
    error::AppError,
    types::{AccessToken, AppResult, Success},
};

pub async fn callback(request: HttpRequest) -> AppResult<Json<Success<AccessToken>>> {
    // TODO actix-webのqueryの機能を使って取得できるように変更する

    // CODEからアクセストークンを取得する
    // トークンエンドポイント
    let url = "https://oauth2.googleapis.com/token";

    // CLIENT_ID
    let client_id = match env::get_env("YOUTUBE_API_CLIENT_ID") {
        Ok(client_id) => client_id,
        Err(e) => {
            return Err(AppError::InternalError);
        }
    };

    // CLIENT_SECRET
    let client_secret = match env::get_env("YOUTUBE_API_CLIENT_SECRET") {
        Ok(client_secret) => client_secret,
        Err(e) => {
            return Err(AppError::InternalError);
        }
    };

    // CODE
    let re = Regex::new(r"code=(?P<code>.*)&").unwrap();
    let caps = re.captures(request.query_string());
    let code = match caps {
        Some(caps) => {
            let code = match caps.name("code") {
                Some(code) => code.as_str().to_string(),
                None => {
                    return Err(AppError::InternalError);
                }
            };
            code.to_string()
        }
        None => {
            return Err(AppError::InternalError);
        }
    };

    // REDIRECT_URI
    let redirect_uri = env::get_env("YOUTUBE_API_REDIRECT_URI")?;

    // トークンエンドポイントに投げるパラメータを生成
    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert(
        "code",
        percent_decode_str(&code).decode_utf8().unwrap().to_string(),
    );
    params.insert("grant_type", "authorization_code".to_string());
    params.insert(
        "redirect_uri",
        percent_decode_str(&redirect_uri)
            .decode_utf8()
            .unwrap()
            .to_string(),
    );

    let client = reqwest::Client::new();
    let result = client.post(url).form(&params).send().await;
    let response = match result {
        Ok(response) => response,
        Err(e) => {
            return Err(AppError::InternalError);
        }
    };

    let access_token = match response.json::<AccessToken>().await {
        Ok(access_token) => access_token,
        Err(e) => {
            return Err(AppError::InternalError);
        }
    };

    Ok(web::Json(Success::new(
        StatusCode::OK.as_u16(),
        access_token,
    )))
}
