use std::fs::File;
use std::io::{Read, Write};
use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::dev::Service;
use actix_web::{web, App, HttpServer};
use futures::Future;

use crate::graph::Graph;
use crate::user::UserState;

// use actix_web::dev::{Service, ServiceResponse};
// use futures::{Future, IntoFuture};

mod auth;
mod routing;

pub struct AppState {
    graph: Graph,
    database_path: String,
    users: Mutex<Vec<UserState>>,
}

impl AppState {
    fn new(graph: Graph, database_path: String) -> Self {
        println!("Reading user database...");
        let users = match File::open(&database_path) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content)
                    .expect("Could not read file content");
                let users: Vec<UserState> =
                    serde_json::from_str(&content).expect("Could not deserialize file content");
                users
            }
            Err(_) => {
                println!("No database file existing");
                vec![UserState::new(
                    // test user
                    String::from("test"),
                    String::from("testtest"),
                )]
            }
        };
        AppState {
            graph,
            database_path,
            users: Mutex::new(users),
        }
    }

    fn write_to_file(&self) {
        let mut file = File::create(&self.database_path).expect("Could not create file");
        let buffer = serde_json::to_vec(&self.users).expect("Could not serialize state");
        file.write_all(&buffer)
            .expect("Could not write state to file");
    }
}

pub fn start_server(graph: Graph, port: String, database_path: String) {
    let state = web::Data::new(AppState::new(graph, database_path));
    println!("Starting server");
    HttpServer::new(move || {
        App::new()
            .register_data(state.clone())
            .wrap(Cors::new().allowed_origin("http://localhost:8080"))
            .wrap_fn(|req, srv| {
                srv.call(req).map(|res| {
                    let req = res.request();
                    let state: &AppState = req.app_data().unwrap();
                    state.write_to_file();
                    res
                })
            })
            /*
            .wrap_fn(|req, srv| {
                let unauth: Box<dyn IntoFuture<Item = ServiceResponse>> =
                    Box::new(ServiceResponse::new(
                        req.into_parts().0,
                        HttpResponse::Unauthorized().finish(),
                    ));
                let auth_header = req.headers().get("Authorization");
                match auth_header {
                    None => unauth,
                    Some(value) => {
                        let token = value.to_str().unwrap();
                        let mut users = state.users.lock().unwrap();
                        let user_state = users.iter_mut().find(|x| x.auth.token == token);
                        match user_state {
                            None => unauth,
                            Some(user) => Box::new(srv.call(req).map(|res| res)),
                        }
                    }
                }
            })
            */
            // routing stuff
            .route("/closest", web::get().to(routing::find_closest))
            .route("/fsp", web::post().to(routing::fsp))
            .route("/preference", web::get().to(routing::get_preference))
            .route("/preference", web::post().to(routing::set_preference))
            .route("/find_preference", web::post().to(routing::find_preference))
            .route("/routes", web::get().to(routing::get_routes))
            .route("/reset", web::post().to(routing::reset_data))

            // auth stuff
            .route("/login", web::post().to(auth::login))
            .route("/register", web::post().to(auth::register))
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect("Can not bind to port 8000")
    .run()
    .expect("Could not start sever");
}

#[cfg(test)]
mod tests {
    use super::*;
}
