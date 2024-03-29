use Cnf;
use cnf_manager::*;
use std::iter::*;
use std::process::exit;

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

impl<'w> SatSolver {
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
					ret.cnf_manager.assertLiteral(-(i as i32), ArrTuple::new(), 0);
				} else if ret.cnf_manager.vars[i].activity[VA::Neg as usize] == 0 && ret.cnf_manager.vars[i].activity[VA::Pos as usize] > 0 {
					ret.cnf_manager.assertLiteral((i as i32), ArrTuple::new(), 0);
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

		ret.cnf_manager.sort_vars();

		for i in 0..ret.cnf_manager.var_order.len() {
			ret.cnf_manager.var_position[ ret.cnf_manager.var_order[i as usize] as usize ] = i as i32;
		}

		ret.cnf_manager.next_var = 0;
		ret.cnf_manager.next_clause = ret.cnf_manager.clauses.len() as i32 - 1;

		ret
	}

	pub fn selectLiteral(& mut self) -> i32 {
		//println!("selectLiteral");
		let mut x = 0 as i32;
		let last_clause = if self.cnf_manager.next_clause > 256 {
			self.cnf_manager.next_clause - 256
		} else {
			0
		};

		for i in (last_clause..(self.cnf_manager.next_clause + 1)).rev() {
			self.cnf_manager.next_clause = i;

			let mut sat = false;
			let mut ind = self.cnf_manager.clauses[i as usize] as usize;
			while self.cnf_manager.lit_pool[ind] != 0 {
				let lit = self.cnf_manager.lit_pool[ind];
				if lit == 0 {
					break;
				}
				if SET(&lit, &self.cnf_manager) {
					sat = true;
					break;
				}
				ind += 1;
			}
			if sat {
				continue;
			}

			let mut score = -1;
			ind = self.cnf_manager.clauses[i as usize] as usize;
			while self.cnf_manager.lit_pool[ind] != 0 {
				let lit = self.cnf_manager.lit_pool[ind];
				if lit == 0 {
					break;
				}
				if FREE(&lit, &self.cnf_manager) && SCORE(&(VAR(&lit) as i32), &self.cnf_manager) > score {
					x = VAR(&lit) as i32;
					score = SCORE(&x, &self.cnf_manager);
				}
				ind += 1;
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

	pub fn run(&mut self) -> bool {
		//println!("run {}", self.cnf_manager.decision_level);
		if self.cnf_manager.decision_level == 0 {
			return false;
		}
		let mut lit = self.selectLiteral();
		while lit != 0 {
			//println!("selected literal: {}", lit);
			if !self.cnf_manager.decide(lit) {
				loop {
					//println!("aLevel {}", self.cnf_manager.assertion_level);
					if self.cnf_manager.assertion_level == 0 {
						return false;
					}

					if self.cnf_manager.conflict_count == self.next_decay {
						self.next_decay += 128;
						self.cnf_manager.scoreDecay();
					}

					self.cnf_manager.next_clause = self.cnf_manager.clauses.len() as i32 - 1;

					if self.cnf_manager.conflict_count == self.next_restart {
						self.cnf_manager.restart_count += 1;
						self.next_restart += self.luby.next() * self.luby_unit;
						self.cnf_manager.backtrack(1);
						if self.cnf_manager.decision_level != self.cnf_manager.assertion_level {
							break;
						}
					} else {
						let level = self.cnf_manager.assertion_level;
						self.cnf_manager.backtrack(level);
					}

					if self.cnf_manager.assertCL() {
						break;
					}
				}
			}
			lit = self.selectLiteral();
		}
		if !self.verifySolution() {
			println!("ERROR");
			exit(0);
		}
		true
	}

	pub fn verifySolution(&self) -> bool {
		let ref pool = self.cnf_manager.lit_pool;
		let mut i = 0;
		while i < self.cnf_manager.lit_pool_size_orig {
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
			i += 1;
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
