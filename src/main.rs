mod graph;
mod helpers;
mod server;

const GRAPH_FILE: &str = "/home/patrick/Uni/Bachelor/Bachelorarbeit/data/ba-wue/concGraph";

const EDGE_COST_DIMENSION: usize = 3;
const EDGE_COST_TAGS: [&str; EDGE_COST_DIMENSION] = ["BicycleUnsuitability", "Distance", "HeightAscent"];

fn main() {
    let graph =
        graph::parse_graph_file(GRAPH_FILE)
            .unwrap();
    server::start_server(graph);
}
