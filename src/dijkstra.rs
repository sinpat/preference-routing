use crate::graph;

struct State {
    node_id: usize,
    cost: f64
}

pub fn find_shortest_path(graph: graph::Graph, source: usize, target: usize) -> Option<(Vec<usize>, f64)> {
    println!("Running Dijkstra search...");
    None
}