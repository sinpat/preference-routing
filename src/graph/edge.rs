use std::convert::TryInto;

use ordered_float::OrderedFloat;

use crate::helpers::add_floats;

use super::EDGE_COST_DIMENSION;

pub fn parse_costs(tokens: &[&str]) -> [OrderedFloat<f64>; EDGE_COST_DIMENSION] {
    let mut edge_costs: [OrderedFloat<f64>; EDGE_COST_DIMENSION] = [OrderedFloat(0.0); EDGE_COST_DIMENSION];
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
    pub edge_costs: [OrderedFloat<f64>; EDGE_COST_DIMENSION],
    repl_edge_1: isize,
    repl_edge_2: isize,
}

impl Edge {
    pub fn new(
        id: usize,
        source_id: usize,
        target_id: usize,
        edge_costs: [OrderedFloat<f64>; EDGE_COST_DIMENSION],
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
    edge_costs: [OrderedFloat<f64>; EDGE_COST_DIMENSION],
}

impl HalfEdge {
    pub fn new(edge_id: usize, target_id: usize, edge_costs: [OrderedFloat<f64>; EDGE_COST_DIMENSION]) -> HalfEdge {
        HalfEdge {
            edge_id,
            target_id,
            edge_costs,
        }
    }

    pub fn calc_costs(&self, alpha: [f64; EDGE_COST_DIMENSION]) -> OrderedFloat<f64> {
        let mut costs = OrderedFloat(0.0);
        for (dim, factor) in alpha.iter().enumerate() {
            let dim_costs = self.edge_costs[dim].0 * *factor;
            costs = add_floats(costs, OrderedFloat(dim_costs))
        }
        costs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}