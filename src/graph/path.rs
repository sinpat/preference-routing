use crate::graph::Graph;
use crate::helpers::{add_edge_costs, Coordinate, Costs, Preference};
use crate::EDGE_COST_DIMENSION;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PathSplit {
    pub cuts: Vec<usize>,
    pub alphas: Vec<Preference>,
    pub dimension_costs: Vec<Costs>,
    pub costs_by_alpha: Vec<f64>,
}

impl PathSplit {
    pub fn get_total_cost(&self) -> f64 {
        self.costs_by_alpha
            .iter()
            .fold(0.0, |acc, cost| acc + *cost)
    }
    pub fn get_total_dimension_costs(&self) -> [f64; EDGE_COST_DIMENSION] {
        self.dimension_costs
            .iter()
            .fold([0.0; EDGE_COST_DIMENSION], |acc, costs| {
                add_edge_costs(acc, *costs)
            })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Path {
    pub id: usize,
    pub name: String,
    pub nodes: Vec<usize>,
    pub edges: Vec<usize>,
    pub waypoints: Vec<Coordinate>,
    pub coordinates: Vec<Coordinate>,
    pub user_split: PathSplit,
    pub algo_split: Option<PathSplit>,
    /*
    pub splits: Vec<usize>,
    pub preference: Vec<Preference>,
    pub dim_costs: Costs,
    pub initial_waypoints: Vec<Coordinate>,
    pub initial_pref: Preference,
    pub costs_by_alpha: f64,
    */
}

impl Path {
    pub fn get_subpath_costs(&self, graph: &Graph, start: usize, end: usize) -> Costs {
        let edges = &self.edges[start..end];
        edges.iter().fold([0.0; EDGE_COST_DIMENSION], |acc, edge| {
            add_edge_costs(acc, graph.edges[*edge].edge_costs)
        })
    }
}
