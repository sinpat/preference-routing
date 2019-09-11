use serde::{Deserialize, Serialize};

use crate::EDGE_COST_DIMENSION;

pub type Preference = [f64; EDGE_COST_DIMENSION];
pub type Costs = [f64; EDGE_COST_DIMENSION];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

impl Coordinate {
    pub fn distance_to(&self, other: &Coordinate) -> f64 {
        ((self.lat - other.lat).powi(2) + (self.lng - other.lng).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to() {
        let a = Coordinate { lat: 5.0, lng: 7.0 };
        let b = Coordinate { lat: 2.0, lng: 3.0 };
        assert_eq!(a.distance_to(&b), 5.0);
    }
}
