use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, web};
use serde::Serialize;

use crate::{EDGE_COST_DIMENSION, EDGE_COST_TAGS};
use crate::graph::dijkstra::DijkstraResult;
use crate::graph::Graph;
use crate::helpers::Coordinate;
use crate::lp::get_preference;

type FspRequest = Vec<Coordinate>;

#[derive(Serialize, Debug)]
struct FspResponse<'a> {
    path: DijkstraResult,
    alpha: [f64; EDGE_COST_DIMENSION],
    cost_tags: [&'a str; EDGE_COST_DIMENSION],
}

fn find_closest(body: web::Json<Coordinate>, state: web::Data<AppState>) -> HttpResponse {
    let graph = &state.graph;
    let point = body.into_inner();

    let (location, _) = graph.find_closest_node(&point);
    HttpResponse::Ok().json(location)
}

fn fsp(body: web::Json<FspRequest>, state: web::Data<AppState>) -> HttpResponse {
    let waypoints = body.into_inner();
    let graph = &state.graph;
    let alpha = *state.alpha.lock().unwrap();

    match graph.find_shortest_path(waypoints, alpha) {
        None => HttpResponse::Ok().finish(),
        Some(path) => {
            *state.current_route.lock().unwrap() = path.clone();
            HttpResponse::Ok().json(FspResponse {
                path,
                alpha,
                cost_tags: EDGE_COST_TAGS,
            })
        }
    }
}

fn calc_preference(state: web::Data<AppState>) -> HttpResponse {
    let graph = &state.graph;
    let mut current_route = state.current_route.lock().unwrap();
    let mut user_routes = state.driven_routes.lock().unwrap();

    user_routes.push(current_route.clone());
    *current_route = DijkstraResult::new();
    match get_preference(graph, &*user_routes) {
        Some(new_pref) => {
            *state.alpha.lock().unwrap() = new_pref;
            HttpResponse::Ok().json(new_pref)
        },
        None => HttpResponse::Ok().finish()
    }
}

pub fn start_server(graph: Graph) {
    println!("Starting server");
    let state = web::Data::new(AppState {
        graph,
        driven_routes: Mutex::new(Vec::new()),
        current_route: Mutex::new(DijkstraResult::new()),
        alpha: Mutex::new([1.0, 0.0, 0.0]),
    });
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new()
                .allowed_origin("http://localhost:8080"))
            .register_data(state.clone())
            .service(
                web::scope("/routing")
                    .route("/closest", web::post().to(find_closest))
                    .route("/fsp", web::post().to(fsp))
                    .route("/preference", web::post().to(calc_preference))
            )
    })
        .bind("localhost:8000")
        .expect("Can not bind to port 8000")
        .run()
        .expect("Could not start sever");
}

struct AppState {
    graph: Graph,
    driven_routes: Mutex<Vec<DijkstraResult>>,
    current_route: Mutex<DijkstraResult>,
    alpha: Mutex<[f64; EDGE_COST_DIMENSION]>,
}

#[cfg(test)]
mod tests {
    use super::*;
}