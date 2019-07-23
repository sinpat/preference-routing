use std::sync::Arc;

use actix_web::{App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse, server};
use actix_web::http::Method;
use actix_web::middleware::cors::Cors;
use futures::future::Future;
use serde::{Deserialize, Serialize};

use crate::EDGE_COST_DIMENSION;
use crate::graph::dijkstra::Dijkstra;
use crate::graph::Graph;
use crate::helpers::Coordinate;

#[derive(Serialize, Debug)]
struct HalfNode<'a> {
    location: &'a Coordinate,
    node_id: usize,
}

#[derive(Deserialize, Debug)]
struct FspRequest {
    source: usize,
    target: usize,
}

#[derive(Serialize, Debug)]
struct FspResult<'a> {
    path: Vec<&'a Coordinate>,
    cost: f64,
}

fn register(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .body("register user")
}

fn login(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .header("Authorization", "foobar")
        .finish()
}

fn verify_token(req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.json()
        .from_err()
        .and_then(|token: String| {
            let response = if token == "foobar" {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::Unauthorized().finish()
            };
            Ok(response)
        }).responder()
}

fn set_source(req: HttpRequest<Arc<AppState>>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.json()
        .from_err()
        .and_then(move |source: Coordinate| {
            let (location, node_id) = req.state().graph.find_closest_node(source);
            let response = HttpResponse::Ok().json(HalfNode { location, node_id });
            Ok(response)
        }).responder()
}

fn set_target(req: HttpRequest<Arc<AppState>>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.json()
        .from_err()
        .and_then(move |target: Coordinate| {
            let (location, node_id) = req.state().graph.find_closest_node(target);
            let response = HttpResponse::Ok().json(HalfNode { location, node_id });
            Ok(response)
        }).responder()
}

fn fsp(req: HttpRequest<Arc<AppState>>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.json()
        .from_err()
        .and_then(move |FspRequest { source, target }| {
            let state = req.state();
            let mut dijkstra = Dijkstra::new(&state.graph);
            let result = dijkstra.find_shortest_path(source, target, state.alpha);
            match result {
                None => Ok(HttpResponse::Ok().finish()),
                Some((node_path, cost)) => {
                    let path = node_path
                        .iter()
                        .map(|node_id| &state.graph.nodes[*node_id].location)
                        .collect();
                    let body = FspResult { path, cost: cost.0 };
                    Ok(HttpResponse::Ok().json(body))
                }
            }
        }).responder()
}

pub fn start_server(graph: Graph) {
    println!("Starting server");
    let state = Arc::new(AppState {
        graph,
        // unsuitability, dist, height
        alpha: [0.0, 1.0, 0.0],
    });
    server::new(move || {
        vec![
            App::new()
                .prefix("/user")
                .configure(|app| Cors::for_app(app)
                    .resource("/register", |r| r.method(Method::POST).f(register))
                    .resource("/login", |r| r.method(Method::POST).f(login))
                    .resource("/verify", |r| r.method(Method::POST).f(verify_token))
                    .register())
                .boxed(),
            App::with_state(state.clone())
                .prefix("/routing")
                .configure(|app| Cors::for_app(app)
                    .resource("/source", |r| r.method(Method::POST).with(set_source))
                    .resource("/target", |r| r.method(Method::POST).with(set_target))
                    .resource("/fsp", |r| r.method(Method::POST).with(fsp))
                    .register())
                .boxed()
        ]
    })
        .bind("localhost:8000")
        .expect("Can not bind to port 8000")
        .run();
}

struct AppState {
    graph: Graph,
    alpha: [f64; EDGE_COST_DIMENSION],
}

#[cfg(test)]
mod tests {
    use super::*;
}