use actix_web::{web, HttpResponse};
use serde::Deserialize;

use super::AppState;
use crate::user::UserState;

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

pub fn login(state: web::Data<AppState>, body: web::Json<Credentials>) -> HttpResponse {
    let Credentials { username, password } = body.into_inner();
    let mut users = state.users.lock().unwrap();
    let user_state = users
        .iter_mut()
        .find(|x| x.auth.credentials_valid(&username, &password));
    match user_state {
        None => HttpResponse::Unauthorized().finish(),
        Some(state) => {
            println!("Login user {}", username);
            HttpResponse::Ok().json(&state.auth.token)
        }
    }
}

pub fn register(state: web::Data<AppState>, body: web::Json<Credentials>) -> HttpResponse {
    let Credentials { username, password } = body.into_inner();
    let mut users = state.users.lock().unwrap();
    let username_taken = users.iter().any(|x| x.auth.username == username);
    if username_taken {
        return HttpResponse::Unauthorized().finish();
    }
    println!("Register user {}", username);
    let new_user = UserState::new(username, password);
    users.push(new_user);
    HttpResponse::Ok().finish()
}
