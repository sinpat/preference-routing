use std::cmp::Ordering;

use ordered_float::OrderedFloat;

use crate::helpers::Costs;
use crate::EDGE_COST_DIMENSION;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    FORWARD,
    BACKWARD,
}

#[derive(PartialEq)]
pub struct State {
    pub node_id: usize,
    // costs of the different metrics
    pub costs: Costs,
    // cost including alpha
    pub total_cost: OrderedFloat<f64>,
    pub direction: Direction,
}

impl std::cmp::Eq for State {}

impl std::cmp::Ord for State {
    // switch comparison, because we want a min-heap
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_cost.cmp(&self.total_cost)
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
pub struct NodeState {
    // Best dist to/from node
    pub to_dist: (Costs, OrderedFloat<f64>),
    pub from_dist: (Costs, OrderedFloat<f64>),

    // Best (node, edge) to/from node
    pub previous: Option<(usize, usize)>,
    pub successive: Option<(usize, usize)>,
}

impl NodeState {
    pub fn new() -> Self {
        NodeState {
            to_dist: ([0.0; EDGE_COST_DIMENSION], OrderedFloat(std::f64::MAX)),
            from_dist: ([0.0; EDGE_COST_DIMENSION], OrderedFloat(std::f64::MAX)),
            previous: None,
            successive: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
