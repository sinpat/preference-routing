use crate::config::get_config;
use crate::graph::path::Path;
use crate::helpers::Preference;
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
            alphas: vec![get_config().initial_pref()],
        }
    }

    pub fn add_route(&mut self, route: &mut Path) {
        route.id = self.counter;
        self.driven_routes.push(route.clone());
        self.counter += 1;
    }

    pub fn update_route(&mut self, route: Option<&Path>) {
        if let Some(route) = route {
            let idx = self
                .driven_routes
                .iter()
                .position(|path| path.id == route.id);
            if let Some(idx) = idx {
                self.driven_routes[idx] = route.clone();
            }
        }
    }

    pub fn delete_route(&mut self, id: usize) {
        let index = self.driven_routes.iter().position(|path| path.id == id);
        if let Some(index) = index {
            self.driven_routes.remove(index);
        }
    }

    pub fn add_pref(&mut self) {
        self.alphas.push(get_config().initial_pref());
    }

    pub fn reset(&mut self) {
        self.driven_routes = Vec::new();
        self.alphas = vec![get_config().initial_pref()];
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserAuth {
    pub username: String,
    hash: String,
    pub token: String,
}

impl UserAuth {
    pub fn new(username: String, password: String) -> Self {
        UserAuth {
            token: Self::hash_value(&username),
            username,
            hash: Self::hash_value(&password),
        }
    }

    pub fn credentials_valid(&self, username: &str, password: &str) -> bool {
        let password_hash = Self::hash_value(password);
        self.username == username && self.hash == password_hash
    }

    fn hash_value(value: &str) -> String {
        let mut hasher = Sha3_512::new();
        hasher.input(value);
        hasher
            .result()
            .to_vec()
            .iter()
            .fold(String::new(), |mut acc, val| {
                acc.push_str(&val.to_string());
                acc
            })
    }
}
