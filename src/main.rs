extern crate rusty_duckling;
use rusty_duckling::*;

fn main() {
    let mut reader = Reader::new();
    
	let input = reader.next();
    
	let instance_count = input.trim().parse::<i32>().unwrap();

	for _ in 0..instance_count {
		let ref sat_instance = SatInstance::read(&mut reader);
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

