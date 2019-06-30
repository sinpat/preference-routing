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
    id: usize,
    source_id: usize,
    target_id: usize,
    edge_costs: [OrderedFloat<f64>; EDGE_COST_DIMENSION],
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

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_source_id(&self) -> usize {
        self.source_id
    }

    pub fn get_target_id(&self) -> usize {
        self.target_id
    }

    pub fn get_edge_costs(&self) -> [OrderedFloat<f64>; EDGE_COST_DIMENSION] {
        self.edge_costs
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
    edge_id: usize,
    target_id: usize,
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

    pub fn get_target_id(&self) -> usize {
        self.target_id
    }

    pub fn get_edge_id(&self) -> usize { self.edge_id }

    pub fn calc_costs(&self) -> OrderedFloat<f64> {
        let mut costs = OrderedFloat(0.0);
        for single_cost in &self.edge_costs {
            costs = add_floats(costs, *single_cost);
        }
        costs
    }
}