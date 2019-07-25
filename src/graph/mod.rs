use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use ordered_float::OrderedFloat;

use edge::{Edge, HalfEdge};
use node::Node;

use crate::EDGE_COST_DIMENSION;
use crate::helpers::Coordinate;
use crate::graph::dijkstra::Dijkstra;

mod edge;
mod node;
pub mod dijkstra;

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub offsets_in: Vec<usize>,
    pub offsets_out: Vec<usize>,
    half_edges_in: Vec<HalfEdge>,
    half_edges_out: Vec<HalfEdge>,
}

impl Graph {
    fn new(mut nodes: Vec<Node>, mut edges: Vec<Edge>) -> Graph {
        println!("Constructing graph...");
        let mut offsets_out: Vec<usize> = vec![0; nodes.len() + 1];
        let mut offsets_in: Vec<usize> = vec![0; nodes.len() + 1];
        let mut half_edges_out: Vec<HalfEdge> = Vec::new();
        let mut half_edges_in: Vec<HalfEdge> = Vec::new();

        // edges and offsets out
        edges.sort_by(|a, b| a.source_id.cmp(&b.source_id));
        for edge in &edges {
            offsets_out[edge.source_id + 1] += 1;
            half_edges_out.push(HalfEdge::new(edge.id, edge.target_id, edge.edge_costs));
        }

        // edges and offsets in
        edges.sort_by(|a, b| a.target_id.cmp(&b.target_id));
        for edge in &edges {
            offsets_in[edge.target_id + 1] += 1;
            half_edges_in.push(HalfEdge::new(edge.id, edge.source_id, edge.edge_costs));
        }

        // finish offset arrays
        for index in 1..offsets_out.len() {
            offsets_out[index] += offsets_out[index - 1];
            offsets_in[index] += offsets_in[index - 1];
        }

        // sort nodes and edges by id
        edges.sort_by(|a, b| a.id.cmp(&b.id));
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        Graph { nodes, edges, offsets_in, offsets_out, half_edges_in, half_edges_out }
    }

    pub fn find_shortest_path(
        &self,
        source: usize,
        target: usize,
        include: Vec<Coordinate>,
        avoid: Vec<Coordinate>,
        alpha: [f64; EDGE_COST_DIMENSION]
    ) -> Option<(Vec<usize>, [f64; EDGE_COST_DIMENSION], f64)> {
        let mut dijkstra = Dijkstra::new(self);
        dijkstra.run(source, target, include, avoid, alpha)
    }

    pub fn get_ch_edges_out(&self, node_id: usize) -> Vec<&HalfEdge> {
        self.get_edges_out(node_id)
            .iter()
            .filter(|x| self.nodes[x.target_id].ch_level >= self.nodes[node_id].ch_level)
            .collect()
    }

    pub fn get_ch_edges_in(&self, node_id: usize) -> Vec<&HalfEdge> {
        self.get_edges_in(node_id)
            .iter()
            .filter(|x| self.nodes[x.target_id].ch_level >= self.nodes[node_id].ch_level)
            .collect()
    }

    pub fn find_closest_node(&self, point: Coordinate) -> (&Coordinate, usize) {
        // TODO: Return Option
        let mut closest = &self.nodes[0];
        let mut distance = OrderedFloat(std::f64::MAX);
        for node in &self.nodes {
            let current_distance = point.distance_to(&node.location);
            if current_distance < distance {
                closest = node;
                distance = current_distance;
            }
        }
        (&closest.location, closest.id)
    }

    fn get_edges_out(&self, node_id: usize) -> &[HalfEdge] {
        &self.half_edges_out[self.offsets_out[node_id]..self.offsets_out[node_id + 1]]
    }

    fn get_edges_in(&self, node_id: usize) -> &[HalfEdge] {
        &self.half_edges_in[self.offsets_in[node_id]..self.offsets_in[node_id + 1]]
    }

    pub fn unwrap_edges(&self, edge_path: Vec<usize>, source_node: usize) -> Vec<usize> {
        let mut node_path = vec![source_node];
        for edge_id in edge_path {
            node_path.append(&mut self.unpack_edge(edge_id));
        }
        node_path
    }

    fn unpack_edge(&self, edge_id: usize) -> Vec<usize> {
        let edge = &self.edges[edge_id];
        if let Some((edge_1, edge_2)) = edge.get_replaced_edges() {
            let mut relaxed_nodes = self.unpack_edge(edge_1);
            relaxed_nodes.append(&mut self.unpack_edge(edge_2));
            return relaxed_nodes;
        }
        vec![edge.target_id]
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
    let cost_dim: usize = lines.next().expect("No edge cost dim given")?.parse()?;
    assert_eq!(EDGE_COST_DIMENSION, cost_dim);
    let num_of_nodes = lines.next().expect("Number of nodes not present in file")?.parse()?;
    let num_of_edges = lines.next().expect("Number of edges not present in file")?.parse()?;

    let mut parsed_nodes: usize = 0;
    let mut parsed_edges: usize = 0;
    while let Some(Ok(line)) = lines.next() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens[0] == "#" || tokens[0] == "\n" {
            continue;
        }
        if parsed_nodes < num_of_nodes {
            nodes.push(Node::new(
                tokens[0].parse()?,
                tokens[2].parse()?,
                tokens[3].parse()?,
                tokens[4].parse()?,
                tokens[5].parse()?,
            ));
            parsed_nodes += 1;
        } else if parsed_edges < num_of_edges {
            edges.push(Edge::new(
                parsed_edges,
                tokens[0].parse()?,
                tokens[1].parse()?,
                edge::parse_costs(&tokens[2..tokens.len() - 2]),
                tokens[tokens.len() - 2].parse()?,
                tokens[tokens.len() - 1].parse()?,
            ));
            parsed_edges += 1;
        } else {
            panic!("Something doesn't add up with the amount of nodes and edges in graph file");
        }
    }
    Ok(Graph::new(nodes, edges))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_parsing() {
        let result = parse_graph_file("./src/test_graphs/testGraph");
        let graph = result.unwrap();
        assert_eq!(12, graph.nodes.len());
        assert_eq!(18, graph.edges.len());

        let exp_offsets_out: Vec<usize> = vec![0, 0, 2, 6, 7, 9, 10, 12, 13, 15, 17, 18, 18];
        let exp_offsets_in: Vec<usize> = vec![0, 1, 2, 3, 4, 6, 8, 11, 12, 14, 16, 18, 18];
        assert_eq!(exp_offsets_out, graph.offsets_out);
        assert_eq!(exp_offsets_in, graph.offsets_in);
    }
}