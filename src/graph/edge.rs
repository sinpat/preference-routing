use ordered_float::OrderedFloat;

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
    repl_edge_2: isize
}

impl Edge {
    pub fn new(id: usize, source_id: usize, target_id: usize, edge_costs: [OrderedFloat<f64>; EDGE_COST_DIMENSION], repl_edge_1: isize, repl_edge_2: isize) -> Edge {
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

    pub fn calc_costs(&self) -> OrderedFloat<f64> {
        let mut costs = OrderedFloat(0.0);
        for single_cost in &self.edge_costs {
            costs = OrderedFloat(costs.0 + single_cost.0);
        }
        costs
    }
}

#[derive(Debug)]
pub struct HalfEdge {
    target_id: usize,
    edge_costs: [OrderedFloat<f64>; EDGE_COST_DIMENSION]
}

impl HalfEdge {
    pub fn new(target_id: usize, edge_costs: [OrderedFloat<f64>; EDGE_COST_DIMENSION]) -> HalfEdge {
        HalfEdge {
            target_id,
            edge_costs
        }
    }

    pub fn get_target_id(&self) -> usize {
        self.target_id
    }

    pub fn calc_costs(&self) -> OrderedFloat<f64> {
        let mut costs = OrderedFloat(0.0);
        for single_cost in &self.edge_costs {
            costs = OrderedFloat(costs.0 + single_cost.0);
        }
        costs
    }
}