mod graph;
mod helpers;
mod server;

// const GRAPH_FILE: &str = "/home/patrick/Uni/Bachelor/Bachelorarbeit/data/stuttgart/concGraph";

fn main() {
    /*
    let graph =
        graph::parse_graph_file(GRAPH_FILE)
            .unwrap();
    let dijkstra = graph::dijkstra::Dijkstra::new(&graph);
    */
    server::start_server();
}

/*
fn main() -> Result<(), ParseIntError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("There are parameters missing");
    }
    let graph = graph::parse_graph_file(&args[1]).unwrap();
    let source_id: usize = args[2].parse()?;
    let target_id: usize = args[3].parse()?;
    let mut dijkstra = Dijkstra::new(&graph);
    let find = dijkstra.find_shortest_path(source_id, target_id);
    match find {
        Some(route) => {
            let coords: Vec<[f64; 2]> = route.0.iter().map(|x| [graph.nodes[*x].long, graph.nodes[*x].lat]).collect();
            println!("Path: {:?}", route.0);
            println!("Coords: {:?}", coords);
            println!("Total cost: {:?}", route.1);
        },
        None => println!("No route found")
    };
    Ok(())
}
*/
