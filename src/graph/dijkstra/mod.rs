use std::collections::binary_heap::BinaryHeap;
use std::collections::HashSet;
use std::time::Instant;

use ordered_float::OrderedFloat;
use serde::Serialize;

use state::Direction::{BACKWARD, FORWARD};
use state::{NodeState, State};

use crate::graph::Graph;
use crate::helpers::{add_floats, Coordinate, Costs, Preference};
use crate::EDGE_COST_DIMENSION;

use super::edge::{add_edge_costs, calc_total_cost};

pub mod state;

#[derive(Debug, Serialize, Clone)]
pub struct Path {
    pub nodes: Vec<usize>,
    pub coordinates: Vec<Coordinate>,
    pub costs: Costs,
    pub alpha: Preference,
    pub total_cost: f64,
}

pub struct Dijkstra<'a> {
    graph: &'a Graph,
    candidates: BinaryHeap<State>,
    // TODO: Check HashSet operation complexities
    touched_nodes: HashSet<usize>,

    // Contains all the information about the nodes
    node_states: Vec<NodeState>,

    // (node_id, cost array, total_cost)
    best_node: (Option<usize>, Costs, OrderedFloat<f64>),
}

impl<'a> Dijkstra<'a> {
    fn new(graph: &Graph) -> Dijkstra {
        let num_of_nodes = graph.nodes.len();
        Dijkstra {
            graph,
            candidates: BinaryHeap::new(),
            touched_nodes: HashSet::new(),
            node_states: vec![NodeState::new(); num_of_nodes],
            best_node: (
                None,
                [0.0; EDGE_COST_DIMENSION],
                OrderedFloat(std::f64::MAX),
            ),
        }
    }

    fn prepare(&mut self, source: usize, target: usize) {
        // Candidates
        self.candidates = BinaryHeap::new();
        self.candidates.push(State::new(source, FORWARD));
        self.candidates.push(State::new(target, BACKWARD));

        // Touched nodes
        for node_id in &self.touched_nodes {
            self.node_states[*node_id] = NodeState::new();
        }
        self.touched_nodes = HashSet::new();

        // Node states
        self.node_states[source].to_dist.1 = OrderedFloat(0.0);
        self.node_states[target].from_dist.1 = OrderedFloat(0.0);
        self.touched_nodes.insert(source);
        self.touched_nodes.insert(target);

        // Best node
        self.best_node = (
            None,
            [0.0; EDGE_COST_DIMENSION],
            OrderedFloat(std::f64::MAX),
        );
    }

    fn run(&mut self, source: usize, target: usize, alpha: Preference) -> Option<Path> {
        self.prepare(source, target);

        // TODO: Implement termination condition?
        let now = Instant::now();
        while let Some(candidate) = self.candidates.pop() {
            self.process_state(candidate, alpha);
        }
        match self.best_node {
            (None, _, _) => None,
            (Some(node_id), costs, total_cost) => {
                println!(
                    "Found node {:?} with cost {:?} in {:?}ms",
                    node_id,
                    total_cost,
                    now.elapsed().as_millis()
                );
                let path = self.construct_path(node_id, source);
                let coordinates = path
                    .iter()
                    .map(|id| Coordinate {
                        lat: self.graph.nodes[*id].location.lat,
                        lng: self.graph.nodes[*id].location.lng,
                    })
                    .collect();
                Some(Path {
                    nodes: path,
                    coordinates,
                    costs,
                    alpha,
                    total_cost: total_cost.into_inner(),
                })
            }
        }
    }

