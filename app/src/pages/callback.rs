use actix_web::{HttpRequest, HttpResponse, Result};

pub async fn callback(request: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", request);
    Ok(HttpResponse::Ok().body("callback"))
}
