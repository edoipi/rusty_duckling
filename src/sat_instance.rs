use std::io;

pub struct SatInstance {
	pub var_count : usize,
	pub clause_count : usize,
	pub clauses : Vec<Vec<i32>>
}

impl SatInstance {
	pub fn read() -> SatInstance {
		let mut input = String::new();
		let _ = io::stdin().read_line(&mut input);
		let mut instance = SatInstance {var_count: 0, clause_count: 0, clauses: Vec::new()};
		{
			let mut iter = input.split_whitespace();
			iter.next();
			iter.next();
			instance.var_count = iter.next().unwrap().parse::<usize>().unwrap();
			instance.clause_count = iter.next().unwrap().parse::<usize>().unwrap();
		}

		for _ in 0..instance.clause_count {
			input = String::new();
			match io::stdin().read_line(&mut input) {
				Ok(_) => {
					let mut vec = Vec::<i32>::new();
					for it in input.split_whitespace() {
						if it != "0" {
							vec.push(it.parse::<i32>().unwrap());
						}
					}
					instance.clauses.push(vec);
				},
				Err(error) => println!("error: {}", error),
			};
		}

		instance
	}
}
