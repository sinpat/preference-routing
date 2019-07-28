use std::sync::Mutex;

use actix_web::{App, HttpResponse, web, HttpServer};
use serde::{Deserialize, Serialize};

use crate::{EDGE_COST_DIMENSION, EDGE_COST_TAGS};
use crate::graph::dijkstra::DijkstraResult;
use crate::graph::Graph;
use crate::helpers::Coordinate;
use crate::lp::get_preference;

#[derive(Deserialize, Debug)]
struct FspRequest {
    // way_points: Vec<Coordinate>,
    source: Coordinate,
    target: Coordinate,
    avoid: Vec<Coordinate>,
}

#[derive(Serialize, Debug)]
struct FspResponse<'a> {
    path: DijkstraResult,
    alpha: [f64; EDGE_COST_DIMENSION],
    cost_tags: [&'a str; EDGE_COST_DIMENSION],
}

/*
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

fn find_closest(req: HttpRequest) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    req.json()
        .from_err()
        .and_then(move |point: Coordinate| {
            let (location, _) = req.state().graph.find_closest_node(&point);
            let response = HttpResponse::Ok().json(location);
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
*/

fn fsp(body: web::Json<FspRequest>, state: web::Data<AppState>) -> HttpResponse {
    let FspRequest {source, target, avoid} = body.into_inner();
    let graph = &state.graph;
    let alpha = *state.alpha.lock().unwrap();
    let result = graph.find_shortest_path(source, target, avoid, alpha);
    match result {
        None => HttpResponse::Ok().finish(),
        Some(path) => {
            /*
            *state.driven_routes.lock().unwrap() = vec![path];
            if let Some(new_pref) = get_preference(graph, &*state.driven_routes.lock().unwrap()) {
                *state.alpha.lock().unwrap() = new_pref;
            }
            */
            HttpResponse::Ok().json(FspResponse {
                path,
                alpha,
                cost_tags: EDGE_COST_TAGS,
            })
        }
    }
}

pub fn start_server(graph: Graph) {
    println!("Starting server");
    let state = web::Data::new(AppState {
        graph,
        driven_routes: Mutex::new(Vec::new()),
        alpha: Mutex::new([0.0, 1.0, 0.0])
    });
    HttpServer::new(move || {
        App::new()
            .register_data(state.clone())
            .service(
                web::scope("/routing")
                    .route("/fsp", web::post().to(fsp)))
    })
        .bind("localhost:8000")
        .expect("Can not bind to port 8000")
        .run();
}

struct AppState {
    graph: Graph,
    driven_routes: Mutex<Vec<DijkstraResult>>,
    alpha: Mutex<[f64; EDGE_COST_DIMENSION]>,
}

#[cfg(test)]
mod tests {
    use super::*;
}