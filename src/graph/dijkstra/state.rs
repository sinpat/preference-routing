use std::cmp::Ordering;

use ordered_float::OrderedFloat;

use crate::graph::NodeId;
use crate::helpers::Costs;
use crate::EDGE_COST_DIMENSION;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    FORWARD,
    BACKWARD,
}

#[derive(PartialEq)]
pub struct State {
    pub node_id: NodeId,
    // costs of the different metrics
    pub costs: Costs,
    // cost including alpha
    pub total_cost: OrderedFloat<f64>,
    pub direction: Direction,
}

impl State {
    pub fn new(node_id: NodeId, direction: Direction) -> Self {
        State {
            node_id,
            costs: [0.0; EDGE_COST_DIMENSION],
            total_cost: OrderedFloat(0.0),
            direction,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
}
