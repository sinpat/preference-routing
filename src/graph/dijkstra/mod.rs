use std::collections::binary_heap::BinaryHeap;
use std::time::Instant;

use state::Direction::{BACKWARD, FORWARD};
use state::State;

use crate::graph::Graph;
use crate::helpers::{add_edge_costs, costs_by_alpha, Costs, Preference};
use crate::EDGE_COST_DIMENSION;

mod state;

pub struct DijkstraResult {
    pub edges: Vec<usize>,
    pub costs: Costs,
    pub total_cost: f64,
}

struct Dijkstra<'a> {
    graph: &'a Graph,
    candidates: BinaryHeap<State>,
    touched_nodes: Vec<usize>,
    found_best_b: bool,
    found_best_f: bool,

    // Best dist to/from node
    pub cost_f: Vec<(Costs, f64)>,
    pub cost_b: Vec<(Costs, f64)>,

    // Best (node, edge) to/from node
    pub previous_f: Vec<Option<(usize, usize)>>,
    pub previous_b: Vec<Option<(usize, usize)>>,

    // (node_id, cost array, total_cost)
    best_node: (Option<usize>, Costs, f64),
}

impl<'a> Dijkstra<'a> {
    fn new(graph: &Graph) -> Dijkstra {
        let num_of_nodes = graph.nodes.len();
        Dijkstra {
            graph,
            candidates: BinaryHeap::new(),
            touched_nodes: Vec::new(),
            found_best_b: false,
            found_best_f: false,
            cost_f: vec![([0.0; EDGE_COST_DIMENSION], std::f64::MAX); num_of_nodes],
            cost_b: vec![([0.0; EDGE_COST_DIMENSION], std::f64::MAX); num_of_nodes],
            previous_f: vec![None; num_of_nodes],
            previous_b: vec![None; num_of_nodes],
            best_node: (None, [0.0; EDGE_COST_DIMENSION], std::f64::MAX),
        }
    }

    fn prepare(&mut self, source: usize, target: usize) {
        // Candidates
        self.candidates = BinaryHeap::new();
        self.candidates.push(State::new(source, FORWARD));
        self.candidates.push(State::new(target, BACKWARD));

        // Touched nodes
        for node_id in &self.touched_nodes {
            self.cost_f[*node_id] = ([0.0; EDGE_COST_DIMENSION], std::f64::MAX);
            self.cost_b[*node_id] = ([0.0; EDGE_COST_DIMENSION], std::f64::MAX);
            self.previous_f[*node_id] = None;
            self.previous_b[*node_id] = None;
        }
        self.touched_nodes.clear();

        self.found_best_b = false;
        self.found_best_f = false;

        // Node states
        self.cost_f[source].1 = 0.0;
        self.cost_b[target].1 = 0.0;
        self.touched_nodes.push(source);
        self.touched_nodes.push(target);

        // Best node
        self.best_node = (None, [0.0; EDGE_COST_DIMENSION], std::f64::MAX);
    }

