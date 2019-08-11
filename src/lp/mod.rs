use lp_modeler::operations::LpOperations;
use lp_modeler::problem::{LpFileFormat, LpObjective, LpProblem};
use lp_modeler::solvers::{GlpkSolver, SolverTrait};
use lp_modeler::variables::{lp_sum, LpContinuous, LpExpression};

use crate::{EDGE_COST_DIMENSION, EDGE_COST_TAGS};
use crate::graph::dijkstra::{DijkstraResult, find_path};
use crate::graph::edge::calc_total_cost;
use crate::graph::Graph;
use crate::helpers::Preference;

pub struct PreferenceEstimator {
    problem: LpProblem,
    variables: Vec<LpContinuous>,
    delta: LpContinuous,
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
        let delta = LpContinuous::new("delta");

        // Objective Function
        problem += &delta;

        // Constraints
        for var in &variables {
            problem += var.ge(0);
        }
        problem += lp_sum(&variables).equal(1);
        problem += delta.ge(0);

        PreferenceEstimator {
            problem,
            variables,
            delta,
            solver: GlpkSolver::new(),
        }
    }
    pub fn get_preference(
        &mut self,
        graph: &Graph,
        driven_routes: &[DijkstraResult],
        alpha_in: Preference
    ) -> Option<Preference> {
        self.check_feasibility(graph, driven_routes, alpha_in);
        while let Some(alpha) = self.solve() {
            let feasible = self.check_feasibility(graph, driven_routes, alpha);
            if feasible {
                return Some(alpha);
            }
        }
        None
    }

    fn check_feasibility(&mut self, graph: &Graph, driven_routes: &[DijkstraResult], alpha: Preference) -> bool {
        let mut all_explained = true;
        for route in driven_routes {
            let source = route.path[0];
            let target = route.path[route.path.len() - 1];
            let result = find_path(graph, vec![source, target], alpha).unwrap();
            if calc_total_cost(route.costs, alpha).0 > result.total_cost {
                all_explained = false;
                println!("Not explained, {} > {}", calc_total_cost(route.costs, alpha).0, result.total_cost);
                self.problem += (0..EDGE_COST_DIMENSION)
                    .fold(LpExpression::ConsCont(self.delta.clone()), |acc, index| {
                        acc + LpExpression::ConsCont(self.variables[index].clone()) * ((route.costs[index] - result.costs[index]) as f32)
                    }).le(0);
            }
        }
        all_explained
    }

    fn solve(&self) -> Option<Preference> {
        self.problem.write_lp("lp_formulation").expect("Could not write LP to file");
        match self.solver.run(&self.problem) {
            Ok((status, var_values)) => {
                println!("Solver Status: {:?}", status);
                let mut alpha = [0.0; EDGE_COST_DIMENSION];
                for (name, value) in var_values.iter() {
                    if name == "delta" {
                        println!("delta: {}", value);
                    }
                    for (index, tag) in EDGE_COST_TAGS.iter().enumerate() {
                        if name == tag {
                            alpha[index] = f64::from(*value);
                            break;
                        }
                    }
                }
                println!("Alpha: {:?}", alpha);
                Some(alpha)
            }
            Err(msg) => {
                println!("LpError: {}", msg);
                None
            }
        }
    }
}