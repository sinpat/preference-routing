use ordered_float::OrderedFloat;

const COST_DIMENSION: usize = 1; // doubled in graph/mod!

#[derive(Debug)]
pub struct Edge {
    source_id: usize,
    target_id: usize,
    edge_costs: [OrderedFloat<f64>; COST_DIMENSION],
    repl_edge_1: isize,
    repl_edge_2: isize
}

impl Edge {
    pub fn new(source_id: usize, target_id: usize, edge_costs: [OrderedFloat<f64>; COST_DIMENSION], repl_edge_1: isize, repl_edge_2: isize) -> Edge {
        Edge { source_id, target_id, edge_costs, repl_edge_1, repl_edge_2 }
    }

    pub fn get_source_id(&self) -> usize {
        self.source_id
    }

    pub fn get_target_id(&self) -> usize {
        self.target_id
    }
}

#[derive(Debug)]
pub struct HalfEdge {
    target_id: usize,
    edge_costs: [OrderedFloat<f64>; COST_DIMENSION]
}