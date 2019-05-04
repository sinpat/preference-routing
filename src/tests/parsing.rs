#[test]
fn graph_parsing() {
    use crate::graph::*;
    let result = parse_graph_file("./src/tests/testGraph");
    let graph = result.unwrap();
    assert_eq!(graph.get_nodes().len(), 16);
    assert_eq!(graph.get_edges().len(), 21);
}