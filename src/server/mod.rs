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

mod routing;
mod user;

pub struct AppState {
    graph: Graph,
    users: Mutex<Vec<UserState>>,
}

pub fn start_server(graph: Graph) {
    println!("Reading user database...");
    let users = if let Some(state) = read_state_from_file() {
        state
    } else {
        vec![UserState::new(
            // test user
            String::from("test"),
            String::from("testtest"),
        )]
    };
    let state = web::Data::new(AppState {
        graph,
        users: Mutex::new(users),
    });
    println!("Starting server");
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().allowed_origin("http://localhost:8080"))
            .register_data(state.clone())
            .service(
                web::scope("/routing")
                    .wrap_fn(|req, srv| {
                        srv.call(req).map(|res| {
                            let req = res.request();
                            let state: &AppState = req.app_data().unwrap();
                            write_state_to_file(&state.users.lock().unwrap());
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
                    .route("/closest", web::get().to(routing::find_closest))
                    .route("/fsp", web::post().to(routing::fsp))
                    .route("/preference", web::get().to(routing::get_preference))
                    .route("/preference", web::post().to(routing::set_preference))
                    .route("/find_preference", web::post().to(routing::find_preference))
                    .route("/routes", web::get().to(routing::get_routes))
                    .route("/reset", web::post().to(routing::reset_data)),
            )
            .service(
                web::scope("/user")
                    .route("/login", web::post().to(user::login))
                    .route("/register", web::post().to(user::register)),
            )
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000")
    .run()
    .expect("Could not start sever");
}

fn read_state_from_file() -> Option<Vec<UserState>> {
    match File::open("database") {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .expect("Could not read file content");
            let users: Vec<UserState> =
                serde_json::from_str(&content).expect("Could not deserialize file content");
            Some(users)
        }
        Err(_) => {
            println!("No database file existing");
            None
        }
    }
}

fn write_state_to_file(state: &[UserState]) {
    let mut file = File::create("database").expect("Could not create file");
    let buffer = serde_json::to_vec(state).expect("Could not serialize state");
    file.write_all(&buffer)
        .expect("Could not write state to file");
}

#[cfg(test)]
mod tests {
    use super::*;
}
