use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use crate::graph::Graph;
use crate::graph::Path;
use crate::helpers::Preference;

const INITIAL_PREF: Preference = [0.0, 1.0, 0.0];

mod routing;
mod user;

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
                    .route("/closest", web::get().to(routing::find_closest))
                    .route("/fsp", web::post().to(routing::fsp))
                    .route("/preference", web::get().to(routing::get_preference))
                    .route("/preference", web::post().to(routing::set_preference))
                    .route("/find_preference", web::post().to(routing::find_preference))
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

// TODO: Use a graph reference, cloning for each request is not efficient
pub struct AppState {
    graph: Graph,
    driven_routes: Mutex<Vec<Path>>,
    current_route: Mutex<Option<Path>>,
    alpha: Mutex<Preference>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
