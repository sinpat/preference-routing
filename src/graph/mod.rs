use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

mod edge;
mod node;

use edge::{ Edge, HalfEdge };
use node::Node;

const NODE_TOKENS: usize = 6;
const EDGE_TOKENS: usize = 4;
const COST_DIMENSION: usize = 1; // doubled in edge!

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
    fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Graph {
        let mut offsets_out: Vec<usize> = vec![edges.len(); nodes.len() + 1];
        offsets_out[0] = 0;
        for (index, edge) in edges.iter().enumerate() {
            if edge.get_source_id() == 0 {
                continue;
            }
            offsets_out[edge.get_source_id()] = index;
        }
        Graph {
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
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        // iterator lassen und mit next() durchgehen
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens[0] == "#" || tokens.len() == 1 {
            continue;
        }
        if tokens.len() == NODE_TOKENS {
            nodes.push(Node::new(
                tokens[0].parse()?,
                tokens[2].parse()?,
                tokens[3].parse()?,
                tokens[4].parse()?,
                tokens[5].parse()?
            ));
        } else if tokens.len() == EDGE_TOKENS + COST_DIMENSION {
            edges.push(Edge::new(
                tokens[0].parse()?,
                tokens[1].parse()?,
                [tokens[2].parse()?], // eigene Methode: mit for-loop durchgehen
                tokens[3].parse()?,
                tokens[4].parse()?
            ));
        } else {
            println!("Invalid format: {:?}", line);
        }
    }
    Ok(Graph::new(nodes, edges))
}