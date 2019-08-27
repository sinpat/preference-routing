use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

use crate::EDGE_COST_DIMENSION;

pub type Preference = [f64; EDGE_COST_DIMENSION];
pub type Costs = [f64; EDGE_COST_DIMENSION];

pub fn add_floats(a: OrderedFloat<f64>, b: OrderedFloat<f64>) -> OrderedFloat<f64> {
    OrderedFloat(a.0 + b.0)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

impl Coordinate {
    pub fn distance_to(&self, other: &Coordinate) -> OrderedFloat<f64> {
        let distance = ((self.lat - other.lat).powi(2) + (self.lng - other.lng).powi(2)).sqrt();
        OrderedFloat(distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_floats() {
        let a = OrderedFloat(1.0);
        let b = OrderedFloat(2.0);
        assert_eq!(add_floats(a, b), OrderedFloat(3.0));
    }

    #[test]
    fn test_distance_to() {
        let a = Coordinate { lat: 5.0, lng: 7.0 };
        let b = Coordinate { lat: 2.0, lng: 3.0 };
        assert_eq!(a.distance_to(&b), OrderedFloat(5.0));
    }
}
