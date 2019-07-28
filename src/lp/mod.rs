use lp_modeler::operations::LpOperations;
use lp_modeler::problem::{LpObjective, LpProblem};
use lp_modeler::solvers::{GlpkSolver, SolverTrait};
use lp_modeler::variables::LpInteger;

use crate::EDGE_COST_DIMENSION;
use crate::graph::dijkstra::{DijkstraResult, find_path};
use crate::graph::Graph;

pub fn get_preference(graph: &Graph, driven_routes: &Vec<DijkstraResult>) -> Option<[f64; EDGE_COST_DIMENSION]> {
    let mut problem = LpProblem::new("Find Preference", LpObjective::Minimize);
    let solver = GlpkSolver::new();

    let a = &LpInteger::new("unsuit");
    let b = &LpInteger::new("distance");
    let c = &LpInteger::new("height");

    // Objective Function
    problem += c;

    // Constraints
    problem += (a + b + c).equal(1);
    problem += a.ge(0);
    problem += b.ge(0);
    problem += c.ge(0);

    let mut alpha = solve(&problem, &solver);
    loop {
        if alpha.is_none() {
            break;
        }
        let mut all_explained = true;
        for route in driven_routes {
            let source = route.path[0];
            let target = route.path[route.path.len() - 1];
            let result = find_path(graph, source, target, Vec::new(), alpha.unwrap()).unwrap();
            if route.total_cost > result.total_cost {
                all_explained = false;
                // add strange constraint
            }
        }
        if all_explained {
            break;
        } else {
            alpha = solve(&problem, &solver);
        }
    }
    alpha
}

fn solve(problem: &LpProblem, solver: &GlpkSolver) -> Option<[f64; EDGE_COST_DIMENSION]> {
    match solver.run(problem) {
        Ok((status, var_values)) => {
            println!("Status {:?}", status);
            let mut alpha = [0.0; EDGE_COST_DIMENSION];
            for (index, (name, value)) in var_values.iter().enumerate() {
                println!("value of {} = {}", name, value);
                alpha[index] = f64::from(*value);
            }
            Some(alpha)
        },
        Err(msg) => {
            println!("LpError: {}", msg);
            None
        },
    }
}