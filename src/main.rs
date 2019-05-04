use std::env;
use std::num::ParseIntError;

mod graph;
mod dijkstra;
mod tests;

fn main() -> Result<(), ParseIntError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("There are parameters missing");
    }
    let graph = graph::parse_graph_file(&args[1]).unwrap();
    let source_id: usize = args[2].parse()?;
    let target_id: usize = args[3].parse()?;
    let find = dijkstra::find_shortest_path(&graph, source_id, target_id);
    match find {
        Some(route) => {
            println!("{:?}", route.0);
            println!("Total cost: {:?}", route.1);
        },
        None => println!("No route found")
    };
    Ok(())
}
