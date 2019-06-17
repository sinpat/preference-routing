use actix_web::{App, http::Method, HttpRequest, server, HttpResponse};
use actix_web::middleware::cors::Cors;

fn handle_register(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .body("register user")
}

fn handle_login(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .header("Authorization", "generatedToken")
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
                    .allowed_origin("http://localhost:8080")
                    .resource("/register", |r| r.method(Method::POST).f(handle_register))
                    .resource("/login", |r| r.method(Method::POST).f(handle_login))
                    .register()),
            App::new()
                .prefix("/routing")
                .configure(|app| Cors::for_app(app)
                    .allowed_origin("http://localhost:8080")
                    .resource("/fsp", |r| r.method(Method::POST).f(handle_fsp))
                    .register())
        ]
    })
        .bind("localhost:8000")
        .unwrap()
        .run();
}