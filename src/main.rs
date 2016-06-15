extern crate rusty_duckling;
use rusty_duckling::*;
use std::io;

fn main() {
	let mut input = String::new();
	let _ = io::stdin().read_line(&mut input);

	let instance_count = input.trim().parse::<i32>().unwrap();

	for _ in 0..instance_count {
		let ref sat_instance = SatInstance::read();
		let mut sat_solver = SatSolver::new(sat_instance);
		let satisfiable = sat_solver.run();
		if satisfiable {
			println!("SAT");
			sat_solver.print_solution();
		} else {
			println!("UNSAT");
		}
	}
}

