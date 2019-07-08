mod graph;
mod helpers;
mod server;

const GRAPH_FILE: &str = "/home/patrick/Uni/Bachelor/Bachelorarbeit/data/stuttgart/concGraph";

fn main() {
    let graph =
        graph::parse_graph_file(GRAPH_FILE)
            .unwrap();
    server::start_server(graph);
}
