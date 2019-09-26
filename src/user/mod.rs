use crate::graph::Path;
use crate::helpers::Preference;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_512};

const INITIAL_PREF: Preference = [1.0, 0.0, 0.0];

#[derive(Deserialize, Serialize)]
pub struct UserState {
    pub auth: UserAuth,
    pub driven_routes: Vec<Vec<Path>>,
    pub current_route: Option<Path>,
    pub alphas: Vec<Preference>,
}

impl UserState {
    pub fn new(username: String, password: String) -> Self {
        UserState {
            auth: UserAuth::new(username, password),
            driven_routes: vec![Vec::new()],
            current_route: None,
            alphas: vec![INITIAL_PREF],
        }
    }

    pub fn add_pref(&mut self) {
        self.driven_routes.push(Vec::new());
        self.alphas.push(INITIAL_PREF);
    }

    pub fn reset(&mut self) {
        self.driven_routes = vec![Vec::new()];
        self.current_route = None;
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
            token: Self::generate_token(&username),
            username,
            hash: Self::hash_password(&password),
        }
    }

    pub fn update_token(&mut self) -> String {
        self.token = Self::generate_token(&self.username);
        self.token.to_string()
    }

    pub fn credentials_valid(&self, username: &str, password: &str) -> bool {
        let password_hash = Self::hash_password(password);
        self.username == username && self.hash == password_hash
    }

    fn generate_token(username: &str) -> String {
        String::from(username)
    }

    fn hash_password(password: &str) -> Vec<u8> {
        let mut hasher = Sha3_512::new();
        hasher.input(password);
        hasher.result().to_vec()
    }
}
