use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Serialize;

use crate::graph::dijkstra::DijkstraResult;
use crate::graph::Graph;
use crate::helpers::{Coordinate, Preference};
use crate::lp::PreferenceEstimator;

type FspRequest = Vec<Coordinate>;

#[derive(Serialize, Debug)]
struct FspResponse {
    path: DijkstraResult,
    alpha: Preference,
}

fn find_closest(query: web::Query<Coordinate>, state: web::Data<AppState>) -> HttpResponse {
    let graph = &state.graph;
    let point = query.into_inner();

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
            HttpResponse::Ok().json(FspResponse { path, alpha })
        }
    }
}

fn get_preference(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(&state.alpha)
}

fn set_preference(body: web::Json<Preference>, state: web::Data<AppState>) -> HttpResponse {
    let new_alpha = body.into_inner();
    *state.alpha.lock().unwrap() = new_alpha;
    HttpResponse::Ok().json(new_alpha)
}

fn calc_preference(state: web::Data<AppState>) -> HttpResponse {
    let graph = &state.graph;
    let mut current_route = state.current_route.lock().unwrap();
    let mut user_routes = state.driven_routes.lock().unwrap();
    let mut alpha = state.alpha.lock().unwrap();
    user_routes.push(current_route.clone());
    *current_route = DijkstraResult::new();

    let mut pref_estimator = PreferenceEstimator::new();
    match pref_estimator.get_preference(graph, &*user_routes, *alpha) {
        Some(new_pref) => {
            *alpha = new_pref;
            HttpResponse::Ok().json(new_pref)
        }
        None => HttpResponse::Ok().json([0.0; 0]),
    }
}

fn reset_data(state: web::Data<AppState>) -> HttpResponse {
    *state.driven_routes.lock().unwrap() = Vec::new();
    *state.current_route.lock().unwrap() = DijkstraResult::new();
    *state.alpha.lock().unwrap() = [0.0, 1.0, 0.0];
    HttpResponse::Ok().finish()
}

pub fn start_server(graph: Graph) {
    println!("Starting server");
    let state = web::Data::new(AppState {
        graph,
        driven_routes: Mutex::new(Vec::new()),
        current_route: Mutex::new(DijkstraResult::new()),
        alpha: Mutex::new([0.0, 1.0, 0.0]),
    });
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().allowed_origin("http://localhost:8080"))
            .register_data(state.clone())
            .service(
                web::scope("/routing")
                    .route("/closest", web::get().to(find_closest))
                    .route("/fsp", web::post().to(fsp))
                    .route("/preference", web::get().to(get_preference))
                    .route("/preference", web::post().to(set_preference))
                    .route("/calc_preference", web::post().to(calc_preference))
                    .route("/reset", web::post().to(reset_data)),
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
    alpha: Mutex<Preference>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
