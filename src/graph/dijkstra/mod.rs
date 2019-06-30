use std::collections::binary_heap::BinaryHeap;

use ordered_float::OrderedFloat;

use state::Direction::{BACKWARD, FORWARD};
use state::State;

use crate::graph::Graph;
use crate::helpers::add_floats;

pub mod state;

pub struct Dijkstra<'a> {
    graph: &'a Graph,
    source_node: usize,
    target_node: usize,
    candidates: BinaryHeap<State>,
    dist_forward: Vec<OrderedFloat<f64>>,
    dist_backward: Vec<OrderedFloat<f64>>,
    previous: Vec<(Option<usize>, Option<usize>)>,
    successive: Vec<(Option<usize>, Option<usize>)>,
    best_node: (Option<usize>, OrderedFloat<f64>),
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &Graph) -> Dijkstra {
        Dijkstra {
            graph,
            source_node: 0,
            target_node: 0,
            candidates: BinaryHeap::new(),
            dist_forward: Vec::new(),
            dist_backward: Vec::new(),
            previous: Vec::new(),
            successive: Vec::new(),
            best_node: (None, OrderedFloat(std::f64::MAX)),
        }
    }

    pub fn find_shortest_path(&mut self, source: usize, target: usize) -> Option<(Vec<usize>, OrderedFloat<f64>)> {
        println!("Running Dijkstra search...");
        self.init_query(source, target);
        self.best_node = (None, OrderedFloat(std::f64::MAX));
        self.candidates.push(State { node_id: source, cost: OrderedFloat(0.0), direction: FORWARD });
        self.candidates.push(State { node_id: target, cost: OrderedFloat(0.0), direction: BACKWARD });
        while let Some(candidate) = self.candidates.pop() {
            self.process_state(candidate);
        }
        match self.best_node {
            (Some(node_id), cost) => {
                println!("Found node {:?} with cost {:?}", node_id, cost);
                let path = self.construct_path(node_id);
                return Some((path, cost));
            }
            (None, _) => None
        }
    }

    fn process_state(&mut self, candidate: State) {
        let State { node_id, cost, direction } = candidate;
        if direction == FORWARD {
            if cost > self.dist_forward[node_id] {
                return;
            }
            let merged_cost = add_floats(cost, self.dist_backward[node_id]);
            if merged_cost < self.best_node.1 {
                self.best_node = (Some(node_id), merged_cost);
            }
            for half_edge in self.graph.get_ch_edges_out(node_id) {
                if self.graph.get_ch_level(half_edge.get_target_id()) < self.graph.get_ch_level(node_id) {
                    continue;
                }
                let next = State {
                    node_id: half_edge.get_target_id(),
                    cost: add_floats(cost, half_edge.calc_costs()),
                    direction,
                };
                if next.cost < self.dist_forward[next.node_id] {
                    self.dist_forward[next.node_id] = next.cost;
                    self.previous[next.node_id] = (Some(node_id), Some(half_edge.get_edge_id()));
                    self.candidates.push(next);
                }
            }
        }
        if direction == BACKWARD {
            if cost > self.dist_backward[node_id] {
                return;
            }
            let merged_cost = add_floats(cost, self.dist_forward[node_id]);
            if merged_cost < self.best_node.1 {
                self.best_node = (Some(node_id), merged_cost);
            }
            for half_edge in self.graph.get_ch_edges_in(node_id) {
                if self.graph.get_ch_level(half_edge.get_target_id()) < self.graph.get_ch_level(node_id) {
                    continue;
                }
                let next = State {
                    node_id: half_edge.get_target_id(),
                    cost: add_floats(cost, half_edge.calc_costs()),
                    direction,
                };
                if next.cost < self.dist_backward[next.node_id] {
                    self.dist_backward[next.node_id] = next.cost;
                    self.successive[next.node_id] = (Some(node_id), Some(half_edge.get_edge_id()));
                    self.candidates.push(next);
                }
            }
        }
    }

    fn construct_path(&self, node_id: usize) -> Vec<usize> {
        println!("Constructing Path around node {:?}", node_id);
        let mut path = Vec::new();
        let mut node_and_edge = self.previous[node_id];
        while let (Some(current_node_id), Some(edge_id)) = node_and_edge {
            node_and_edge = self.previous[current_node_id];
            path.push(edge_id);
        }
        path.reverse();
        node_and_edge = self.successive[node_id];
        while let (Some(current_node_id), Some(edge_id)) = node_and_edge {
            node_and_edge = self.successive[current_node_id];
            path.push(edge_id);
        }
        return self.graph.unwrap_edges(path, self.source_node);
    }

    fn init_query(&mut self, source: usize, target: usize) {
        self.source_node = source;
        self.target_node = target;
        self.candidates = BinaryHeap::new();
        let num_of_nodes = self.graph.get_nodes().len();
        self.dist_forward = vec![OrderedFloat(std::f64::MAX); num_of_nodes];
        self.dist_forward[source] = OrderedFloat(0.0);
        self.dist_backward = vec![OrderedFloat(std::f64::MAX); num_of_nodes];
        self.dist_backward[target] = OrderedFloat(0.0);

        self.previous = vec![(None, None); num_of_nodes];
        self.successive = vec![(None, None); num_of_nodes];
    }
}

#[cfg(test)]
mod tests {
    use ordered_float::OrderedFloat;

    use crate::graph::{Graph, parse_graph_file};

    use super::*;

    fn get_graph() -> Graph {
        parse_graph_file("./src/test_graphs/concTestGraph").unwrap()
    }

    #[test]
    fn from_isolated() {}

    #[test]
    fn to_isolated() {}

    #[test]
    fn to_one_way() {}

    #[test]
    fn from_one_way() {}

    #[test]
    fn normal_case() {
        let graph = get_graph();
        let mut dijkstra = Dijkstra::new(&graph);
        let mut shortest_path;
        let mut path;
        let mut expected_path: Vec<usize>;

        // first query
        shortest_path = dijkstra.find_shortest_path(0, 4);
        assert!(shortest_path.is_none());

        // second query
        shortest_path = dijkstra.find_shortest_path(4, 11);
        assert!(shortest_path.is_none());

        // third query
        shortest_path = dijkstra.find_shortest_path(2, 5);
        assert!(shortest_path.is_some());
        path = shortest_path.unwrap();
        expected_path = vec![2, 4, 5];
        assert_eq!(expected_path, path.0);
        assert_eq!(OrderedFloat(4.0), path.1);

        // fourth query
        shortest_path = dijkstra.find_shortest_path(2, 10);
        assert!(shortest_path.is_some());
        path = shortest_path.unwrap();
        expected_path = vec![2, 4, 5, 7, 10];
        assert_eq!(expected_path, path.0);
        assert_eq!(OrderedFloat(8.0), path.1);

        // fifth query
        shortest_path = dijkstra.find_shortest_path(6, 10);
        assert!(shortest_path.is_some());
        path = shortest_path.unwrap();
        expected_path = vec![6, 4, 5, 7, 10];
        assert_eq!(expected_path, path.0);
        assert_eq!(OrderedFloat(10.0), path.1);
    }
}