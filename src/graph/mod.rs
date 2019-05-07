use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

mod edge;
mod node;

use edge::{ Edge, HalfEdge };
use node::Node;

const EDGE_COST_DIMENSION: usize = 1;

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
    fn new(nodes: Vec<Node>, mut edges: Vec<Edge>) -> Graph {
        println!("Constructing graph...");
        let mut offsets_out: Vec<usize> = vec![0; nodes.len() + 1];
        let mut offsets_in: Vec<usize> = vec![0; nodes.len() + 1];
        let mut half_edges_out: Vec<HalfEdge> = Vec::new();
        let mut half_edges_in: Vec<HalfEdge> = Vec::new();

        edges.sort_by(|a, b| a.get_source_id().cmp(&b.get_source_id()));
        for edge in &edges {
            offsets_out[edge.get_source_id() + 1] += 1;
            half_edges_out.push(HalfEdge::new(edge.get_target_id(), edge.get_edge_costs()));
        }
        edges.sort_by(|a, b| a.get_target_id().cmp(&b.get_target_id()));
        for edge in &edges {
            offsets_in[edge.get_target_id() + 1] += 1;
            half_edges_in.push(HalfEdge::new(edge.get_source_id(), edge.get_edge_costs()));
        }
        for index in 1..offsets_out.len() {
            offsets_out[index] += offsets_out[index - 1];
            offsets_in[index] += offsets_in[index - 1];
        }
        edges.sort_by(|a, b| a.get_id().cmp(&b.get_id()));
        Graph { nodes, edges, half_edges_in, half_edges_out, offsets_in, offsets_out }
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn get_edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    pub fn get_edges_out(&self, node_id: usize) -> &[HalfEdge] {
        &self.half_edges_out[self.offsets_out[node_id]..self.offsets_out[node_id + 1]]
    }

    pub fn get_edges_in(&self, node_id: usize) -> &[HalfEdge] {
        &self.half_edges_in[self.offsets_in[node_id]..self.offsets_in[node_id + 1]]
    }

    pub fn get_offsets_out(&self) -> &Vec<usize> {
        &self.offsets_out
    }

    pub fn get_offsets_in(&self) -> &Vec<usize> {
        &self.offsets_in
    }
}

pub fn parse_graph_file(file_path: &str) -> Result<Graph, Box<dyn std::error::Error>> {
    println!("Parsing graph...");
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    for _i in 0..4 {
        // comments and blanks
        lines.next();
    }
    assert_eq!(EDGE_COST_DIMENSION, lines.next().expect("No edge cost dim given")?.parse()?);
    let num_of_nodes = lines.next().expect("Number of nodes not present in file")?.parse()?;
    let num_of_edges = lines.next().expect("Number of edges not present in file")?.parse()?;

    let mut parsed_nodes: usize = 0;
    let mut parsed_edges: usize = 0;
    while let Some(Ok(line)) = lines.next() {
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens[0] == "#" || tokens[0] == "\n" {
            continue;
        }
        if parsed_nodes < num_of_nodes {
            nodes.push(Node::new(
                tokens[0].parse()?,
                tokens[2].parse()?,
                tokens[3].parse()?,
                tokens[4].parse()?,
                tokens[5].parse()?
            ));
            parsed_nodes += 1;
        } else if parsed_edges < num_of_edges {
            edges.push(Edge::new(
                parsed_edges,
                tokens[0].parse()?,
                tokens[1].parse()?,
                edge::parse_costs(&tokens[2..tokens.len() - 2]),
                tokens[tokens.len() - 2].parse()?,
                tokens[tokens.len() - 1].parse()?
            ));
            parsed_edges += 1;
        } else {
            panic!("Something doesn't add up with the amount of nodes and edges in graph file");
        }
    }
    Ok(Graph::new(nodes, edges))
}