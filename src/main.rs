mod graph;
mod helpers;
mod server;

const GRAPH_FILE: &str = "/home/patrick/Uni/Bachelor/Bachelorarbeit/data/ba-wue/concGraph";

// BicycleUnsuitability, Distance, HeightAscent
const EDGE_COST_DIMENSION: usize = 3;

fn main() {
    let graph =
        graph::parse_graph_file(GRAPH_FILE)
            .unwrap();
    server::start_server(graph);
}
