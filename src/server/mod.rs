use actix_web::{App, HttpRequest, HttpResponse, server};
use actix_web::http::Method;
use actix_web::middleware::cors::Cors;

fn handle_register(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .body("register user")
}

fn handle_login(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .header("Authorization", "REPLACE ME")
        .finish()
}

fn set_source(req: &HttpRequest) -> HttpResponse {
    println!("{:?}", req);
    // println!("{:?}", req.content_type());
    HttpResponse::Ok()
        .finish()
}

fn set_target(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .finish()
}

fn handle_fsp(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .body("shortest route")
}

pub fn start_server() {
    server::new(|| {
        vec![
            App::new()
                .prefix("/user")
                .configure(|app| Cors::for_app(app)
                    .resource("/register", |r| r.method(Method::POST).f(handle_register))
                    .resource("/login", |r| r.method(Method::POST).f(handle_login))
                    .register()),
            App::new()
                .prefix("/routing")
                .configure(|app| Cors::for_app(app)
                    .resource("/source", |r| r.method(Method::POST).f(set_source))
                    .resource("/target", |r| r.method(Method::POST).f(set_target))
                    .resource("/fsp", |r| r.method(Method::POST).f(handle_fsp))
                    .register())
        ]
    })
        .bind("localhost:8000")
        .expect("Can not bind to port 8000")
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
}