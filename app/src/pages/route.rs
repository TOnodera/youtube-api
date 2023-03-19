use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use super::callback::callback;
use super::index::index;
use super::oauth::oauth;

pub fn config(cfg: &mut web::ServiceConfig) {
    // topページ
    cfg.service(web::resource("/").route(web::get().to(index)));
    // oauthページ
    cfg.service(web::resource("/oauth").route(web::get().to(oauth)));
    // callbackページ（oauth認証後にリダイレクトされるページ）
    cfg.service(web::resource("/callback").route(web::get().to(callback)));
}
