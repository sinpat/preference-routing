use std::io::prelude::*;
use std::env;
use std::num::ParseIntError;
use std::fs::File;
use std::io::BufReader;

mod graph;

fn read_file(file_path: &String) -> Result<graph::Graph, std::io::Error> {
    println!("Parsing graph...");
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    /*
    let mut line = String::new();
    while let len = reader.read_line(&mut line) {
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens[0] == "#" || tokens.len() == 1 {
            println!("Skip");
            println!("{:?}", tokens);
        }
        line.clear();
    }
    */
    for result in reader.lines() {
        let line = result.unwrap();
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens[0] == "#" || tokens.len() == 1 {
            continue;
        }
        // parse nodes and edges
    }
    Ok(graph::Graph {})
}

fn main() -> Result<(), ParseIntError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("There are parameters missing");
    }
    let graph: graph::Graph = match read_file(&args[1]) {
        Ok(graph) => graph,
        Err(e) => panic!("Can not read file")
    };
    let source_id: usize = args[2].parse()?;
    let target_id: usize = args[3].parse()?;
    Ok(())
}
