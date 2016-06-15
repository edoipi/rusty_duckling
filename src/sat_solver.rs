use std::iter::*;
use std::process::exit;
use cnf_manager::*;
use SatInstance;
use Restarter;
use utils::*;

pub struct SatSolver {
	pub cnf_manager : CnfManager,
	pub restarter : Restarter,
	pub restarter_unit : i32,
	pub next_decay : i32,
	pub next_restart : i32
}

impl SatSolver {
	pub fn new(sat_instance: &SatInstance) -> SatSolver {
		let mut ret = SatSolver {
			cnf_manager : CnfManager::new(sat_instance),
			restarter : Restarter::new(),
			restarter_unit : 512,
			next_decay : 128,
			next_restart : 0
		};

		ret.next_restart = ret.restarter.next_threshold() * ret.restarter_unit;

		if ret.cnf_manager.decision_level == 0 {
			return ret;
		}


		for i in 1..(ret.cnf_manager.var_count+1) as usize {
			if ret.cnf_manager.vars[i].value == VA::Free {
				if ret.cnf_manager.vars[i].activity[VA::Pos as usize] == 0 && ret.cnf_manager.vars[i].activity[VA::Neg as usize] > 0 {
					ret.cnf_manager.assert_literal(-(i as i32), ArrTuple::new(), 0);
				} else if ret.cnf_manager.vars[i].activity[VA::Neg as usize] == 0 && ret.cnf_manager.vars[i].activity[VA::Pos as usize] > 0 {
					ret.cnf_manager.assert_literal((i as i32), ArrTuple::new(), 0);
				}
			}
		}

		for i in 1..(ret.cnf_manager.var_count+1) as usize {
			if ret.cnf_manager.vars[i].value == VA::Free && ret.cnf_manager.weight(&(i as i32)) > 0 {
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

	pub fn select_literal(& mut self) -> i32 {
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
				if self.cnf_manager.good(&lit) {
					sat = true;
					break;
				}
				ind += 1;
			}
			if sat {
				continue;
			}

			let mut weight = -1;
			ind = self.cnf_manager.clauses[i as usize] as usize;
			while self.cnf_manager.lit_pool[ind] != 0 {
				let lit = self.cnf_manager.lit_pool[ind];
				if lit == 0 {
					break;
				}
				if self.cnf_manager.free(&lit) && self.cnf_manager.weight(&(to_var(&lit) as i32)) > weight {
					x = to_var(&lit) as i32;
					weight = self.cnf_manager.weight(&x);
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
		if self.cnf_manager.decision_level == 0 {
			return false;
		}
		let mut lit = self.select_literal();
		while lit != 0 {
			if !self.cnf_manager.decide(lit) {
				loop {
					if self.cnf_manager.assertion_level == 0 {
						return false;
					}

					if self.cnf_manager.conflict_count == self.next_decay {
						self.next_decay += 128;
						self.cnf_manager.score_decay();
					}

					self.cnf_manager.next_clause = self.cnf_manager.clauses.len() as i32 - 1;

					if self.cnf_manager.conflict_count == self.next_restart {
						self.cnf_manager.restart_count += 1;
						self.next_restart += self.restarter.next_threshold() * self.restarter_unit;
						self.cnf_manager.backtrack(1);
						if self.cnf_manager.decision_level != self.cnf_manager.assertion_level {
							break;
						}
					} else {
						let level = self.cnf_manager.assertion_level;
						self.cnf_manager.backtrack(level);
					}

					if self.cnf_manager.assert_conflict_literal() {
						break;
					}
				}
			}
			lit = self.select_literal();
		}
		if !self.verify_solution() {
			println!("ERROR");
			exit(0);
		}
		true
	}

	pub fn verify_solution(&self) -> bool {
		let ref pool = self.cnf_manager.lit_pool;
		let mut i = 0;
		while i < self.cnf_manager.lit_pool_size_orig {
			let mut satisfied = false;
			while pool[i] != 0 {
				let ref lit = pool[i];
				i += 1;
				if self.cnf_manager.good(&lit) {
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

	pub fn print_solution(&self) -> () {
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

	pub fn print_stats(&self) -> () {
		let ref m = self.cnf_manager;
		println!("c {} decisions, {} conflicts, {} restarts", m.decision_count, m.conflict_count, m.restart_count);
	}
}
