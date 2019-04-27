struct Node {
    id: usize,
    lat: f64,
    long: f64,
    height: f64,
    ch_level: usize
}

struct HalfEdge {
    target_id: usize,
    edge_costs: [f64; 3]
}

struct Edge {
    source_id: usize,
    target_id: usize,
    edge_costs: [f64; 3],
    repl_edge_1: usize,
    repl_edge_2: usize
}

pub struct Graph {
    /*
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    half_edges_in: Vec<HalfEdge>,
    half_edges_out: Vec<HalfEdge>,
    offsets_in: Vec<usize>,
    offsets_out: Vec<usize>
    */
}