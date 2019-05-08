use crate::graph::parse_graph_file;
use super::{ TEST_GRAPH, CONCAT_TEST_GRAPH };

#[test]
fn graph_parsing() {
    let result = parse_graph_file(TEST_GRAPH);
    let graph = result.unwrap();
    assert_eq!(12, graph.get_nodes().len());
    assert_eq!(18, graph.get_edges().len());

    let exp_offsets_out: Vec<usize> = vec![0, 0, 2, 6, 7, 9, 10, 12, 13, 15, 17, 18, 18];
    let exp_offsets_in: Vec<usize> = vec![0, 1, 2, 3, 4, 6, 8, 11, 12, 14, 16, 18, 18];
    assert_eq!(&exp_offsets_out, graph.get_offsets_out());
    assert_eq!(&exp_offsets_in, graph.get_offsets_in());
}