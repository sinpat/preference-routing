use std::collections::binary_heap::BinaryHeap;
use std::time::Instant;

use ordered_float::OrderedFloat;

use state::Direction::{BACKWARD, FORWARD};
use state::{NodeState, State};

use crate::graph::{EdgeId, Graph, NodeId};
use crate::helpers::{add_floats, Costs, Preference};
use crate::EDGE_COST_DIMENSION;

use super::edge::{add_edge_costs, calc_total_cost};

pub mod state;

pub struct DijkstraResult {
    pub edges: Vec<EdgeId>,
    pub costs: Costs,
    pub total_cost: f64,
}

pub struct Dijkstra<'a> {
    graph: &'a Graph,
    candidates: BinaryHeap<State>,
    touched_nodes: Vec<NodeId>,
    found_best_backward: bool,
    found_best_forward: bool,

    // Contains all the information about the nodes
    node_states: Vec<NodeState>,

    // (node_id, cost array, total_cost)
    best_node: (Option<NodeId>, Costs, OrderedFloat<f64>),
}

impl<'a> Dijkstra<'a> {
    fn new(graph: &Graph) -> Dijkstra {
        let num_of_nodes = graph.nodes.len();
        Dijkstra {
            graph,
            candidates: BinaryHeap::new(),
            touched_nodes: Vec::new(),
            found_best_backward: false,
            found_best_forward: false,
            node_states: vec![NodeState::new(); num_of_nodes],
            best_node: (
                None,
                [0.0; EDGE_COST_DIMENSION],
                OrderedFloat(std::f64::MAX),
            ),
        }
    }

    fn prepare(&mut self, source: NodeId, target: NodeId) {
        // Candidates
        self.candidates = BinaryHeap::new();
        self.candidates.push(State::new(source, FORWARD));
        self.candidates.push(State::new(target, BACKWARD));

        // Touched nodes
        for node_id in &self.touched_nodes {
            self.node_states[*node_id] = NodeState::new();
        }
        self.touched_nodes.clear();

        self.found_best_backward = false;
        self.found_best_forward = false;

        // Node states
        self.node_states[source].dist_f.1 = OrderedFloat(0.0);
        self.node_states[target].dist_b.1 = OrderedFloat(0.0);
        self.touched_nodes.push(source);
        self.touched_nodes.push(target);

        // Best node
        self.best_node = (
            None,
            [0.0; EDGE_COST_DIMENSION],
            OrderedFloat(std::f64::MAX),
        );
    }

    fn run(&mut self, source: NodeId, target: NodeId, alpha: Preference) -> Option<DijkstraResult> {
        self.prepare(source, target);

        let now = Instant::now();
        while let Some(candidate) = self.candidates.pop() {
            if !(self.found_best_forward && self.found_best_backward) {
                self.process_state(candidate, alpha);
            }
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
                let edges = self.make_edge_path(node_id);
                Some(DijkstraResult {
                    edges,
                    costs,
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
        let node_state = &self.node_states[node_id];

        let my_costs;
        let other_costs;
        if direction == FORWARD {
            my_costs = node_state.dist_f;
            other_costs = node_state.dist_b;
        } else {
            my_costs = node_state.dist_b;
            other_costs = node_state.dist_f;
        };

        if total_cost > my_costs.1 {
            return;
        };
        if total_cost > self.best_node.2 {
            if direction == FORWARD {
                self.found_best_forward = true;
            } else {
                self.found_best_backward = true;
            }
            return;
        }
        if other_costs.1 != OrderedFloat(std::f64::MAX) {
            let merged_cost = add_floats(total_cost, other_costs.1);
            if merged_cost < self.best_node.2 {
                let merged_cost_vector = add_edge_costs(costs, other_costs.0);
                self.best_node = (Some(node_id), merged_cost_vector, merged_cost);
            }
        }

        let edges = if direction == FORWARD {
            self.graph.get_ch_edges_out(node_id)
        } else {
            self.graph.get_ch_edges_in(node_id)
        };
        for half_edge in edges {
            let next_node = half_edge.target_id;
            let next_costs = add_edge_costs(costs, half_edge.edge_costs);
            let next_total_cost =
                add_floats(total_cost, calc_total_cost(half_edge.edge_costs, alpha));

            let next_node_state = &mut self.node_states[next_node];
            let dist;
            let previous;
            if direction == FORWARD {
                dist = &mut next_node_state.dist_f;
                previous = &mut next_node_state.previous_f;
            } else {
                dist = &mut next_node_state.dist_b;
                previous = &mut next_node_state.previous_b;
            };
            if next_total_cost < dist.1 {
                *dist = (next_costs, next_total_cost);
                *previous = Some((node_id, half_edge.edge_id));
                self.touched_nodes.push(next_node);
                self.candidates.push(State {
                    node_id: next_node,
                    costs: next_costs,
                    total_cost: next_total_cost,
                    direction,
                });
            }
        }
    }

    fn make_edge_path(&self, connector: NodeId) -> Vec<EdgeId> {
        let mut edges = Vec::new();
        let mut previous_state = self.node_states[connector].previous_f;
        let mut successive_state = self.node_states[connector].previous_b;

        // backwards
        while let Some((previous_node, edge_id)) = previous_state {
            edges.push(edge_id);
            previous_state = self.node_states[previous_node].previous_f;
        }
        edges.reverse();

        // forwards
        while let Some((successive_node, edge_id)) = successive_state {
            edges.push(edge_id);
            successive_state = self.node_states[successive_node].previous_b;
        }
        edges
    }
}

pub fn find_path(graph: &Graph, include: Vec<NodeId>, alpha: Preference) -> DijkstraResult {
    println!("=== Running Dijkstra search ===");
    let mut dijkstra = Dijkstra::new(graph);
    let mut edges = Vec::new();
    let mut costs = [0.0; EDGE_COST_DIMENSION];
    let mut total_cost = 0.0;
    include.windows(2).for_each(|win| {
        if let Some(mut result) = dijkstra.run(win[0], win[1], alpha) {
            edges.append(&mut result.edges);
            costs = add_edge_costs(costs, result.costs);
            total_cost += result.total_cost;
        }
    });
    println!(
        "=== Found path with costs: {:?} and total cost: {} ===",
        costs, total_cost
    );
    DijkstraResult {
        edges,
        costs,
        total_cost,
    }
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
        let mut expected_path: Vec<NodeId>;

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
