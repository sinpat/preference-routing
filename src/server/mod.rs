use std::sync::Arc;

use actix_web::{App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse, server};
use actix_web::http::Method;
use actix_web::middleware::cors::Cors;
use futures::future::Future;
use serde::{Deserialize, Serialize};

use crate::{EDGE_COST_DIMENSION, EDGE_COST_TAGS};
use crate::graph::Graph;
use crate::helpers::Coordinate;

#[derive(Deserialize, Debug)]
struct FspRequest {
    source: Coordinate,
    target: Coordinate,
    include: Vec<Coordinate>,
    avoid: Vec<Coordinate>,
}

#[derive(Serialize, Debug)]
struct Path<'a> {
    waypoints: Vec<&'a Coordinate>,
    costs: [f64; EDGE_COST_DIMENSION],
    total_cost: f64,
    alpha: [f64; EDGE_COST_DIMENSION],
    cost_tags: [&'a str; EDGE_COST_DIMENSION],
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

fn verify_token(req: &HttpRequest) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
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

fn set_source(req: HttpRequest<Arc<AppState>>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    req.json()
        .from_err()
        .and_then(move |source: Coordinate| {
            let (location, _) = req.state().graph.find_closest_node(source);
            let response = HttpResponse::Ok().json(location);
            Ok(response)
        }).responder()
}

fn set_target(req: HttpRequest<Arc<AppState>>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    req.json()
        .from_err()
        .and_then(move |target: Coordinate| {
            let (location, _) = req.state().graph.find_closest_node(target);
            let response = HttpResponse::Ok().json(location);
            Ok(response)
        }).responder()
}

fn fsp(req: HttpRequest<Arc<AppState>>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    req.json()
        .from_err()
        .and_then(move |FspRequest { source, target, include, avoid }| {
            let graph = &req.state().graph;
            let alpha = req.state().alpha;
            let (_, source_id) = graph.find_closest_node(source);
            let (_, target_id) = graph.find_closest_node(target);
            let result = graph.find_shortest_path(source_id, target_id, include, avoid, alpha);
            match result {
                None => Ok(HttpResponse::Ok().finish()),
                Some((node_path, costs, total_cost)) => {
                    let waypoints = node_path
                        .iter()
                        .map(|node_id| &graph.nodes[*node_id].location)
                        .collect();
                    let body = Path {
                        waypoints,
                        costs,
                        total_cost,
                        alpha,
                        cost_tags: EDGE_COST_TAGS,
                    };
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