    fn process_state(&mut self, candidate: State, alpha: Preference) {
        let State {
            node_id,
            costs,
            total_cost,
            direction,
        } = candidate;
        let mut node_state = &self.node_states[node_id];
        if direction == FORWARD {
            if total_cost > node_state.to_dist.1 {
                return;
            };
            let merged_cost_vector = add_edge_costs(costs, node_state.from_dist.0);
            let merged_cost = add_floats(total_cost, node_state.from_dist.1);
            if merged_cost < self.best_node.2 {
                self.best_node = (Some(node_id), merged_cost_vector, merged_cost);
            }
            for half_edge in self.graph.get_ch_edges_out(node_id) {
                let next = State {
                    node_id: half_edge.target_id,
                    costs: add_edge_costs(costs, half_edge.edge_costs),
                    total_cost: add_floats(
                        total_cost,
                        calc_total_cost(half_edge.edge_costs, alpha),
                    ),
                    direction,
                };
                let next_node_state = &mut self.node_states[next.node_id];
                if next.total_cost < next_node_state.to_dist.1 {
                    next_node_state.to_dist = (next.costs, next.total_cost);
                    next_node_state.previous = Some((node_id, half_edge.edge_id));
                    self.touched_nodes.insert(next.node_id);
                    self.candidates.push(next);
                }
            }
        }
        node_state = &self.node_states[node_id];
        if direction == BACKWARD {
            if total_cost > node_state.from_dist.1 {
                return;
            }
            let merged_cost_vector = add_edge_costs(costs, node_state.to_dist.0);
            let merged_cost = add_floats(total_cost, node_state.to_dist.1);
            if merged_cost < self.best_node.2 {
                self.best_node = (Some(node_id), merged_cost_vector, merged_cost);
            }
            for half_edge in self.graph.get_ch_edges_in(node_id) {
                let next = State {
                    node_id: half_edge.target_id,
                    costs: add_edge_costs(costs, half_edge.edge_costs),
                    total_cost: add_floats(
                        total_cost,
                        calc_total_cost(half_edge.edge_costs, alpha),
                    ),
                    direction,
                };
                let next_node_state = &mut self.node_states[next.node_id];
                if next.total_cost < next_node_state.from_dist.1 {
                    next_node_state.from_dist = (next.costs, next.total_cost);
                    next_node_state.successive = Some((node_id, half_edge.edge_id));
                    self.touched_nodes.insert(next.node_id);
                    self.candidates.push(next);
                }
            }
        }
    }

    fn construct_path(&self, node_id: usize, source: usize) -> Vec<usize> {
        let mut path = Vec::new();
        let node_state = &self.node_states[node_id];
        let mut node_and_edge = node_state.previous;
        while let Some((current_node_id, edge_id)) = node_and_edge {
            node_and_edge = self.node_states[current_node_id].previous;
            path.push(edge_id);
        }
        path.reverse();
        node_and_edge = node_state.successive;
        while let Some((current_node_id, edge_id)) = node_and_edge {
            node_and_edge = self.node_states[current_node_id].successive;
            path.push(edge_id);
        }
        self.graph.unwrap_edges(path, source)
    }
}

pub fn find_path(graph: &Graph, include: Vec<usize>, alpha: Preference) -> Option<Path> {
    println!("=== Running Dijkstra search ===");
    let mut dijkstra = Dijkstra::new(graph);
    let mut path = Vec::new();
    let mut coordinates = Vec::new();
    let mut costs = [0.0; EDGE_COST_DIMENSION];
    let mut total_cost = 0.0;
    include.windows(2).for_each(|win| {
        if let Some(mut result) = dijkstra.run(win[0], win[1], alpha) {
            path.append(&mut result.nodes);
            coordinates.append(&mut result.coordinates);
            costs = add_edge_costs(costs, result.costs);
            total_cost += result.total_cost;
        }
    });
    println!(
        "=== Found path with costs: {:?} and total cost: {} ===",
        costs, total_cost
    );
    Some(Path {
        nodes: path,
        coordinates,
        costs,
        alpha,
        total_cost,
    })
}

#[cfg(test)]
mod tests {
    use ordered_float::OrderedFloat;

    use crate::graph::{parse_graph_file, Graph};

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
        /*
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
        */
    }
}
