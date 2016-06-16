use std::iter::*;
use logic::*;
use consts;
use SatInstance;
use Restarter;
use AnteLocation;
use utils::*;

pub struct SatSolver {
	pub logic : Logic,
	pub restarter : Restarter,
	pub restarter_unit : usize,
	pub next_decay : usize,
	pub next_restart : usize
}

impl SatSolver {
	pub fn new(sat_instance: &SatInstance) -> SatSolver {
		let mut ret = SatSolver {
			logic : Logic::new(sat_instance),
			restarter : Restarter::new(),
			restarter_unit : consts::RESTART_MULTIPLIER,
			next_decay : consts::DECAY_INTERVAL,
			next_restart : 0
		};

		if ret.logic.failed == true {
			return ret;
		}

		ret.next_restart = ret.restarter.next_threshold() * ret.restarter_unit;

		if ret.logic.decision_level == 0 {
			return ret;
		}


		for i in 1..(ret.logic.var_count+1) {
			if ret.logic.vars[i].value == VA::Free {
				if ret.logic.vars[i].occurs[VA::Pos as usize] == 0 && ret.logic.vars[i].occurs[VA::Neg as usize] > 0 {
					ret.logic.assert_literal(-(i as i32), AnteLocation::new(), 0);
				} else if ret.logic.vars[i].occurs[VA::Neg as usize] == 0 && ret.logic.vars[i].occurs[VA::Pos as usize] > 0 {
					ret.logic.assert_literal((i as i32), AnteLocation::new(), 0);
				}
			}
		}

		for i in 1..(ret.logic.var_count+1) {
			if ret.logic.vars[i].value == VA::Free && ret.logic.weight(&i) > 0 {
				ret.logic.var_order.push(i);
				ret.logic.vars[i].phase =
					if ret.logic.vars[i].occurs[VA::Pos as usize] > ret.logic.vars[i].occurs[VA::Neg as usize] {
						true
					} else {
						false
					};
			}
		}

		ret.logic.sort_vars();

		for i in 0..ret.logic.var_order.len() {
			ret.logic.var_position[ret.logic.var_order[i]] = i;
		}

		ret.logic.next_var = 0;
		ret.logic.next_clause = ret.logic.clauses.len() as i32 - 1;

		ret
	}

	pub fn select_literal(& mut self) -> i32 {
		let mut x = 0;
		let last_clause = if self.logic.next_clause > consts::CLAUSE_LIMIT {
			self.logic.next_clause - consts::CLAUSE_LIMIT
		} else {
			0
		};

		for i in (last_clause..(self.logic.next_clause + 1)).rev() {
			self.logic.next_clause = i;

			let mut sat = false;
			let mut ind = self.logic.clauses[i as usize];
			while self.logic.lit_pool[ind] != 0 {
				let lit = self.logic.lit_pool[ind];
				if lit == 0 {
					break;
				}
				if self.logic.good(&lit) {
					sat = true;
					break;
				}
				ind += 1;
			}
			if sat {
				continue;
			}

			let mut weight = -1;
			ind = self.logic.clauses[i as usize];
			while self.logic.lit_pool[ind] != 0 {
				let lit = self.logic.lit_pool[ind];
				if lit == 0 {
					break;
				}
				if self.logic.free(&lit) && self.logic.weight(&to_var(&lit)) > weight {
					x = to_var(&lit);
					weight = self.logic.weight(&x);
				}
				ind += 1;
			}
			return self.decide_sign(x);
		}

		for i in self.logic.next_var..self.logic.var_order.len() {
			if self.logic.vars[self.logic.var_order[i]].value == VA::Free {
				x = self.logic.var_order[i];
				self.logic.next_var = i + 1;
				return self.decide_sign(x);
			}
		}
		0
	}

	pub fn decide_sign(&self, var : usize) -> i32 {
		let diff = self.logic.vars[var].occurs[VA::Pos as usize]
				- self.logic.vars[var].occurs[VA::Neg as usize];
		if diff > consts::PHASE_THRESHOLD {
			var as i32
		} else if -diff > consts::PHASE_THRESHOLD {
			-(var as i32)
		} else if self.logic.vars[var].phase {
			var as i32
		} else {
			-(var as i32)
		}
	}

	pub fn run(&mut self) -> bool {
		if self.logic.decision_level == 0 {
			return false;
		}
		let mut lit = self.select_literal();
		while lit != 0 {
			if !self.logic.decide(lit) {
				loop {
					if self.logic.assertion_level == 0 {
						return false;
					}

					if self.logic.conflict_count == self.next_decay {
						self.next_decay += consts::DECAY_INTERVAL;
						self.logic.score_decay();
					}

					self.logic.next_clause = self.logic.clauses.len() as i32 - 1;

					if self.logic.conflict_count == self.next_restart {
						self.logic.restart_count += 1;
						self.next_restart += self.restarter.next_threshold() * self.restarter_unit;
						self.logic.revert_to_decision_level(1);
						if self.logic.decision_level != self.logic.assertion_level {
							break;
						}
					} else {
						let level = self.logic.assertion_level;
						self.logic.revert_to_decision_level(level);
					}

					if self.logic.assert_conflict_literal() {
						break;
					}
				}
			}
			lit = self.select_literal();
		}
		true
	}

	pub fn print_solution(&self) -> () {
		for i in 1..(self.logic.var_count+1) {
			let ref vars = self.logic.vars;
			if vars[i].value == VA::Pos {
				print!("{} ", i);
			} else {
				print!("-{} ", i);
			}
		}
		println!("0");
	}
}
