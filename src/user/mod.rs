use crate::graph::Path;
use crate::helpers::Preference;

const INITIAL_PREF: Preference = [0.0, 1.0, 0.0];

pub struct UserState {
    pub auth: UserAuth,
    pub driven_routes: Vec<Path>,
    pub current_route: Option<Path>,
    pub alpha: Preference,
}

impl UserState {
    pub fn new(username: String, password: String) -> Self {
        UserState {
            auth: UserAuth::new(username, password),
            driven_routes: Vec::new(),
            current_route: None,
            alpha: INITIAL_PREF,
        }
    }

    pub fn reset(&mut self) {
        self.driven_routes = Vec::new();
        self.current_route = None;
        self.alpha = INITIAL_PREF;
    }
}

pub struct UserAuth {
    pub username: String,
    pub password: String,
    pub token: String,
}

impl UserAuth {
    pub fn new(username: String, password: String) -> Self {
        UserAuth {
            token: Self::generate_token(&username),
            username,
            password,
        }
    }

    pub fn update_token(&mut self) -> &str {
        self.token = Self::generate_token(&self.username);
        &self.token
    }

    fn generate_token(username: &str) -> String {
        String::from(username)
    }
}
