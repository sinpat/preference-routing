use crate::graph::path::Path;
use crate::helpers::Preference;
use crate::INITIAL_PREF;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_512};

#[derive(Deserialize, Serialize)]
pub struct UserState {
    pub auth: UserAuth,
    pub driven_routes: Vec<Path>,
    pub counter: usize,
    pub alphas: Vec<Preference>,
}

impl UserState {
    pub fn new(username: String, password: String) -> Self {
        UserState {
            auth: UserAuth::new(username, password),
            driven_routes: Vec::new(),
            counter: 1,
            alphas: vec![INITIAL_PREF],
        }
    }

    pub fn add_route(&mut self, mut route: Path) -> Path {
        route.id = self.counter;
        route.name = format!("Route {}", self.counter);
        self.driven_routes.push(route.clone());
        self.counter += 1;
        return route
    }

    /*
    pub fn add_pref(&mut self) {
        self.driven_routes.push(Vec::new());
        self.alphas.push(INITIAL_PREF);
    }
    */

    pub fn reset(&mut self) {
        self.driven_routes = Vec::new();
        self.alphas = vec![INITIAL_PREF];
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserAuth {
    pub username: String,
    hash: Vec<u8>,
    pub token: String,
}

impl UserAuth {
    pub fn new(username: String, password: String) -> Self {
        UserAuth {
            token: String::new(),
            username,
            hash: Self::hash_password(&password),
        }
    }

    pub fn update_token(&mut self) -> String {
        self.token = self.generate_token();
        self.token.to_string()
    }

    pub fn credentials_valid(&self, username: &str, password: &str) -> bool {
        let password_hash = Self::hash_password(password);
        self.username == username && self.hash == password_hash
    }

    fn generate_token(&self) -> String {
        String::from(&self.username)
    }

    fn hash_password(password: &str) -> Vec<u8> {
        let mut hasher = Sha3_512::new();
        hasher.input(password);
        hasher.result().to_vec()
    }
}
