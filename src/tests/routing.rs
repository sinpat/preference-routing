use crate::graph::{ parse_graph_file, Graph };

fn get_graph() -> Graph {
    parse_graph_file("./src/tests/graph_files/validGraph").unwrap()
}

#[test]
fn from_isolated() {
    let graph = get_graph();
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

}