use crate::graph::path::Path;
use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;

mod config;
mod graph;
mod helpers;
mod lp;
mod server;
mod user;

const EDGE_COST_DIMENSION: usize = 4;
const ITERATIONS: usize = 25;

/*
#[derive(Debug, Serialize)]
pub struct CalcPrefTracker {
    total_time: u128,
    n_dijkstras: usize,
    n_lp_solves: usize,
    n_added_constraints: usize,
}

#[derive(Debug, Serialize)]
struct SubpathTracker {
    total_time: u64,
    iterations: Vec<CalcPrefTracker>,
}
*/

#[derive(Debug, Serialize, Default)]
pub struct RuntimeTracker {
    total_times: Vec<Vec<u64>>,
    subpath_times: Vec<Vec<u64>>,
    calc_pref_times: Vec<u128>,
    n_calc_pref: Vec<usize>,
    dijkstra_times: Vec<u128>,
    lp_solve_times: Vec<u128>,
    n_lp_solves: Vec<usize>,
    // n_dijkstras: Vec<usize>,
    // n_added_constraints: Vec<usize>,
    // subpaths: Vec<SubpathTracker>,
}

/*
impl RuntimeTracker {
    pub fn new() -> Self {
        RuntimeTracker {
            times: Vec::new(),
            subpath_times: Vec::new(),
            calc_pref_times: Vec::new(),
            n_dijkstras: Vec::new(),
            n_lp_solves: Vec::new(),
            n_added_constraints: Vec::new(),
        }
    }
}
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide exactly one parameter, which is the path to the graph file");
    }
    let graph = graph::parse_graph_file(&args[1]).unwrap();

    let mut file = File::open("case-study-routes").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let routes: Vec<Path> = serde_json::from_str(&content).unwrap();

    // let mut results: Vec<Vec<RuntimeTracker>> = Vec::new();
    let mut results = RuntimeTracker::default();
    for (index, mut path) in routes.into_iter().enumerate() {
        results.total_times.push(Vec::new());
        results.subpath_times.push(Vec::new());
        // let mut route_results = Vec::new();
        for iteration in 0..ITERATIONS {
            println!("Route {}, Iteration {}", index + 1, iteration + 1);
            let now = Instant::now();
            graph.find_preference(&mut path, &mut results, index);
            results.total_times[index].push(now.elapsed().as_secs());
            // route_results.push(tracker);
        }
        // results.push(route_results);
    }

    let total_times: Vec<f64> = results
        .total_times
        .iter()
        .map(|result| {
            let sum: u64 = result.iter().sum();
            (sum as f64) / (result.len() as f64)
        })
        .collect();
    let subpath_times: Vec<f64> = results
        .subpath_times
        .iter()
        .map(|result| {
            let sum: u64 = result.iter().sum();
            (sum as f64) / (result.len() as f64)
        })
        .collect();
    let calc_pref_times: u128 = results.calc_pref_times.iter().sum();
    let n_calc_pref: usize = results.n_calc_pref.iter().sum();
    let dijkstra_times: u128 = results.dijkstra_times.iter().sum();
    let lp_solve_times: u128 = results.lp_solve_times.iter().sum();
    let n_lp_solves: usize = results.n_lp_solves.iter().sum();

    let mut file = File::create("runtime-analysis").expect("Could not create file");
    file.write_all(format!("total: {:?}\n", total_times).as_bytes());
    file.write_all(format!("subpath: {:?}\n", subpath_times).as_bytes());
    // file.write_all(format!("subpath: {:?}\n", (subpath_times as f64) / results.subpath_times.len() as f64).as_bytes());
    file.write_all(
        format!(
            "calc_pref: {:?}\n",
            (calc_pref_times as usize) / results.calc_pref_times.len()
        )
        .as_bytes(),
    );
    file.write_all(
        format!(
            "n_calc_pref: {:?}\n",
            (n_calc_pref as f64) / (results.n_calc_pref.len() as f64)
        )
        .as_bytes(),
    );
    file.write_all(
        format!(
            "dijkstra: {:?}\n",
            (dijkstra_times as usize) / results.dijkstra_times.len()
        )
        .as_bytes(),
    );
    file.write_all(
        format!(
            "lp_solve: {:?}\n",
            (lp_solve_times as usize) / results.lp_solve_times.len()
        )
        .as_bytes(),
    );
    file.write_all(
        format!(
            "n_lp_solves: {:?}\n",
            (n_lp_solves as f64) / (results.n_lp_solves.len() as f64)
        )
        .as_bytes(),
    );

    // server::start_server(graph);
}
