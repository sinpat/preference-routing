use crate::helpers::Preference;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

static mut INSTANCE: Option<AppConfig> = None;

#[derive(Deserialize)]
pub struct AppConfig {
    port: String,
    database_path: String,
    edge_cost_tags: Vec<String>,
    initial_pref: Preference,
}

impl AppConfig {
    pub fn new() -> Self {
        match File::open("config.toml") {
            Ok(mut file) => {
                let mut file_content = String::new();
                file.read_to_string(&mut file_content)
                    .expect("Could not read config file");
                let config: AppConfig =
                    toml::from_str(&file_content).expect("Could not parse config");
                config
            }
            Err(_err) => panic!("config.toml is missing"),
        }
    }

    pub fn port(&self) -> &str {
        &self.port
    }

    pub fn database_path(&self) -> &str {
        &self.database_path
    }

    pub fn edge_cost_tags(&self) -> &[String] {
        &self.edge_cost_tags
    }

    pub fn initial_pref(&self) -> Preference {
        self.initial_pref
    }
}

pub fn get_config() -> &'static AppConfig {
    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some(AppConfig::new());
        }
        INSTANCE.as_ref().unwrap()
    }
}
