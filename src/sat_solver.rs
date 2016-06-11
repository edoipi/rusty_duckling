use Cnf;
use cnf_manager::*;
use std::iter::*;

pub struct Luby {
	pub seq : Vec<i32>,
	pub index : i32,
	pub k : i32
}

impl Luby {
	pub fn new() -> Luby {
		Luby {seq : Vec::new(), index : 0, k : 1}
	}

	pub fn next(& mut self) -> i32 {
		self.index += 1;
		if self.index == (1 << self.k) - 1 {
			self.seq.push(1 << (self.k - 1));
			self.k += 1;
		} else {
			let val = self.seq[(self.index - (1 << (self.k - 1))) as usize];
			self.seq.push(val);
		}
		self.seq.last().unwrap().clone()
	}
}

pub struct SatSolver {
	pub cnf_manager : CnfManager,
	pub luby : Luby,
	pub luby_unit : i32,
	pub next_decay : i32,
	pub next_restart : i32
}

impl SatSolver {
	pub fn new(cnf: &Cnf) -> SatSolver {
		let mut ret = SatSolver {
			cnf_manager : CnfManager::new(cnf),
			luby : Luby::new(),
			luby_unit : 512,
			next_decay : 128,
			next_restart : 0
		};

		ret.next_restart = ret.luby.next()*ret.luby_unit;

		if ret.cnf_manager.decision_level == 0 {
			return ret;
		}


		for i in 1..(ret.cnf_manager.var_count+1) as usize {
			if ret.cnf_manager.vars[i].value == VA::Free {
				if ret.cnf_manager.vars[i].activity[VA::Pos as usize] == 0 && ret.cnf_manager.vars[i].activity[VA::Neg as usize] > 0 {
					//ret.cnf_manager.assertLiteral(-(i as i32), Vec::new());
				} else if ret.cnf_manager.vars[i].activity[VA::Neg as usize] == 0 && ret.cnf_manager.vars[i].activity[VA::Pos as usize] > 0 {
					//ret.cnf_manager.assertLiteral((i as i32), Vec::new());
				}
			}
		}

		for i in 1..(ret.cnf_manager.var_count+1) as usize {
			if ret.cnf_manager.vars[i].value == VA::Free && SCORE(&(i as i32), &ret.cnf_manager) > 0 {
				ret.cnf_manager.var_order.push(i as i32);
				ret.cnf_manager.vars[i].phase = 
					if ret.cnf_manager.vars[i].activity[VA::Pos as usize] > ret.cnf_manager.vars[i].activity[VA::Neg as usize] {
						true
					} else {
						false
					};
			}
		}

		//ret.cnf_manager.var_order.sort_by(|a, b| SCORE(a, &ret.cnf_manager).cmp(&SCORE(b, &ret.cnf_manager)));
		ret.cnf_manager.sort_vars();

		for i in 0..ret.cnf_manager.var_order.len() {
			ret.cnf_manager.var_position[ ret.cnf_manager.var_order[i as usize] as usize ] = i as i32;
		}

		ret.cnf_manager.next_var = 0;
		ret.cnf_manager.next_clause = (ret.cnf_manager.clauses.len() - 1) as i32;

		ret
	}

	pub fn selectLiteral(& mut self) -> i32 {
		let mut x = 0 as i32;
		let last_clause = if self.cnf_manager.next_clause > 256 {
			self.cnf_manager.next_clause - 256
		} else {
			0
		};

		for i in (last_clause..(self.cnf_manager.next_clause + 1)).rev() {
			self.cnf_manager.next_clause = i;

			let mut sat = false;
			for &lit in self.cnf_manager.clauses[i as usize].iter() {
				if lit == 0 {
					break;
				}
				if SET(&lit, &self.cnf_manager) {
					sat = true;
					break;
				}
			}
			if sat {
				continue;
			}

			let mut score = -1;
			for &lit in self.cnf_manager.clauses[i as usize].iter() {
				if lit == 0 {
					break;
				}
				if FREE(&lit, &self.cnf_manager) && SCORE(&(VAR(&lit) as i32), &self.cnf_manager) > score {
					x = VAR(&lit) as i32;
					score = SCORE(&x, &self.cnf_manager);
				}
			}

			let d = self.cnf_manager.vars[x as usize].activity[VA::Pos as usize]
					- self.cnf_manager.vars[x as usize].activity[VA::Neg as usize];
			if d > 32 {
				return x;
			} else if -d > 32 {
				return -x;
			} else if self.cnf_manager.vars[x as usize].phase {
				return x;
			} else {
				return -x;
			}
		}

		for i in (self.cnf_manager.next_var as usize)..self.cnf_manager.var_order.len() {
			if self.cnf_manager.vars[self.cnf_manager.var_order[i] as usize].value == VA::Free {
				x = self.cnf_manager.var_order[i];
				self.cnf_manager.next_var = (i + 1) as i32;

				let d = self.cnf_manager.vars[x as usize].activity[VA::Pos as usize]
					- self.cnf_manager.vars[x as usize].activity[VA::Neg as usize];
				if d > 32 {
					return x;
				} else if -d > 32 {
					return -x;
				} else if self.cnf_manager.vars[x as usize].phase {
					return x;
				} else {
					return -x;
				}
			}
		}
		0
	}

	pub fn verifySolution(&self) -> bool {
		let ref pool = self.cnf_manager.lit_pool;
		for mut i in 0..self.cnf_manager.lit_pool_size_orig as usize {
			let mut satisfied = false;
			while pool[i] != 0 {
				let ref lit = pool[i];
				i += 1;
				if SET(&lit, &self.cnf_manager) {
					satisfied = true;
					while pool[i] != 0 {
						i += 1;
					}
					break;
				}
			}
			if !satisfied {
				return false;
			}
		}
		true
	}

	pub fn printSolution(&self) -> () {
		for i in 1..(self.cnf_manager.var_count+1) as usize {
			let ref vars = self.cnf_manager.vars;
			if vars[i].value == VA::Pos {
				print!("{} ", i);
			} else {
				print!("-{} ", i);
			}
		}
		println!("0");
	}

	pub fn printStats(&self) -> () {
		let ref m = self.cnf_manager;
		println!("c {} decisions, {} conflicts, {} restarts", m.decision_count, m.conflict_count, m.restart_count);
	}
}
