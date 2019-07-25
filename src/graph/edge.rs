use std::convert::TryInto;

use ordered_float::OrderedFloat;

use super::EDGE_COST_DIMENSION;

pub fn parse_costs(tokens: &[&str]) -> [f64; EDGE_COST_DIMENSION] {
    let mut edge_costs: [f64; EDGE_COST_DIMENSION] = [0.0; EDGE_COST_DIMENSION];
    for (index, token) in tokens.iter().enumerate() {
        edge_costs[index] = token.parse().unwrap();
    }
    edge_costs
}

#[derive(Debug)]
pub struct Edge {
    pub id: usize,
    pub source_id: usize,
    pub target_id: usize,
    pub edge_costs: [f64; EDGE_COST_DIMENSION],
    repl_edge_1: isize,
    repl_edge_2: isize,
}

impl Edge {
    pub fn new(
        id: usize,
        source_id: usize,
        target_id: usize,
        edge_costs: [f64; EDGE_COST_DIMENSION],
        repl_edge_1: isize,
        repl_edge_2: isize,
    ) -> Edge {
        Edge { id, source_id, target_id, edge_costs, repl_edge_1, repl_edge_2 }
    }

    pub fn get_replaced_edges(&self) -> Option<(usize, usize)> {
        if self.repl_edge_1 == -1 {
            return None;
        }
        Some((self.repl_edge_1.try_into().unwrap(), self.repl_edge_2.try_into().unwrap()))
    }
}

#[derive(Debug)]
pub struct HalfEdge {
    pub edge_id: usize,
    pub target_id: usize,
    pub edge_costs: [f64; EDGE_COST_DIMENSION],
}

impl HalfEdge {
    pub fn new(edge_id: usize, target_id: usize, edge_costs: [f64; EDGE_COST_DIMENSION]) -> HalfEdge {
        HalfEdge {
            edge_id,
            target_id,
            edge_costs,
        }
    }

    pub fn calc_costs(&self, alpha: [f64; EDGE_COST_DIMENSION]) -> OrderedFloat<f64> {
        self.edge_costs
            .iter()
            .zip(alpha.iter())
            .fold(0.0, |acc, (cost, factor)| acc + cost * *factor)
            .into()
    }
}

pub fn add_edge_costs(a: [f64; EDGE_COST_DIMENSION], b: [f64; EDGE_COST_DIMENSION]) -> [f64; EDGE_COST_DIMENSION] {
    let mut result = [0.0; EDGE_COST_DIMENSION];
    a.iter()
        .zip(b.iter())
        .enumerate()
        .for_each(|(index, (first, second))| result[index] = first + second);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge_costs() {
        let a = [1.5, 2.0, 0.7];
        let b = [1.3, 0.1, 0.3];
        let result = add_edge_costs(a, b);
        assert_eq!([2.8, 2.1, 1.0], result);
    }
}