use actix_web::{web, HttpResponse};
use serde::Deserialize;

use super::AppState;
use crate::server::UserState;

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
            let token = state.auth.update_token();
            super::write_state_to_file(&*users);
            HttpResponse::Ok().json(token)
        }
    }
}

pub fn register(state: web::Data<AppState>, body: web::Json<Credentials>) -> HttpResponse {
    let Credentials { username, password } = body.into_inner();
    let mut users = state.users.lock().unwrap();
    let username_taken = users.iter().any(|x| x.auth.username == username);
    if username_taken {
        return HttpResponse::Ok().finish();
    }
    println!("Register user {}", username);
    let new_user = UserState::new(username, password);
    let token = new_user.auth.token.clone();
    users.push(new_user);
    super::write_state_to_file(&*users);
    HttpResponse::Ok().json(token)
}
