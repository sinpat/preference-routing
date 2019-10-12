use crate::graph::{parse_graph_file, Graph};
use rand::seq::SliceRandom;
use crate::INITIAL_PREF;
use crate::lp::find_preference;

fn get_rnd_nodes(graph: &Graph, amount: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    (0..amount).map(|_| graph.nodes.choose(&mut rng).unwrap().id).collect()
}

#[test]
fn test_three() {
    let graph = parse_graph_file("./src/test_graphs/concTestGraph").unwrap();
    for _ in 0..100 {
        let path = graph.find_shortest_path(get_rnd_nodes(&graph, 3), INITIAL_PREF);
        if let Some(path) = path {
            let pref = find_preference(&graph, &path);
            dbg!(pref);
        }
    }
}