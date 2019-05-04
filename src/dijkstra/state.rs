use std::cmp::Ordering;
use ordered_float::OrderedFloat;

#[derive(PartialEq)]
pub struct State {
    pub node_id: usize,
    pub cost: OrderedFloat<f64>,
    pub forward: bool
}

impl std::cmp::Eq for State {}

impl std::cmp::Ord for State {
    // switch comparison, because we want a min-heap
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}