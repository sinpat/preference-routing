use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

const NODE_TOKENS: usize = 6;
const EDGE_TOKENS: usize = 4;
const COST_DIMENSION: usize = 1;

#[derive(Debug)]
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

#[derive(Debug)]
struct Edge {
    source_id: usize,
    target_id: usize,
    edge_costs: [f64; COST_DIMENSION],
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

impl Graph {
    fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Graph {
        Graph {
            nodes,
            edges,
            half_edges_in: Vec::new(),
            half_edges_out: Vec::new(),
            offsets_in: Vec::new(),
            offsets_out: Vec::new()
        }
    }
}

pub fn parse_graph_file(file_path: &String) -> Result<Graph, std::io::Error> {
    println!("Parsing graph...");
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for result in reader.lines() {
        let line = result.unwrap();
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens[0] == "#" || tokens.len() == 1 {
            continue;
        }
        if tokens.len() == NODE_TOKENS {
            let new_node = Node {
                id: tokens[0].parse().unwrap(),
                lat: tokens[2].parse().unwrap(),
                long: tokens[3].parse().unwrap(),
                height: tokens[4].parse().unwrap(),
                ch_level: tokens[5].parse().unwrap()
            };
            nodes.push(new_node);
        } else if tokens.len() == EDGE_TOKENS + COST_DIMENSION {
            let new_edge = Edge {
                source_id: tokens[0].parse().unwrap(),
                target_id: tokens[1].parse().unwrap(),
                edge_costs: [tokens[2].parse().unwrap()],
                repl_edge_1: tokens[3].parse().unwrap(),
                repl_edge_2: tokens[4].parse().unwrap()
            };
            edges.push(new_edge);
        } else {
            println!("Invalid format: {:?}", line);
        }
    }
    Ok(Graph::new(nodes, edges))
}