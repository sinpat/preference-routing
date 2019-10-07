use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;

mod graph;
mod helpers;
mod lp;
mod server;
mod user;

const EDGE_COST_DIMENSION: usize = 4;
const EDGE_COST_TAGS: [&str; EDGE_COST_DIMENSION] = ["Distance", "Unit", "Height", "UnsuitDist"];

#[derive(Deserialize)]
struct AppConfig {
    port: String,
    database_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide exactly one parameter, which is the path to the graph file");
    }
    let graph_file = &args[1];

    let config = read_config();
    let graph = graph::parse_graph_file(graph_file).unwrap();
    server::start_server(graph, config.port, config.database_path);
}

fn read_config() -> AppConfig {
    match File::open("config.toml") {
        Ok(mut file) => {
            let mut file_content = String::new();
            file.read_to_string(&mut file_content)
                .expect("Could not read config file");
            let config: AppConfig = toml::from_str(&file_content).expect("Could not parse config");
            config
        }
        Err(_err) => panic!("config.toml is missing"),
    }
}
