use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

pub fn add_floats(a: OrderedFloat<f64>, b: OrderedFloat<f64>) -> OrderedFloat<f64> {
    OrderedFloat(a.0 + b.0)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

impl Coordinate {
    pub fn distance_to(&self, other: &Coordinate) -> OrderedFloat<f64> {
        let distance = (self.lat - other.lat).powi(2) + (self.lng - other.lng).powi(2);
        return OrderedFloat(distance);
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
}