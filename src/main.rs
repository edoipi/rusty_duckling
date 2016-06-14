extern crate rusty_duckling;
use rusty_duckling::*;
use std::io;

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_line(&mut input);

	let instances_count = input.trim().parse::<i32>().unwrap();

	for _ in 0..instances_count {
		let ref instance = preprocess();
		let mut sat_solver = SatSolver::new(instance);
		let satisfiable = sat_solver.run();
		if satisfiable {
			println!("SAT");
			sat_solver.printSolution();
		} else {
			println!("UNSAT");
		}
		//sat_solver.printStats();
	}
}

