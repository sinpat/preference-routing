use crate::graph::{ parse_graph_file, Graph };
use crate::graph::dijkstra::Dijkstra;
use super::{ TEST_GRAPH, CONCAT_TEST_GRAPH };
use ordered_float::OrderedFloat;

fn get_graph() -> Graph {
    parse_graph_file(CONCAT_TEST_GRAPH).unwrap()
}

#[test]
fn from_isolated() {
}

#[test]
fn to_isolated() {

}

#[test]
fn to_one_way() {

}

#[test]
fn from_one_way() {

}

#[test]
fn normal_case() {
    let graph = get_graph();
    let mut dijkstra = Dijkstra::new(&graph);
    let mut shortest_path;
    let mut path;
    let mut expected_path: Vec<usize>;

    // first query
    shortest_path = dijkstra.find_shortest_path(0, 4);
    assert!(shortest_path.is_none());

    // second query
    shortest_path = dijkstra.find_shortest_path(4, 11);
    assert!(shortest_path.is_none());

    // third query
    shortest_path = dijkstra.find_shortest_path(2, 5);
    assert!(shortest_path.is_some());
    path = shortest_path.unwrap();
    expected_path = vec![2, 4, 5];
    assert_eq!(expected_path, path.0);
    assert_eq!(OrderedFloat(4.0), path.1);

    // fourth query
    shortest_path = dijkstra.find_shortest_path(2, 10);
    assert!(shortest_path.is_some());
    path = shortest_path.unwrap();
    expected_path = vec![2, 4, 5, 7, 10];
    assert_eq!(expected_path, path.0);
    assert_eq!(OrderedFloat(8.0), path.1);

    // fifth query
    shortest_path = dijkstra.find_shortest_path(6, 10);
    assert!(shortest_path.is_some());
    path = shortest_path.unwrap();
    expected_path = vec![6, 4, 5, 7, 10];
    assert_eq!(expected_path, path.0);
    assert_eq!(OrderedFloat(10.0), path.1);
}