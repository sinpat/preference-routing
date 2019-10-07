use crate::helpers::Preference;
use serde::Deserialize;
use std::env;

mod config;
mod graph;
mod helpers;
mod lp;
mod server;
mod user;

const EDGE_COST_DIMENSION: usize = 4;
const EDGE_COST_TAGS: [&str; EDGE_COST_DIMENSION] = ["Distance", "Unit", "Height", "UnsuitDist"];
const INITIAL_PREF: Preference = [0.0, 0.0, 0.0, 1.0];

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

    let graph = graph::parse_graph_file(graph_file).unwrap();
    server::start_server(graph);
}
