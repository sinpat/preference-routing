use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

const NODE_TOKENS: usize = 6;
const EDGE_TOKENS: usize = 4;
const COST_DIMENSION: usize = 1;

struct Node {
    id: usize,
    lat: f64,
    long: f64,
    height: f64,
    ch_level: usize
}

struct HalfEdge {
    target_id: usize,
    edge_costs: [f64; COST_DIMENSION]
}

struct Edge {
    source_id: usize,
    target_id: usize,
    // edge_costs: [f64; COST_DIMENSION],
    edge_costs: Vec<f64>,
    repl_edge_1: isize,
    repl_edge_2: isize
}

pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    half_edges_in: Vec<HalfEdge>,
    half_edges_out: Vec<HalfEdge>,
    offsets_in: Vec<usize>,
    offsets_out: Vec<usize>
}

pub fn parse_graph_file(file_path: &String) -> Result<Graph, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for result in reader.lines() {
        let line = result.unwrap();
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens[0] == "#" || tokens.len() == 1 {
            continue;
        }
    }
    Ok(Graph::new())
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            half_edges_in: Vec::new(),
            half_edges_out: Vec::new(),
            offsets_in: Vec::new(),
            offsets_out: Vec::new()
        }
    }
}