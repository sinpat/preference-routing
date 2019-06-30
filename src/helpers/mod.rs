use ordered_float::OrderedFloat;

pub fn add_floats(a: OrderedFloat<f64>, b: OrderedFloat<f64>) -> OrderedFloat<f64> {
    OrderedFloat(a.0 + b.0)
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