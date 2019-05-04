use crate::graph::Graph;
use std::collections::binary_heap::BinaryHeap;

use ordered_float::{ OrderedFloat };

mod state;

use state::State;

// find shortest path by doing a dijkstra search
pub fn find_shortest_path(graph: &Graph, source: usize, target: usize) -> Option<(Vec<usize>, OrderedFloat<f64>)> {
    println!("Running Dijkstra search...");
    let mut dist_forward = vec![(OrderedFloat(std::f64::NAN), None); graph.get_nodes().len()];
    let mut dist_backward = vec![(OrderedFloat(std::f64::NAN), None); graph.get_nodes().len()];
    let mut candidates = BinaryHeap::new();
    let mut last_node = (None, None);
    dist_forward[source] = (OrderedFloat(0.0), None);
    dist_backward[target] = (OrderedFloat(0.0), None);
    candidates.push(State { node_id: source, cost: OrderedFloat(0.0), forward: true });
    candidates.push(State { node_id: target, cost: OrderedFloat(0.0), forward: false });
    while let Some(State { node_id, cost, forward }) = candidates.pop() {
        if forward {
            if cost > dist_forward[node_id].0 {
                continue;
            }
            last_node.0 = Some(node_id);
            for half_edge in graph.get_edges_out(node_id) {
                let next = State {
                    node_id: half_edge.get_target_id(),
                    cost: OrderedFloat(cost.0 + half_edge.calc_costs().0),
                    forward
                };
                if next.cost < dist_forward[next.node_id].0 {
                    dist_forward[next.node_id] = (next.cost, Some(node_id));
                    candidates.push(next);
                }
            }
        }
        if !forward {
            if cost > dist_backward[node_id].0 {
                continue;
            }
            last_node.1 = Some(node_id);
            for half_edge in graph.get_edges_in(node_id) {
                let next = State {
                    node_id: half_edge.get_target_id(),
                    cost: OrderedFloat(cost.0 + half_edge.calc_costs().0),
                    forward
                };
                if next.cost < dist_backward[next.node_id].0 {
                    dist_backward[next.node_id] = (next.cost, Some(node_id));
                    candidates.push(next);
                }
            }
        }
        if last_node.0 == last_node.1 {
            let path = construct_path(&dist_forward, &dist_backward, last_node.0.unwrap());
            let dist_up = dist_forward[last_node.0.unwrap()].0;
            let dist_down = dist_backward[last_node.0.unwrap()].0;
            return Some((path, OrderedFloat(dist_up.0 + dist_down.0)));
        }
    }
    None
}

fn construct_path(dist_forward: &Vec<(OrderedFloat<f64>, Option<usize>)>, dist_backward: &Vec<(OrderedFloat<f64>, Option<usize>)>, node_id: usize) -> Vec<usize> {
    let mut path = Vec::new();
    let mut current_dist = dist_forward[node_id];
    path.push(node_id);
    while let Some(prev) = current_dist.1 {
        path.push(prev);
        current_dist = dist_forward[prev];
    }
    path.reverse();
    current_dist = dist_backward[node_id];
    while let Some(succ) = current_dist.1 {
        path.push(succ);
        current_dist = dist_backward[succ];
    }
    path
}