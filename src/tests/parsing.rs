use crate::graph::parse_graph_file;

#[test]
fn graph_parsing() {
    let result = parse_graph_file("./src/tests/graph_files/offset_test_graph");
    let graph = result.unwrap();
    assert_eq!(4, graph.get_nodes().len());
    assert_eq!(5, graph.get_edges().len());

    let exp_offsets_out: Vec<usize> = vec![0, 2, 3, 4, 5];
    let exp_offsets_in: Vec<usize> = vec![0, 1, 2, 4, 5];
    assert_eq!(&exp_offsets_out, graph.get_offsets_out());
    assert_eq!(&exp_offsets_in, graph.get_offsets_in());
}