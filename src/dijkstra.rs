use crate::graph::Graph;
use std::collections::binary_heap::BinaryHeap;
use std::cmp::Ordering;

use ordered_float::{OrderedFloat};

#[derive(PartialEq)]
struct State {
    node_id: usize,
    cost: OrderedFloat<f64>
}

impl std::cmp::Eq for State {}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_shortest_path(graph: &Graph, source: usize, target: usize) -> Option<(Vec<usize>, OrderedFloat<f64>)> {
    println!("Running Dijkstra search...");
    let mut dist = vec![(OrderedFloat(std::f64::NAN), None); graph.nodes.len()];
    let mut heap = BinaryHeap::new();
    dist[source] = (OrderedFloat(0.0), None);
    heap.push(State {
        node_id: source,
        cost: OrderedFloat(0.0)
    });
    while let Some(State { node_id, cost }) = heap.pop() {
        if node_id == target {
            let mut path = Vec::new();
            let mut current_dist = dist[target];
            path.push(target);
            while let Some(prev) = current_dist.1 {
                path.push(prev);
                current_dist = dist[prev];
            }
            path.reverse();
            return Some((path, cost));
        }
        if cost > dist[node_id].0 {
            continue
        }
        for edge in graph.get_edges(node_id) {
            let next = State {
                node_id: edge.get_target_id(),
                cost: /*cost + edge.calc_costs()*/ OrderedFloat(0.0)
            };
            if next.cost < dist[next.node_id].0 {
                dist[next.node_id] = (next.cost, Some(node_id));
                heap.push(next);
            }
        }
    }
    None
}