use std::env;
use std::num::ParseIntError;

mod graph;
mod dijkstra;

fn main() -> Result<(), ParseIntError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("There are parameters missing");
    }
    println!("Parsing graph...");
    let graph = match graph::parse_graph_file(&args[1]) {
        Ok(graph) => graph,
        Err(e) => panic!(e)
    };
    let source_id: usize = args[2].parse()?;
    let target_id: usize = args[3].parse()?;
    let find = dijkstra::find_shortest_path(graph, source_id, target_id);
    match find {
        Some(route) => {
            for node in route.0 {
                println!("{:?}", node);
            }
            println!("Total cost: {:?}", route.1);
        },
        None => println!("No route found")
    };
    Ok(())
}
