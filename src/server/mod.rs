use std::sync::Arc;

use actix_web::{App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse, server};
use actix_web::http::Method;
use actix_web::middleware::cors::Cors;
use futures::future::Future;
use serde::{ Serialize, Deserialize };

use crate::graph::Graph;

#[derive(Serialize, Deserialize, Debug)]
struct Coordinate {
    lat: f64,
    lng: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct FspRequest {
    source: Coordinate,
    target: Coordinate
}

fn handle_register(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .body("register user")
}

fn handle_login(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .header("Authorization", "REPLACE ME")
        .finish()
}

fn set_source(req: &HttpRequest<Arc<Graph>>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
        .from_err()
        .and_then(|source: Coordinate| {
            let response = HttpResponse::Ok().json(source);
            Ok(response)
        }).responder()
}

fn set_target(req: &HttpRequest<Arc<Graph>>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
        .from_err()
        .and_then(|target: Coordinate| {
            let response = HttpResponse::Ok().json(target);
            Ok(response)
        }).responder()
}

fn handle_fsp(req: &HttpRequest<Arc<Graph>>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
        .from_err()
        .and_then(|FspRequest { source, target }| {
            let response = HttpResponse::Ok().json(vec![source, target]);
            Ok(response)
        }).responder()
}

pub fn start_server(graph: Graph) {
    println!("Starting server");
    let gr = Arc::new(graph);
    server::new(move || {
        vec![
            App::new()
                .prefix("/user")
                .configure(|app| Cors::for_app(app)
                    .resource("/register", |r| r.method(Method::POST).f(handle_register))
                    .resource("/login", |r| r.method(Method::POST).f(handle_login))
                    .register())
                .boxed(),
            App::with_state(gr.clone())
                .prefix("/routing")
                .configure(|app| Cors::for_app(app)
                    .resource("/source", |r| r.method(Method::POST).f(set_source))
                    .resource("/target", |r| r.method(Method::POST).f(set_target))
                    .resource("/fsp", |r| r.method(Method::POST).f(handle_fsp))
                    .register())
                .boxed()
        ]
    })
        .bind("localhost:8000")
        .expect("Can not bind to port 8000")
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
}