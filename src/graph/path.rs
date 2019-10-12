use crate::helpers::{Coordinate, Costs, Preference};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Path {
    pub name: String,
    pub nodes: Vec<usize>,
    pub edges: Vec<usize>,
    pub coordinates: Vec<Coordinate>,
    pub splits: Vec<usize>,
    pub preference: Vec<Preference>,
    pub dim_costs: Costs,
    pub initial_waypoints: Vec<Coordinate>,
    pub initial_pref: Preference,
    pub costs_by_alpha: f64,
}

/*
impl Path {
    pub fn get_total_cost(&self) -> f64 {
        self.costs_by_alpha.iter().fold(0.0, |acc, cost| acc + *cost)
    }
    pub fn get_total_dim_cost(&self) -> [f64; EDGE_COST_DIMENSION] {
        self.dim_costs.iter().fold([0.0; EDGE_COST_DIMENSION], |acc, costs| add_edge_costs(acc, *costs))
    }
}
*/
