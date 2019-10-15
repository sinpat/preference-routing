use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use edge::{Edge, HalfEdge};
use node::Node;
use path::Path;

use crate::graph::path::PathSplit;
use crate::helpers::{Coordinate, Preference};
use crate::lp::PreferenceEstimator;
use crate::EDGE_COST_DIMENSION;

mod dijkstra;
mod edge;
mod node;
pub mod path;

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

        // sort nodes by id
        nodes.sort_by(|a, b| a.id.cmp(&b.id));

        // half_edges and offsets out
        edges.sort_by(|a, b| a.source_id.cmp(&b.source_id));
        edges
            .iter()
            .filter(|edge| nodes[edge.target_id].ch_level >= nodes[edge.source_id].ch_level)
            .for_each(|edge| {
                offsets_out[edge.source_id + 1] += 1;
                half_edges_out.push(HalfEdge::new(edge.id, edge.target_id, edge.edge_costs));
            });

        // half_edges and offsets in
        edges.sort_by(|a, b| a.target_id.cmp(&b.target_id));
        edges
            .iter()
            .filter(|edge| nodes[edge.source_id].ch_level >= nodes[edge.target_id].ch_level)
            .for_each(|edge| {
                offsets_in[edge.target_id + 1] += 1;
                half_edges_in.push(HalfEdge::new(edge.id, edge.source_id, edge.edge_costs));
            });

        // finish offset arrays
        for index in 1..offsets_out.len() {
            offsets_out[index] += offsets_out[index - 1];
            offsets_in[index] += offsets_in[index - 1];
        }

        // sort edges by id
        edges.sort_by(|a, b| a.id.cmp(&b.id));
        Graph {
            nodes,
            edges,
            offsets_in,
            offsets_out,
            half_edges_in,
            half_edges_out,
        }
    }

    pub fn find_shortest_path_alt(
        &self,
        include: Vec<Coordinate>,
        alpha: Preference,
    ) -> Option<Path> {
        let include = include
            .iter()
            .map(|x| self.find_closest_node(x).id)
            .collect();
        self.find_shortest_path(include, alpha)
    }

    pub fn find_shortest_path(&self, include: Vec<usize>, alpha: Preference) -> Option<Path> {
        if let Some(result) = dijkstra::find_path(self, &include, alpha) {
            let unpacked_edges: Vec<Vec<usize>> = result
                .edges
                .iter()
                .map(|subpath_edges| {
                    subpath_edges
                        .iter()
                        .flat_map(|edge| self.unpack_edge(*edge))
                        .collect()
                })
                .collect();
            let cuts = unpacked_edges.iter().map(|edges| edges.len()).collect();

            let edges: Vec<usize> = unpacked_edges.into_iter().flatten().collect();
            let mut nodes: Vec<usize> = edges
                .iter()
                .map(|edge| self.edges[*edge].source_id)
                .collect();
            nodes.push(*include.last().unwrap());

            let coordinates = nodes.iter().map(|id| self.nodes[*id].location).collect();
            let waypoints = include.iter().map(|id| self.nodes[*id].location).collect();

            return Some(Path {
                id: 0,
                name: String::from("New Route"),
                nodes,
                edges,
                coordinates,
                waypoints,
                user_split: PathSplit {
                    cuts,
                    alphas: vec![alpha],
                    dimension_costs: result.dimension_costs,
                    costs_by_alpha: result.costs_by_alpha,
                },
                algo_split: None,
            });
        }
        None
    }

    pub fn find_preference(&self, path: &mut Path) {
        println!("=== Calculate Preference ===");
        let path_length = path.nodes.len();
        let mut cuts = Vec::new();
        let mut alphas = Vec::new();
        let mut start: usize = 0;
        while start != path_length - 1 {
            let mut low = start;
            let mut high = path_length;
            let mut best_pref = None;
            let mut best_cut = 0;
            loop {
                let m = (low + high) / 2;
                let mut estimator = PreferenceEstimator::new(self);
                let pref = estimator.calc_preference(&path, start, m);
                if pref.is_some() {
                    low = m + 1;
                    best_pref = pref;
                    best_cut = m;
                } else {
                    high = m;
                }
                if low == high {
                    alphas.push(best_pref.unwrap());
                    cuts.push(best_cut);
                    break;
                }
            }
            start = best_cut;
        }
        let dimension_costs = Vec::new();
        let costs_by_alpha = Vec::new();
        path.algo_split = Some(PathSplit {
            cuts,
            alphas,
            dimension_costs,
            costs_by_alpha,
        });
        println!("=== Found Preference ===");
    }

    pub fn find_closest_node(&self, point: &Coordinate) -> &Node {
        self.nodes
            .iter()
            .min_by_key(|node| point.distance_to(&node.location))
            .expect("The graph has no nodes!")
    }

    fn get_ch_edges_out(&self, node_id: usize) -> &[HalfEdge] {
        &self.half_edges_out[self.offsets_out[node_id]..self.offsets_out[node_id + 1]]
    }

    fn get_ch_edges_in(&self, node_id: usize) -> &[HalfEdge] {
        &self.half_edges_in[self.offsets_in[node_id]..self.offsets_in[node_id + 1]]
    }

    fn unpack_edge(&self, edge: usize) -> Vec<usize> {
        if let Some((edge1, edge2)) = self.edges[edge].replaced_edges {
            let mut first = self.unpack_edge(edge1);
            first.extend(self.unpack_edge(edge2).iter());
            return first;
        }
        vec![edge]
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
    let num_of_nodes = lines
        .next()
        .expect("Number of nodes not present in file")?
        .parse()?;
    let num_of_edges = lines
        .next()
        .expect("Number of edges not present in file")?
        .parse()?;

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
            let replaced_edges = if tokens[tokens.len() - 2] == "-1" {
                None
            } else {
                Some((
                    tokens[tokens.len() - 2].parse()?,
                    tokens[tokens.len() - 1].parse()?,
                ))
            };
            edges.push(Edge::new(
                parsed_edges,
                tokens[0].parse()?,
                tokens[1].parse()?,
                edge::parse_costs(&tokens[2..tokens.len() - 2]),
                replaced_edges,
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
