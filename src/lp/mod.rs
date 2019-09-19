use lp_modeler::operations::LpOperations;
use lp_modeler::problem::{LpFileFormat, LpObjective, LpProblem};
use lp_modeler::solvers::{GlpkSolver, SolverTrait};
use lp_modeler::variables::{lp_sum, LpContinuous, LpExpression};

use crate::graph::Graph;
use crate::graph::Path;
use crate::helpers::{costs_by_alpha, Preference};
use crate::{EDGE_COST_DIMENSION, EDGE_COST_TAGS};

// TODO: Remove this struct?
pub struct PreferenceEstimator {
    problem: LpProblem,
    variables: Vec<LpContinuous>,
    deltas: Vec<LpContinuous>,
    solver: GlpkSolver,
}

impl PreferenceEstimator {
    pub fn new() -> Self {
        let mut problem = LpProblem::new("Find Preference", LpObjective::Maximize);

        // Variables
        let mut variables = Vec::new();
        for tag in &EDGE_COST_TAGS {
            variables.push(LpContinuous::new(tag));
        }
        let deltas = Vec::new();

        // Constraints
        for var in &variables {
            problem += var.ge(0);
        }
        problem += lp_sum(&variables).equal(1);

        PreferenceEstimator {
            problem,
            variables,
            deltas,
            solver: GlpkSolver::new(),
        }
    }
    pub fn get_preference(
        &mut self,
        graph: &Graph,
        driven_routes: &[Path],
        alpha_in: Preference,
    ) -> Option<Preference> {
        let current_feasible = self.check_feasibility(graph, driven_routes, alpha_in);
        if current_feasible {
            return Some(alpha_in);
        }
        while let Some(alpha) = self.solve() {
            let feasible = self.check_feasibility(graph, driven_routes, alpha);
            if feasible {
                return Some(alpha);
            }
        }
        None
    }

    fn check_feasibility(
        &mut self,
        graph: &Graph,
        driven_routes: &[Path],
        alpha: Preference,
    ) -> bool {
        let mut all_explained = true;
        for route in driven_routes {
            let source = route.coordinates[0].clone();
            let target = route.coordinates[route.coordinates.len() - 1].clone();
            let result = graph.find_shortest_path(vec![source, target], alpha);
            if route.nodes == result.nodes {
                println!("Paths are equal, proceed with next route");
            } else if costs_by_alpha(route.costs, alpha) > result.total_cost {
                all_explained = false;
                println!(
                    "Not explained, {} > {}",
                    costs_by_alpha(route.costs, alpha),
                    result.total_cost
                );
                let new_delta = LpContinuous::new(&format!("delta{}", self.deltas.len()));
                self.problem += new_delta.ge(0);
                self.problem += new_delta.clone();
                self.deltas.push(new_delta.clone());
                self.problem += (0..EDGE_COST_DIMENSION)
                    .fold(LpExpression::ConsCont(new_delta), |acc, index| {
                        acc + LpExpression::ConsCont(self.variables[index].clone())
                            * ((route.costs[index] - result.costs[index]) as f32)
                    })
                    .le(0);
            }
        }
        all_explained
    }

    fn solve(&self) -> Option<Preference> {
        self.problem
            .write_lp("lp_formulation")
            .expect("Could not write LP to file");
        match self.solver.run(&self.problem) {
            Ok((status, var_values)) => {
                println!("Solver Status: {:?}", status);
                let mut alpha = [0.0; EDGE_COST_DIMENSION];
                let mut all_zero = true;
                for (name, value) in var_values.iter() {
                    if name.contains("delta") {
                        println!("{}: {}", name, value);
                    } else {
                        if *value != 0.0 {
                            all_zero = false;
                        }
                        // The order of variables in the HashMap is not fixed
                        for (index, tag) in EDGE_COST_TAGS.iter().enumerate() {
                            if name == tag {
                                alpha[index] = f64::from(*value);
                                break;
                            }
                        }
                    }
                }
                println!("Alpha: {:?}", alpha);
                if all_zero {
                    return None;
                }
                Some(alpha)
            }
            Err(msg) => {
                println!("LpError: {}", msg);
                None
            }
        }
    }
}
