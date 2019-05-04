use crate::graph::{ parse_graph_file, Graph };
use crate::dijkstra::Dijkstra;

fn get_graph() -> Graph {
    parse_graph_file("./src/tests/graph_files/offset_test_graph").unwrap()
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
    // first query
    let mut shortest_path = dijkstra.find_shortest_path(0, 1);
    assert!(shortest_path.is_some());
    let mut path = shortest_path.unwrap().0;
    let mut exp_path: Vec<usize> = vec![0, 1].into();
    assert_eq!(exp_path, path);

    // second query
    shortest_path = dijkstra.find_shortest_path(1, 0);
    assert!(shortest_path.is_some());
    path = shortest_path.unwrap().0;
    exp_path = vec![1, 2, 3, 0];
    assert_eq!(exp_path, path);
}