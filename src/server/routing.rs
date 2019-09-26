use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::helpers::{Coordinate, Preference};
use crate::lp::PreferenceEstimator;

use super::AppState;
use actix_web::web::Path;

#[derive(Deserialize)]
pub struct FspRequest {
    waypoints: Vec<Coordinate>,
    alpha: Preference,
}

#[derive(Serialize)]
pub struct PrefResponse {
    message: String,
    preference: Option<Vec<Preference>>,
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
                    let data = body.into_inner();

                    let path = state.graph.find_shortest_path(data.waypoints, data.alpha);
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
                Some(user) => HttpResponse::Ok().json(&user.alphas),
            }
        }
    }
}

pub fn set_preference(
    req: HttpRequest,
    body: web::Json<Vec<Preference>>,
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
                    let new_alphas = body.into_inner();
                    user.alphas = new_alphas;
                    HttpResponse::Ok().finish()
                }
            }
        }
    }
}

pub fn new_preference(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    user.add_pref();
                    HttpResponse::Ok().json(&user.alphas)
                }
            }
        }
    }
}

pub fn find_preference(
    req: HttpRequest,
    state: web::Data<AppState>,
    path_params: Path<usize>,
) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    let graph = &state.graph;
                    match user.current_route {
                        None => HttpResponse::Ok().json(PrefResponse {
                            message: String::from("You first have to set a route! Keeping old preference"),
                            preference: None,
                        }),
                        Some(ref route) => {
                            let index = path_params.into_inner();
                            let user_routes = &mut user.driven_routes;
                            user_routes[index].push(route.clone());

                            // Calculate new preference
                            let mut pref_estimator = PreferenceEstimator::new();
                            match pref_estimator.get_preference(
                                graph,
                                &user_routes[index],
                                user.alphas[index],
                            ) {
                                None => {
                                    user_routes[index].pop();
                                    HttpResponse::Ok().json(PrefResponse {
                                        message: String::from("No feasible preference found"),
                                        preference: None,
                                    })
                                }
                                Some(new_pref) => {
                                    user.alphas[index] = new_pref;
                                    HttpResponse::Ok().json(PrefResponse {
                                        message: String::new(),
                                        preference: Some(user.alphas.clone()),
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