    fn run(&mut self, source: usize, target: usize, alpha: Preference) -> Option<DijkstraResult> {
        self.prepare(source, target);

        let now = Instant::now();
        while let Some(candidate) = self.candidates.pop() {
            if self.found_best_f && self.found_best_b {
                break;
            }
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
                let edges = self.make_edge_path(node_id);
                Some(DijkstraResult {
                    edges,
                    costs,
                    total_cost,
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

        let my_costs;
        let other_costs;
        let found_best;
        let previous;
        if direction == FORWARD {
            my_costs = &mut self.cost_f;
            other_costs = &self.cost_b;
            found_best = &mut self.found_best_f;
            previous = &mut self.previous_f;
        } else {
            my_costs = &mut self.cost_b;
            other_costs = &self.cost_f;
            found_best = &mut self.found_best_b;
            previous = &mut self.previous_b;
        };

        if total_cost > my_costs[node_id].1 {
            return;
        };
        if total_cost > self.best_node.2 {
            *found_best = true;
            return;
        }
        if other_costs[node_id].1 != std::f64::MAX {
            let merged_cost = total_cost + other_costs[node_id].1;
            if merged_cost < self.best_node.2 {
                let merged_cost_vector = add_edge_costs(costs, other_costs[node_id].0);
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
            let next_total_cost = total_cost + costs_by_alpha(half_edge.edge_costs, alpha);

            if next_total_cost < my_costs[next_node].1 {
                my_costs[next_node] = (next_costs, next_total_cost);
                previous[next_node] = Some((node_id, half_edge.edge_id));
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

    fn make_edge_path(&self, connector: usize) -> Vec<usize> {
        let mut edges = Vec::new();
        let mut previous_state = self.previous_f[connector];
        let mut successive_state = self.previous_b[connector];

        // backwards
        while let Some((previous_node, edge_id)) = previous_state {
            edges.push(edge_id);
            previous_state = self.previous_f[previous_node];
        }
        edges.reverse();

        // forwards
        while let Some((successive_node, edge_id)) = successive_state {
            edges.push(edge_id);
            successive_state = self.previous_b[successive_node];
        }
        edges
    }
}

pub fn find_path(graph: &Graph, include: &[usize], alpha: Preference) -> Option<DijkstraResult> {
    println!("=== Running Dijkstra search ===");
    let mut dijkstra = Dijkstra::new(graph);
    let mut edges = Vec::new();
    let mut costs = [0.0; EDGE_COST_DIMENSION];
    let mut total_cost = 0.0;

    for win in include.windows(2) {
        if let Some(mut result) = dijkstra.run(win[0], win[1], alpha) {
            edges.append(&mut result.edges);
            costs = add_edge_costs(costs, result.costs);
            total_cost += result.total_cost;
        } else {
            println!("=== Could not find a route ===");
            return None;
        }
    }

    println!(
        "=== Found path with costs: {:?} and total cost: {} ===",
        costs, total_cost
    );
    Some(DijkstraResult {
        edges,
        costs,
        total_cost,
    })
}

#[cfg(test)]
mod tests {
    use crate::graph::{parse_graph_file, Graph};

    use super::*;

    fn get_graph() -> Graph {
        parse_graph_file("./src/test_graphs/testGraph").unwrap()
    }

    fn get_conc_graph() -> Graph {
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
        let conc_graph = get_conc_graph();
        let mut dijkstra = Dijkstra::new(&graph);
        let mut dijkstra_conc = Dijkstra::new(&conc_graph);
        let alpha = [0.0, 1.0, 0.0];

        let mut shortest_path;
        let mut shortest_path_conc;
        let mut path;
        let mut path_conc;

        // first query
        assert!(dijkstra.run(0, 4, alpha).is_none());
        assert!(dijkstra_conc.run(0, 4, alpha).is_none());

        // second query
        assert!(dijkstra.run(4, 11, alpha).is_none());
        assert!(dijkstra_conc.run(4, 11, alpha).is_none());

        // third query
        shortest_path = dijkstra.run(2, 5, alpha);
        shortest_path_conc = dijkstra_conc.run(2, 5, alpha);
        assert!(shortest_path.is_some());
        assert!(shortest_path_conc.is_some());

        path = shortest_path.unwrap();
        path_conc = shortest_path_conc.unwrap();
        assert_eq!(path.edges, vec![4, 7]);
        assert_eq!(path.total_cost, 2.0);
        assert_eq!(path_conc.edges, vec![4, 7]);
        assert_eq!(path_conc.total_cost, 2.0);

        // fourth query
        shortest_path = dijkstra.run(2, 10, alpha);
        shortest_path_conc = dijkstra_conc.run(2, 10, alpha);
        assert!(shortest_path.is_some());
        assert!(shortest_path_conc.is_some());

        path = shortest_path.unwrap();
        path_conc = shortest_path_conc.unwrap();
        assert_eq!(path.edges, vec![4, 7, 9, 12]);
        assert_eq!(path.total_cost, 4.0);
        assert_eq!(path_conc.edges, vec![4, 21]);
        assert_eq!(path_conc.total_cost, 4.0);

        // fifth query
        shortest_path = dijkstra.run(4, 10, alpha);
        shortest_path_conc = dijkstra_conc.run(4, 10, alpha);
        assert!(shortest_path.is_some());
        assert!(shortest_path_conc.is_some());

        path = shortest_path.unwrap();
        path_conc = shortest_path_conc.unwrap();
        assert_eq!(path.edges, vec![7, 9, 12]);
        assert_eq!(path.total_cost, 3.0);
        assert_eq!(path_conc.edges, vec![21]);
        assert_eq!(path_conc.total_cost, 3.0);
    }
}
