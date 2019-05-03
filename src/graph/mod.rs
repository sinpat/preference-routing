use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

mod edge;
mod node;

use edge::{ Edge, HalfEdge };
use node::Node;

#[derive(Debug)]
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
        let mut offsets_out: Vec<usize> = vec![0; nodes.len() + 1];
        offsets_out[0] = 0;
        for edge in &edges {
            offsets_out[edge.get_source_id() + 1] += 1;
        }
        for index in 0..offsets_out.len() - 1 {
            offsets_out[index + 1] += offsets_out[index];
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

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
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
    for _i in 0..5 {
        // comments and blanks
        lines.next();
    }
    let num_of_nodes = lines.next().unwrap().unwrap().parse().unwrap();
    let num_of_edges = lines.next().unwrap().unwrap().parse().unwrap();
    for _i in 0..num_of_nodes {
        let line = lines.next().unwrap().unwrap();
        let tokens: Vec<&str> = line.split(" ").collect();
        nodes.push(Node::new(
            tokens[0].parse()?,
            tokens[2].parse()?,
            tokens[3].parse()?,
            tokens[4].parse()?,
            tokens[5].parse()?
        ));
    }
    for index in 0..num_of_edges {
        let line = lines.next().unwrap().unwrap();
        let tokens: Vec<&str> = line.split(" ").collect();
        edges.push(Edge::new(
            index,
            tokens[0].parse()?,
            tokens[1].parse()?,
            edge::parse_costs(&tokens[2..tokens.len() - 2]),
            tokens[tokens.len() - 2].parse()?,
            tokens[tokens.len() - 1].parse()?
        ));
    }
    Ok(Graph::new(nodes, edges))
}