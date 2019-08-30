use actix_web::{web, HttpResponse};
use serde::Serialize;

use crate::helpers::{Coordinate, Preference};
use crate::lp::PreferenceEstimator;

use super::AppState;
use super::INITIAL_PREF;

type FspRequest = Vec<Coordinate>;

#[derive(Serialize)]
pub struct PrefResponse<'a> {
    message: &'a str,
    preference: Option<Preference>,
}

pub fn find_closest(query: web::Query<Coordinate>, state: web::Data<AppState>) -> HttpResponse {
    let graph = &state.graph;
    let point = query.into_inner();

    let (location, _) = graph.find_closest_node(&point);
    HttpResponse::Ok().json(location)
}

pub fn fsp(body: web::Json<FspRequest>, state: web::Data<AppState>) -> HttpResponse {
    let waypoints = body.into_inner();
    let graph = &state.graph;
    let alpha = *state.alpha.lock().unwrap();

    let path = graph.find_shortest_path(waypoints, alpha);
    *state.current_route.lock().unwrap() = Some(path.clone());
    HttpResponse::Ok().json(path)
}

pub fn get_preference(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(&state.alpha)
}

pub fn set_preference(body: web::Json<Preference>, state: web::Data<AppState>) -> HttpResponse {
    let new_alpha = body.into_inner();
    *state.alpha.lock().unwrap() = new_alpha;
    HttpResponse::Ok().json(new_alpha)
}

pub fn find_preference(state: web::Data<AppState>) -> HttpResponse {
    let graph = &state.graph;
    let current_route = state.current_route.lock().unwrap();
    let mut alpha = state.alpha.lock().unwrap();
    match current_route.clone() {
        None => HttpResponse::Ok().json(PrefResponse {
            message: "You first have to set a route! Keeping old preference",
            preference: None,
        }),
        Some(route) => {
            let mut user_routes = state.driven_routes.lock().unwrap();
            user_routes.push(route);

            // Calculate new preference
            let mut pref_estimator = PreferenceEstimator::new();
            match pref_estimator.get_preference(graph, &user_routes, *alpha) {
                None => {
                    user_routes.pop();
                    HttpResponse::Ok().json(PrefResponse {
                        message: "No feasible preference found",
                        preference: None,
                    })
                }
                Some(new_pref) => {
                    *alpha = new_pref;
                    HttpResponse::Ok().json(PrefResponse {
                        message: "",
                        preference: Some(new_pref),
                    })
                }
            }
        }
    }
}

pub fn reset_data(state: web::Data<AppState>) -> HttpResponse {
    *state.driven_routes.lock().unwrap() = Vec::new();
    *state.current_route.lock().unwrap() = None;
    *state.alpha.lock().unwrap() = INITIAL_PREF;
    HttpResponse::Ok().finish()
}
