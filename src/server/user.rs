use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

pub fn login(body: web::Json<Credentials>) -> HttpResponse {
    HttpResponse::Ok().json("token")
}

pub fn register(body: web::Json<Credentials>) -> HttpResponse {
    HttpResponse::Ok().json("token")
}
