use crate::graph::Graph;
use crate::helpers::add_floats;
use std::collections::binary_heap::BinaryHeap;
use ordered_float::OrderedFloat;

mod state;
use state::State;
use state::Direction::{ FORWARD, BACKWARD };

pub struct Dijkstra<'a> {
    graph: &'a Graph,
    dist_forward: Vec<(OrderedFloat<f64>, Option<usize>)>,
    dist_backward: Vec<(OrderedFloat<f64>, Option<usize>)>,
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &Graph) -> Dijkstra {
        Dijkstra { graph, dist_forward: Vec::new(), dist_backward: Vec::new() }
    }

    pub fn find_shortest_path(&mut self, source: usize, target: usize) -> Option<(Vec<usize>, OrderedFloat<f64>)> {
        println!("Running Dijkstra search...");
        self.reset_fields();
        let mut candidates = BinaryHeap::new();
        let mut best_node = (None, OrderedFloat(std::f64::MAX));
        self.dist_forward[source] = (OrderedFloat(0.0), None);
        self.dist_backward[target] = (OrderedFloat(0.0), None);
        candidates.push(State { node_id: source, cost: OrderedFloat(0.0), direction: FORWARD });
        candidates.push(State { node_id: target, cost: OrderedFloat(0.0), direction: BACKWARD });
        while let Some(State { node_id, cost, direction }) = candidates.pop() {
            if direction == FORWARD {
                if cost > self.dist_forward[node_id].0 {
                    continue;
                }
                let merged_cost = add_floats(cost, self.dist_backward[node_id].0);
                if merged_cost < best_node.1 {
                    best_node = (Some(node_id), merged_cost);
                }
                for half_edge in self.graph.get_edges_out(node_id) {
                    let next = State {
                        node_id: half_edge.get_target_id(),
                        cost: add_floats(cost, half_edge.calc_costs()),
                        direction
                    };
                    if next.cost < self.dist_forward[next.node_id].0 {
                        self.dist_forward[next.node_id] = (next.cost, Some(node_id));
                        candidates.push(next);
                    }
                }
            }
            if direction == BACKWARD {
                if cost > self.dist_backward[node_id].0 {
                    continue;
                }
                let merged_cost = add_floats(cost, self.dist_forward[node_id].0);
                if merged_cost < best_node.1 {
                    best_node = (Some(node_id), merged_cost);
                }
                for half_edge in self.graph.get_edges_in(node_id) {
                    let next = State {
                        node_id: half_edge.get_target_id(),
                        cost: add_floats(cost, half_edge.calc_costs()),
                        direction
                    };
                    if next.cost < self.dist_backward[next.node_id].0 {
                        self.dist_backward[next.node_id] = (next.cost, Some(node_id));
                        candidates.push(next);
                    }
                }
            }
        }
        self.construct_path(best_node)
    }

    fn construct_path(&self, best_node: (Option<usize>, OrderedFloat<f64>)) -> Option<(Vec<usize>, OrderedFloat<f64>)> {
        if best_node.0.is_some() {
            let node_id = best_node.0.unwrap();
            let mut path = Vec::new();
            let mut current_dist = self.dist_forward[node_id];
            path.push(node_id);
            while let Some(prev) = current_dist.1 {
                path.push(prev);
                current_dist = self.dist_forward[prev];
            }
            path.reverse();
            current_dist = self.dist_backward[node_id];
            while let Some(succ) = current_dist.1 {
                path.push(succ);
                current_dist = self.dist_backward[succ];
            }
            return Some((path, best_node.1));
        }
        None
    }

    fn reset_fields(&mut self) {
        self.dist_forward = vec![(OrderedFloat(std::f64::MAX), None); self.graph.get_nodes().len()];
        self.dist_backward = vec![(OrderedFloat(std::f64::MAX), None); self.graph.get_nodes().len()];
    }
}