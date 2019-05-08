use crate::graph::Graph;
use crate::helpers::add_floats;
use std::collections::binary_heap::BinaryHeap;
use ordered_float::OrderedFloat;

pub mod state;
use state::State;
use state::Direction::{ FORWARD, BACKWARD };

pub struct Dijkstra<'a> {
    graph: &'a Graph,
    source_node: usize,
    target_node: usize,
    dist_forward: Vec<OrderedFloat<f64>>,
    dist_backward: Vec<OrderedFloat<f64>>,
    previous: Vec<(Option<usize>, Option<usize>)>,
    successive: Vec<(Option<usize>, Option<usize>)>
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &Graph) -> Dijkstra {
        Dijkstra {
            graph,
            source_node: 0,
            target_node: 0,
            dist_forward: Vec::new(),
            dist_backward: Vec::new(),
            previous: Vec::new(),
            successive: Vec::new()
        }
    }

    pub fn find_shortest_path(&mut self, source: usize, target: usize) -> Option<(Vec<usize>, OrderedFloat<f64>)> {
        println!("Running Dijkstra search...");
        self.init_query(source, target);
        let mut candidates = BinaryHeap::new();
        let mut best_node = (None, OrderedFloat(std::f64::MAX));
        candidates.push(State { node_id: source, cost: OrderedFloat(0.0), direction: FORWARD });
        candidates.push(State { node_id: target, cost: OrderedFloat(0.0), direction: BACKWARD });
        while let Some(State { node_id, cost, direction }) = candidates.pop() {
            if direction == FORWARD {
                if cost > self.dist_forward[node_id] {
                    continue;
                }
                let merged_cost = add_floats(cost, self.dist_backward[node_id]);
                if merged_cost < best_node.1 {
                    best_node = (Some(node_id), merged_cost);
                }
                for half_edge in self.graph.get_ch_edges_out(node_id) {
                    if self.graph.get_ch_level(half_edge.get_target_id()) < self.graph.get_ch_level(node_id) {
                        continue;
                    }
                    let next = State {
                        node_id: half_edge.get_target_id(),
                        cost: add_floats(cost, half_edge.calc_costs()),
                        direction
                    };
                    if next.cost < self.dist_forward[next.node_id] {
                        self.dist_forward[next.node_id] = next.cost;
                        self.previous[next.node_id] = (Some(node_id), Some(half_edge.get_edge_id()));
                        candidates.push(next);
                    }
                }
            }
            if direction == BACKWARD {
                if cost > self.dist_backward[node_id] {
                    continue;
                }
                let merged_cost = add_floats(cost, self.dist_forward[node_id]);
                if merged_cost < best_node.1 {
                    best_node = (Some(node_id), merged_cost);
                }
                for half_edge in self.graph.get_ch_edges_in(node_id) {
                    if self.graph.get_ch_level(half_edge.get_target_id()) < self.graph.get_ch_level(node_id) {
                        continue;
                    }
                    let next = State {
                        node_id: half_edge.get_target_id(),
                        cost: add_floats(cost, half_edge.calc_costs()),
                        direction
                    };
                    if next.cost < self.dist_backward[next.node_id] {
                        self.dist_backward[next.node_id] = next.cost;
                        self.successive[next.node_id] = (Some(node_id), Some(half_edge.get_edge_id()));
                        candidates.push(next);
                    }
                }
            }
        }
        if let Some(node_id) = best_node.0 {
            let path = self.construct_path(node_id);
            return Some((path, best_node.1));
        }
        None
    }

    fn construct_path(&self, best_node: usize) -> Vec<usize> {
        println!("Constructing Path around node {:?}", best_node);
        let mut path = Vec::new();
        let mut node_and_edge = self.previous[best_node];
        while let (Some(node_id), Some(edge_id)) = node_and_edge {
            node_and_edge = self.previous[node_id];
            path.push(edge_id);
        }
        path.reverse();
        node_and_edge = self.successive[best_node];
        while let (Some(node_id), Some(edge_id)) = node_and_edge {
            node_and_edge = self.successive[node_id];
            path.push(edge_id);
        }
        return self.graph.unwrap_edges(path, self.source_node);
    }

    fn init_query(&mut self, source: usize, target: usize) {
        self.source_node = source;
        self.target_node = target;
        let num_of_nodes = self.graph.get_nodes().len();
        self.dist_forward = vec![OrderedFloat(std::f64::MAX); num_of_nodes];
        self.dist_forward[source] = OrderedFloat(0.0);
        self.dist_backward = vec![OrderedFloat(std::f64::MAX); num_of_nodes];
        self.dist_backward[target] = OrderedFloat(0.0);

        self.previous = vec![(None, None); num_of_nodes];
        self.successive = vec![(None, None); num_of_nodes];
    }
}