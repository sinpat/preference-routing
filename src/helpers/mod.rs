use ordered_float::OrderedFloat;

pub fn add_floats(a: OrderedFloat<f64>, b: OrderedFloat<f64>) -> OrderedFloat<f64> {
    OrderedFloat(a.0 + b.0)
}