use std::collections::binary_heap::BinaryHeap;

use ordered_float::OrderedFloat;
use serde::Serialize;

use state::Direction::{BACKWARD, FORWARD};
use state::State;

use crate::EDGE_COST_DIMENSION;
use crate::graph::Graph;
use crate::helpers::{add_floats, Coordinate};

use super::edge::add_edge_costs;
use super::node::HalfNode;

pub mod state;

#[derive(Debug, Serialize)]
pub struct DijkstraResult {
    pub path: Vec<HalfNode>,
    pub costs: [f64; EDGE_COST_DIMENSION],
    pub total_cost: f64,
}

pub struct Dijkstra<'a> {
    graph: &'a Graph,
    candidates: BinaryHeap<State>,

    // Best dist(-vector) to/from node
    to_dist: Vec<([f64; EDGE_COST_DIMENSION], OrderedFloat<f64>)>,
    from_dist: Vec<([f64; EDGE_COST_DIMENSION], OrderedFloat<f64>)>,

    // Vec of (node_id, edge_id)
    previous: Vec<Option<(usize, usize)>>,
    successive: Vec<Option<(usize, usize)>>,

    // (node_id, cost array, total_cost)
    best_node: (Option<usize>, [f64; EDGE_COST_DIMENSION], OrderedFloat<f64>),

    // Contains all the information about the nodes
    // state: Vec<NodeState>,
}

impl<'a> Dijkstra<'a> {
    fn new(graph: &Graph) -> Dijkstra {
        let num_of_nodes = graph.nodes.len();
        Dijkstra {
            graph,
            candidates: BinaryHeap::new(),
            to_dist: vec![([0.0, 0.0, 0.0], OrderedFloat(std::f64::MAX)); num_of_nodes],
            from_dist: vec![([0.0, 0.0, 0.0], OrderedFloat(std::f64::MAX)); num_of_nodes],
            previous: vec![None; num_of_nodes],
            successive: vec![None; num_of_nodes],
            best_node: (None, [0.0; EDGE_COST_DIMENSION], OrderedFloat(std::f64::MAX)),
            /*
            state: vec![NodeState {
                to_dist: OrderedFloat(std::f64::MAX),
                from_dist: OrderedFloat(std::f64::MAX),
                previous: None,
                successive: None,
            }; num_of_nodes],
            */
        }
    }

    fn run(
        &mut self,
        source: usize,
        target: usize,
        _avoid: &Vec<usize>,
        alpha: [f64; EDGE_COST_DIMENSION],
    ) -> Option<DijkstraResult> {
        // Preparations
        self.candidates.push(State { node_id: source, costs: [0.0, 0.0, 0.0], total_cost: OrderedFloat(0.0), direction: FORWARD });
        self.candidates.push(State { node_id: target, costs: [0.0, 0.0, 0.0], total_cost: OrderedFloat(0.0), direction: BACKWARD });
        self.to_dist[source].1 = OrderedFloat(0.0);
        self.from_dist[target].1 = OrderedFloat(0.0);
        /*
        self.state[source].to_dist = OrderedFloat(0.0);
        self.state[target].from_dist = OrderedFloat(0.0);
        */

        while let Some(candidate) = self.candidates.pop() {
            self.process_state(candidate, alpha);
        }
        match self.best_node {
            // TODO: Think about wrapping the whole tuple in an option, not just the node_id
            (None, _, _) => None,
            (Some(node_id), costs, total_cost) => {
                println!("Found node {:?} with cost {:?}", node_id, total_cost);
                let path = self.construct_path(node_id, source)
                    .iter()
                    .map(|id| HalfNode { id: *id, location: Coordinate {
                        lat: self.graph.nodes[*id].location.lat,
                        lng: self.graph.nodes[*id].location.lng,
                    } })
                    .collect();
                Some(DijkstraResult {
                    path,
                    costs,
                    total_cost: total_cost.into_inner(),
                })
            }
        }
    }

    fn process_state(&mut self, candidate: State, alpha: [f64; EDGE_COST_DIMENSION]) {
        let State { node_id, costs, total_cost, direction } = candidate;
        if direction == FORWARD {
            if total_cost > self.to_dist[node_id].1 {
                return;
            }
            let merged_cost_vector = add_edge_costs(costs, self.from_dist[node_id].0);
            let merged_cost = add_floats(total_cost, self.from_dist[node_id].1);
            if merged_cost < self.best_node.2 {
                self.best_node = (Some(node_id), merged_cost_vector, merged_cost);
            }
            for half_edge in self.graph.get_ch_edges_out(node_id) {
                let next = State {
                    node_id: half_edge.target_id,
                    costs: add_edge_costs(costs, half_edge.edge_costs),
                    total_cost: add_floats(total_cost, half_edge.calc_costs(alpha)),
                    direction,
                };
                if next.total_cost < self.to_dist[next.node_id].1 {
                    self.to_dist[next.node_id] = (next.costs, next.total_cost);
                    self.previous[next.node_id] = Some((node_id, half_edge.edge_id));
                    self.candidates.push(next);
                }
            }
        }
        if direction == BACKWARD {
            if total_cost > self.from_dist[node_id].1 {
                return;
            }
            let merged_cost_vector = add_edge_costs(costs, self.to_dist[node_id].0);
            let merged_cost = add_floats(total_cost, self.to_dist[node_id].1);
            if merged_cost < self.best_node.2 {
                self.best_node = (Some(node_id), merged_cost_vector, merged_cost);
            }
            for half_edge in self.graph.get_ch_edges_in(node_id) {
                let next = State {
                    node_id: half_edge.target_id,
                    costs: add_edge_costs(costs, half_edge.edge_costs),
                    total_cost: add_floats(total_cost, half_edge.calc_costs(alpha)),
                    direction,
                };
                if next.total_cost < self.from_dist[next.node_id].1 {
                    self.from_dist[next.node_id] = (next.costs, next.total_cost);
                    self.successive[next.node_id] = Some((node_id, half_edge.edge_id));
                    self.candidates.push(next);
                }
            }
        }
    }

    fn construct_path(&self, node_id: usize, source: usize) -> Vec<usize> {
        let mut path = Vec::new();
        let mut node_and_edge = self.previous[node_id];
        while let Some((current_node_id, edge_id)) = node_and_edge {
            node_and_edge = self.previous[current_node_id];
            path.push(edge_id);
        }
        path.reverse();
        node_and_edge = self.successive[node_id];
        while let Some((current_node_id, edge_id)) = node_and_edge {
            node_and_edge = self.successive[current_node_id];
            path.push(edge_id);
        }
        self.graph.unwrap_edges(path, source)
    }
}

pub fn find_path(
    graph: &Graph,
    nodes: Vec<usize>,
    avoid: Vec<usize>,
    alpha: [f64; EDGE_COST_DIMENSION],
) -> Vec<Option<DijkstraResult>> {
    println!("Running Dijkstra search...");
    let mut results = Vec::new();
    for index in 0..nodes.len() - 1 {
        let mut dijkstra = Dijkstra::new(graph);
        let output = dijkstra.run(nodes[index], nodes[index + 1], &avoid, alpha);
        results.push(output);
    };
    println!("Done");
    results
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