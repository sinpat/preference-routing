use lp_modeler::operations::LpOperations;
use lp_modeler::problem::{LpObjective, LpProblem};
use lp_modeler::solvers::{GlpkSolver, SolverTrait};
use lp_modeler::variables::{lp_sum, LpExpression, LpInteger};

use crate::{EDGE_COST_DIMENSION, EDGE_COST_TAGS};
use crate::graph::dijkstra::{DijkstraResult, find_path};
use crate::graph::Graph;

pub fn get_preference(graph: &Graph, driven_routes: &[DijkstraResult]) -> Option<[f64; EDGE_COST_DIMENSION]> {
    let mut problem = LpProblem::new("Find Preference", LpObjective::Maximize);
    let solver = GlpkSolver::new();

    // Variables
    let mut variables = Vec::new();
    for tag in &EDGE_COST_TAGS {
        variables.push(LpInteger::new(tag));
    }

    // Objective Function: Maximize zeros
    problem += &variables[1];

    // Constraints
    for var in &variables {
        problem += var.ge(0);
    }
    problem += lp_sum(&variables).equal(1);

    while let Some(alpha) = solve(&problem, &solver) {
        let mut all_explained = true;
        for route in driven_routes {
            let source = route.path[0];
            let target = route.path[route.path.len() - 1];
            let result = find_path(graph, vec![source, target], alpha).unwrap();
            if route.total_cost > result.total_cost {
                all_explained = false;
                println!("Not explained");
                problem += EDGE_COST_TAGS
                    .iter()
                    .enumerate()
                    .fold(LpExpression::LitVal(0.0), |acc, (index, _)| {
                        acc + &variables[index] * ((route.costs[index] - result.costs[index]) as f32)
                    }).le(0);

            }
        }
        if all_explained {
            return Some(alpha);
        }
    }
    None
}

fn solve(problem: &LpProblem, solver: &GlpkSolver) -> Option<[f64; EDGE_COST_DIMENSION]> {
    match solver.run(problem) {
        Ok((status, var_values)) => {
            println!("Status: {:?}", status);
            let mut alpha = [0.0; EDGE_COST_DIMENSION];
            for (name, value) in var_values.iter() {
                println!("value of {}: {}", name, value);
                for (index, tag) in EDGE_COST_TAGS.iter().enumerate() {
                    if name == tag {
                        alpha[index] = f64::from(*value);
                        break;
                    }
                }
            }
            Some(alpha)
        }
        Err(msg) => {
            println!("LpError: {}", msg);
            None
        }
    }
}