use std::io;
use std::collections::HashSet;

pub struct SatInstance {
	pub var_count : usize,
	pub clause_count : usize,
	pub clauses : Vec<Vec<i32>>
}

impl SatInstance {
    pub fn read_line() -> String {
        let mut input = String::new();
        while input.len() == 0 {
            input = String::new();
            let _ = io::stdin().read_line(&mut input);
            input = String::from(input.trim());
        }
        input
    }

	pub fn read() -> SatInstance {
		//let mut input = String::new();
		//let _ = io::stdin().read_line(&mut input);
        let mut input = SatInstance::read_line();
		let mut instance = SatInstance {var_count: 0, clause_count: 0, clauses: Vec::new()};
		{
			let mut iter = input.split_whitespace();
			iter.next();
			iter.next();
			instance.var_count = iter.next().unwrap().parse::<usize>().unwrap();
			instance.clause_count = iter.next().unwrap().parse::<usize>().unwrap();
		}

		for _ in 0..instance.clause_count {
			input = SatInstance::read_line();
			
            let mut vec = Vec::<i32>::new();
            let mut set = HashSet::new();
            let mut omit = false;
            for it in input.split_whitespace() {
                if it != "0" {
                    let lit = it.parse::<i32>().unwrap(); 
                    vec.push(lit);
                    if set.contains(&(-lit)) {
                        omit = true;
                        break;
                    }
                    set.insert(lit);
                }
            }
            if omit {
                continue;
            }
            vec.sort();
            vec.dedup();
            instance.clauses.push(vec);
		}
        instance.clause_count = instance.clauses.len();
		instance
	}
}
