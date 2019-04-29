use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

const NODE_TOKENS: usize = 6;
const EDGE_TOKENS: usize = 4;
const COST_DIMENSION: usize = 1;



#[derive(Debug)]
pub struct Node {
    id: usize,
    lat: f64,
    long: f64,
    height: f64,
    ch_level: usize
}

#[derive(Debug)]
struct HalfEdge {
    target_id: usize,
    edge_costs: [f64; COST_DIMENSION]
}

#[derive(Debug)]
pub struct Edge {
    source_id: usize,
    pub target_id: usize,
    edge_costs: [usize; COST_DIMENSION], // change back to float
    repl_edge_1: isize,
    repl_edge_2: isize
}

impl Edge {
    pub fn calc_costs(&self) -> usize {
        self.edge_costs[0]
    }
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    edges: Vec<Edge>,
    half_edges_in: Vec<HalfEdge>,
    half_edges_out: Vec<HalfEdge>,
    offsets_in: Vec<usize>,
    offsets_out: Vec<usize>
}

impl Graph {
    fn new(nodes: Vec<Node>, edges: Vec<Edge>, offsets_out: Vec<usize>) -> Graph {
        Graph {
            // get nodes and edges and set up graph here
            nodes,
            edges,
            half_edges_in: Vec::new(),
            half_edges_out: Vec::new(),
            offsets_in: Vec::new(),
            offsets_out
        }
    }

    pub fn get_edges(&self, node_id: usize) -> &[Edge] {
        &self.edges[self.offsets_out[node_id]..self.offsets_out[node_id + 1]]
    }
}

pub fn parse_graph_file(file_path: &String) -> Result<Graph, Box<dyn std::error::Error>> {
    println!("Parsing graph...");
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();
    let mut offsets_out: Vec<usize> = vec![0; 12];
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut last_edge_source: usize = 0;
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        // iterator lassen und mit next() durchgehen
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens[0] == "#" || tokens.len() == 1 {
            continue;
        }
        if tokens.len() == NODE_TOKENS {
            let new_node = Node {
                id: tokens[0].parse()?,
                lat: tokens[2].parse()?,
                long: tokens[3].parse()?,
                height: tokens[4].parse()?,
                ch_level: tokens[5].parse()?
            };
            nodes.push(new_node);
        } else if tokens.len() == EDGE_TOKENS + COST_DIMENSION {
            let source_id = tokens[0].parse()?;
            if source_id != last_edge_source {
                offsets_out[source_id] = edges.len();
                last_edge_source = source_id;
            }
            let new_edge = Edge {
                source_id,
                target_id: tokens[1].parse()?,
                edge_costs: [tokens[2].parse()?], // eigene Methode: mit for-loop durchgehen
                repl_edge_1: tokens[3].parse()?,
                repl_edge_2: tokens[4].parse()?
            };
            edges.push(new_edge);
        } else {
            println!("Invalid format: {:?}", line);
        }
    }
    Ok(Graph::new(nodes, edges, offsets_out))
}