use actix_web::{web, HttpRequest, HttpResponse};
use serde::Serialize;

use crate::helpers::{Coordinate, Preference};
use crate::lp::PreferenceEstimator;

use super::AppState;

type FspRequest = Vec<Coordinate>;

#[derive(Serialize)]
pub struct PrefResponse<'a> {
    message: &'a str,
    preference: Option<Preference>,
}

pub fn find_closest(query: web::Query<Coordinate>, state: web::Data<AppState>) -> HttpResponse {
    let graph = &state.graph;
    let coordinate = query.into_inner();

    let location = &graph.find_closest_node(&coordinate).location;
    HttpResponse::Ok().json(location)
}

pub fn fsp(
    req: HttpRequest,
    body: web::Json<FspRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    let waypoints = body.into_inner();
                    let alpha = user.alpha;

                    let path = state.graph.find_shortest_path(waypoints, alpha);
                    user.current_route = Some(path.clone());
                    HttpResponse::Ok().json(path)
                }
            }
        }
    }
}

pub fn get_preference(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => HttpResponse::Ok().json(user.alpha),
            }
        }
    }
}

pub fn set_preference(
    req: HttpRequest,
    body: web::Json<Preference>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    let new_alpha = body.into_inner();
                    user.alpha = new_alpha;
                    HttpResponse::Ok().json(new_alpha)
                }
            }
        }
    }
}

pub fn find_preference(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    let graph = &state.graph;
                    match user.current_route.clone() {
                        None => HttpResponse::Ok().json(PrefResponse {
                            message: "You first have to set a route! Keeping old preference",
                            preference: None,
                        }),
                        Some(route) => {
                            let user_routes = &mut user.driven_routes;
                            user_routes.push(route);

                            // Calculate new preference
                            let mut pref_estimator = PreferenceEstimator::new();
                            match pref_estimator.get_preference(graph, user_routes, user.alpha) {
                                None => {
                                    user_routes.pop();
                                    HttpResponse::Ok().json(PrefResponse {
                                        message: "No feasible preference found",
                                        preference: None,
                                    })
                                }
                                Some(new_pref) => {
                                    user.alpha = new_pref;
                                    HttpResponse::Ok().json(PrefResponse {
                                        message: "",
                                        preference: Some(new_pref),
                                    })
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn get_routes(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let users = state.users.lock().unwrap();
            let user_state = users.iter().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    let routes = &user.driven_routes;
                    HttpResponse::Ok().json(routes)
                }
            }
        }
    }
}

pub fn reset_data(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    user.reset();
                    HttpResponse::Ok().finish()
                }
            }
        }
    }
}

fn extract_token(req: &HttpRequest) -> Option<&str> {
    let auth_header = req.headers().get("Authorization");
    match auth_header {
        None => None,
        Some(value) => Some(value.to_str().unwrap()),
    }
}
