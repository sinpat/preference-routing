use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Serialize;

use crate::graph::Graph;
use crate::graph::Path;
use crate::helpers::{Coordinate, Preference};
use crate::lp::PreferenceEstimator;
use crate::EDGE_COST_DIMENSION;

type FspRequest = Vec<Coordinate>;

const INITIAL_PREF: Preference = [0.0, 1.0, 0.0];

#[derive(Serialize)]
struct PrefResponse<'a> {
    message: &'a str,
    preference: Preference,
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

    let path = graph.find_shortest_path(waypoints, alpha);
    *state.current_route.lock().unwrap() = Some(path.clone());
    HttpResponse::Ok().json(path)
}

fn get_preference(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(&state.alpha)
}

fn set_preference(body: web::Json<Preference>, state: web::Data<AppState>) -> HttpResponse {
    let new_alpha = body.into_inner();
    *state.alpha.lock().unwrap() = new_alpha;
    HttpResponse::Ok().json(new_alpha)
}

fn find_preference(state: web::Data<AppState>) -> HttpResponse {
    let graph = &state.graph;
    let current_route = state.current_route.lock().unwrap();
    let mut alpha = state.alpha.lock().unwrap();
    match current_route.clone() {
        None => HttpResponse::Ok().json(PrefResponse {
            message: "You first have to set a route! Keeping old preference",
            preference: *alpha,
        }),
        Some(route) => {
            let mut user_routes = state.driven_routes.lock().unwrap();
            // TODO: Do not push route prematurely, or pop it when no pref found?
            user_routes.push(route);

            // Calculate new preference
            let mut pref_estimator = PreferenceEstimator::new();
            match pref_estimator.get_preference(graph, &user_routes, *alpha) {
                None => {
                    HttpResponse::Ok().json(PrefResponse {
                        message: "No feasible preference found",
                        preference: [0.0; EDGE_COST_DIMENSION],
                    })
                },
                Some(new_pref) => {
                    *alpha = new_pref;
                    HttpResponse::Ok().json(PrefResponse {
                        message: "",
                        preference: new_pref,
                    })
                }
            }
        }
    }
}

fn reset_data(state: web::Data<AppState>) -> HttpResponse {
    *state.driven_routes.lock().unwrap() = Vec::new();
    *state.current_route.lock().unwrap() = None;
    *state.alpha.lock().unwrap() = INITIAL_PREF;
    HttpResponse::Ok().finish()
}

pub fn start_server(graph: Graph) {
    println!("Starting server");
    let state = web::Data::new(AppState {
        graph,
        driven_routes: Mutex::new(Vec::new()),
        current_route: Mutex::new(None),
        alpha: Mutex::new(INITIAL_PREF),
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
                    .route("/find_preference", web::post().to(find_preference))
                    .route("/reset", web::post().to(reset_data)),
            )
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000")
    .run()
    .expect("Could not start sever");
}

struct AppState {
    graph: Graph,
    driven_routes: Mutex<Vec<Path>>,
    current_route: Mutex<Option<Path>>,
    alpha: Mutex<Preference>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
