use actix_web::{App, http::Method, HttpRequest, server};

fn handle_register(_req: &HttpRequest) -> &'static str {
    "Hello, world"
}

fn handle_login(_req: &HttpRequest) -> &'static str {
    "Hello, world!"
}

fn handle_fsp(_req: &HttpRequest) -> &'static str {
    "Hello, world"
}

pub fn start_server() {
    server::new(|| {
        vec![
            App::new()
                .prefix("/user")
                .resource("/register", |r| r.method(Method::POST).f(handle_register))
                .resource("/login", |r| r.method(Method::POST).f(handle_login)),
            App::new()
                .prefix("/routing")
                .resource("/fsp", |r| r.method(Method::POST).f(handle_fsp))
        ]
    })
        .bind("localhost:8000")
        .unwrap()
        .run();
}