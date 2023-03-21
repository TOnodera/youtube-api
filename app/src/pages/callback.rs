use std::collections::HashMap;

use actix_web::{HttpRequest, HttpResponse, Result};
use percent_encoding::percent_decode_str;
use regex::Regex;

use crate::env;

pub async fn callback(request: HttpRequest) -> Result<HttpResponse> {
    // TODO actix-webのqueryの機能を使って取得できるように変更する

    // CODEからアクセストークンを取得する
    // トークンエンドポイント
    let url = "https://oauth2.googleapis.com/token";

    // CLIENT_ID
    let client_id = env::get_env("YOUTUBE_API_CLIENT_ID")?;

    // CLIENT_SECRET
    let client_secret = env::get_env("YOUTUBE_API_CLIENT_SECRET")?;

    // CODE
    let re = Regex::new(r"code=(?P<code>.*)&").unwrap();
    let caps = re.captures(request.query_string());
    let code = match caps {
        Some(caps) => {
            let code = match caps.name("code") {
                Some(code) => code.as_str().to_string(),
                None => {
                    return Ok(HttpResponse::Ok().body("code not found"));
                }
            };
            code.to_string()
        }
        None => {
            return Ok(HttpResponse::Ok().body("code not found"));
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
            return Ok(HttpResponse::Ok().body(format!("error: {}", e)));
        }
    };
    let text = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            return Ok(HttpResponse::Ok().body(format!("error: {}", e)));
        }
    };
    Ok(HttpResponse::Ok().body(text))
}
