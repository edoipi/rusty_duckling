use std::collections::VecDeque;
use Cnf;
use std::ptr;
use std::process::exit;

#[derive(PartialEq, Clone)]
pub enum VA {
	Neg = 0,
	Pos = 1,
	Free = 2
}

pub fn SIGN(&lit : &i32) -> VA {
	if lit > 0 {VA::Pos} else {VA::Neg}
}

pub fn VAR(&lit : &i32) -> usize {
	lit.abs() as usize
}

pub fn NEG(&lit : &i32) -> i32 {
	-lit
}

pub fn FREE(&lit : &i32, m : &CnfManager) -> bool {
	m.vars[VAR(&lit)].value == VA::Free
}

pub fn SET(&lit : &i32, m : &CnfManager) -> bool {
	m.vars[VAR(&lit)].value == SIGN(&lit)
}

pub fn RESOLVED(&lit : &i32, m : &CnfManager) -> bool {
	let neg = NEG(&lit);
	m.vars[VAR(&lit)].value == SIGN(&neg)
}

pub fn SCORE(&var : &i32, m : &CnfManager) -> i32 {
	m.vars[var as usize].activity[0] + m.vars[var as usize].activity[1]
}


pub struct Variable<'w> {
	pub uip_mark : bool,
	pub phase : bool,
	pub value : VA,
	pub decision_level : i32,
	pub ante : Option<&'w Vec<i32>>,
	pub ante_ind : usize,
	pub activity : [i32; 2],
	pub bin_imp : [Vec<i32>; 2],
	pub watch : [Vec<usize>; 2]
}

impl<'w> Variable<'w> {
	pub fn new() -> Variable<'w> {
		Variable {
			uip_mark : false,
			phase : false,
			value : VA::Free,
			decision_level : 0,
			ante : None,
			ante_ind : 0,
			activity : [0, 0],
			bin_imp : [Vec::new(), Vec::new()],
			watch : [Vec::new(), Vec::new()]
		}
	}
}

pub struct CnfManager<'w> {
	pub var_count : i32,
	pub vars : Vec<Variable<'w>>,
	pub var_order : Vec<i32>,
	pub var_position : Vec<i32>,
	pub next_var : i32,

	pub lit_pool : Vec<i32>,
	pub lit_pool_size_orig : usize,
	pub clauses : Vec<usize>,
	pub next_clause : usize,

	pub decision_stack : Vec<i32>,
	pub assertion_level : i32,
	pub decision_level : i32,
	pub decision_count : i32,
	pub conflict_count : i32,
	pub restart_count : i32,
	pub conflict_lit : VecDeque<i32>,
	pub tmp_conflict_lit : VecDeque<i32>,
    pub conflict_clause_ind : usize
}

impl<'w> CnfManager<'w> {
	pub fn new(cnf : &Cnf) -> CnfManager<'w> {
		let mut ret = CnfManager {
			var_count : cnf.var_count,
			vars : Vec::new(),
			var_order : Vec::new(),
			var_position : Vec::new(),
			next_var : 0,
			lit_pool : Vec::new(),
			lit_pool_size_orig : 0,
			clauses : Vec::new(),
			next_clause : 0,
			decision_stack : Vec::new(),
			assertion_level : 0,
			decision_level : 1,
			decision_count : 0,
			conflict_count : 0,
			restart_count : 0,
			conflict_lit : VecDeque::new(),
			tmp_conflict_lit : VecDeque::new(),
            conflict_clause_ind : 0
		};
		let mut imp : [Vec<Vec<i32>>; 2] = [Vec::new(), Vec::new()];
		for i in 0..ret.var_count+1 {
			ret.vars.push(Variable::new());
			ret.var_position.push(0);
			imp[0].push(Vec::new());
			imp[1].push(Vec::new());
		}

		ret.decision_stack.push(0);
		ret.vars[0].decision_level = 0;
		ret.vars[0].value = VA::Free;

		for i in 0..cnf.clause_count as usize {
			if cnf.clauses[i].len() == 1 {
				let lit = cnf.clauses[i][0];
				if FREE(&lit, &ret) {
					ret.decision_stack.push(lit);
					ret.setLiteral(lit, None, 0);
				} else if RESOLVED(&lit, &ret) {
					println!("UNSAT");
					exit(0);
				}
			} else if cnf.clauses[i].len() == 2 {
				let lit0 = cnf.clauses[i][0];
				let lit1 = cnf.clauses[i][1];
				imp[SIGN(&lit0) as usize][VAR(&lit0)].push(lit1);
				imp[SIGN(&lit1) as usize][VAR(&lit1)].push(lit0);
				ret.vars[VAR(&lit0)].activity[SIGN(&lit0) as usize] += 1;
				ret.vars[VAR(&lit1)].activity[SIGN(&lit1) as usize] += 1;
			} else {
				let lit0 = cnf.clauses[i][0];
				let lit1 = cnf.clauses[i][1];
				ret.vars[VAR(&lit0)].watch[SIGN(&lit0) as usize].push(ret.lit_pool.len());
				ret.vars[VAR(&lit1)].watch[SIGN(&lit1) as usize].push(ret.lit_pool.len());
				for j in cnf.clauses[i].iter() {
					ret.vars[VAR(j)].activity[SIGN(j) as usize] += 1;
					ret.lit_pool.push(j.clone());
				}
			}
		}
		ret.lit_pool_size_orig = ret.lit_pool.len();

		for i in 1..ret.var_count+1 {
			for j in 0..2 {
				ret.vars[i as usize].bin_imp[j].push(0);
				ret.vars[i as usize].bin_imp[j].push(if j == VA::Pos as usize {i} else {-i});
				ret.vars[i as usize].bin_imp[j].push(0);
				for k in imp[j][i as usize].iter() {
					ret.vars[i as usize].bin_imp[j].push(k.clone());
				}
				ret.vars[i as usize].bin_imp[j].push(0);
			}
		}

		ret.assertUnitClauses();
		ret
	}

	pub fn setLiteral(&mut self, lit : i32, ante : Option<&'w Vec<i32>>, ind : usize) -> () {
		self.vars[VAR(&lit)].value = SIGN(&lit);
		self.vars[VAR(&lit)].ante = ante;
		self.vars[VAR(&lit)].ante_ind = ind;
		self.vars[VAR(&lit)].decision_level = self.decision_level;
	}

	pub fn assertLiteral(&self, lit : i32, ante : Option<&Vec<i32>>, ante_ind : usize) -> bool {
		//TODO implement
		false
	}

	pub fn assertUnitClauses(&mut self) -> bool {
		let mut lit : i32 = *(self.decision_stack.last().unwrap());
		while lit != 0 {
			self.decision_stack.pop();
			if !self.assertLiteral(lit, Some(&self.lit_pool), self.lit_pool.len() - 1) {
				let level = self.decision_level - 1;
				self.backtrack(level);
				return false;
			}
			lit = *(self.decision_stack.last().unwrap());
		}
		true
	}

	pub fn decide(&mut self, lit : i32) -> bool {
        self.decision_count += 1;
        self.decision_level += 1;
        return self.assertLiteral(lit, None, 0);
    }

	pub fn learnClause(&'w mut self, conflict_clause : &Vec<i32>, mut ind : usize) -> () {
		//TODO implement
		if self.decision_level == 1 {
			self.assertion_level = 0;
			return;
		}

		self.updateScores(conflict_clause, ind);

		self.conflict_lit.clear();
		self.tmp_conflict_lit.clear();
		let mut cur_level_lits = 0;
		while conflict_clause[ind] != 0 {
			let lit = conflict_clause[ind];
			ind += 1;
			if self.vars[VAR(&lit)].decision_level == 1 {
				continue;
			}
			if self.vars[VAR(&lit)].decision_level < self.decision_level {
				self.tmp_conflict_lit.push_back(lit);
			} else {
				cur_level_lits += 1;
			}
			self.vars[VAR(&lit)].uip_mark = true;
		}

		let mut lit = 0;
		loop {
			lit = *self.decision_stack.last().unwrap();
			self.decision_stack.pop();
			let var = VAR(&lit);
			self.vars[var].value = VA::Free;
			if !self.vars[var].uip_mark {
				if self.var_position[var] < self.next_var {
					self.next_var = self.var_position[var];
				}
				continue;
			}

			self.vars[var].uip_mark = false;
			match self.vars[var].ante {
				Some(ante) => {
					let self2 = unsafe {&mut *(self as *mut CnfManager)};
					self.updateScores(ante, self2.vars[var].ante_ind - 1)
				},
				None => (),
			}

			if self.var_position[var] < self.next_var {
				self.next_var = self.var_position[var];
			}

			if cur_level_lits == 1 {
				cur_level_lits -= 1;
				break;
			}

			match self.vars[var].ante {
				Some(ante) => {
					let mut z = self.vars[var].ante_ind;
					while ante[z] != 0 {
						let v = ante[z];
						z += 1;
						if self.vars[VAR(&v)].uip_mark || self.vars[VAR(&v)].decision_level == 1 {
							continue;
						}
						if self.vars[VAR(&v)].decision_level < self.decision_level {
							self.tmp_conflict_lit.push_back(v);
						} else {
							cur_level_lits += 1;
						}
						self.vars[VAR(&v)].uip_mark = true;
					}
				},
				None => (),
			}
		}

		self.assertion_level = 1;
		for conf_lit in self.tmp_conflict_lit.iter() {
			let mut redundant = true;
			match self.vars[VAR(conf_lit)].ante {
				Some(ante) => {
					let mut z = self.vars[VAR(conf_lit)].ante_ind;
					while ante[z] != 0 {
						if !self.vars[VAR(&ante[z])].uip_mark {
							redundant = false;
							break;
						}
						z += 1;
					}
				},
				None => {
					redundant = false;
				},
			}
			if !redundant {
				if self.vars[VAR(conf_lit)].decision_level > self.assertion_level {
					self.assertion_level = self.vars[VAR(conf_lit)].decision_level;
					self.conflict_lit.push_front(conf_lit.clone());
				} else {
					self.conflict_lit.push_back(conf_lit.clone());
				}
			}
		}

		for tlit in self.tmp_conflict_lit.iter() {
			self.vars[VAR(tlit)].uip_mark = false;
		}

		self.conflict_lit.push_back(-lit);
		self.addClause();
	}

	pub fn addClause(&'w mut self) -> () {
		self.conflict_clause_ind = self.lit_pool.len();
		self.lit_pool.push(*self.conflict_lit.back().unwrap());
		if self.conflict_lit.len() > 1 {
			self.clauses.push(self.conflict_clause_ind);
			self.lit_pool.push(*self.conflict_lit.front().unwrap());
			let back_lit = self.conflict_lit.back().unwrap();
			self.vars[VAR(back_lit)].watch[SIGN(back_lit) as usize].push(self.conflict_clause_ind);
			let front_lit = self.conflict_lit.front().unwrap();
			self.vars[VAR(front_lit)].watch[SIGN(front_lit) as usize].push(self.conflict_clause_ind);

			for i in 1..self.conflict_lit.len()-1 {
				self.lit_pool.push(self.conflict_lit[i]);
			}
		}
		self.lit_pool.push(0);
	}

	pub fn assertCL(&self) -> bool {
		return self.assertLiteral(self.lit_pool[self.conflict_clause_ind], Some(&self.lit_pool), self.conflict_clause_ind+1);
	}

	pub fn backtrack(&mut self, level : i32) -> () {
		let mut var = VAR(self.decision_stack.last().unwrap());
		while self.vars[var].decision_level > level {
			if self.vars[var].decision_level < self.decision_level {
				let val = self.vars[var].value.clone() as i32;
				self.vars[var].phase = val > 0;
			}
			self.vars[var].value = VA::Free;
			if self.var_position[var] < self.next_var {
				self.next_var = self.var_position[var];
			}
			self.decision_stack.pop();
			var = VAR(self.decision_stack.last().unwrap());
		}
		self.decision_level = level;
	}

	pub fn scoreDecay(&mut self) -> () {
		for i in 1..(self.var_count + 1) as usize {
			self.vars[i].activity[0] >>= 1;
			self.vars[i].activity[1] >>= 1;
		}
	}

	pub fn updateScores(&mut self, vec : &Vec<i32>, mut ind : usize) -> () {
		while vec[ind] != 0 {
			let lit = &vec[ind];
			let v = VAR(lit);
			self.vars[v].activity[SIGN(lit) as usize] += 1;
			let pos = self.var_position[v];
			ind += 1;

			if pos == 0 {
				continue;
			}

			let score = SCORE(&(v as i32), &self);
			if score <= SCORE(&self.var_order[(pos - 1) as usize], &self) {
				continue;
			}

			let mut step = 0x400;
			let mut q = pos - step;
			while q >= 0 {
				if SCORE(&self.var_order[q as usize], &self) >= score {
					break;
				}
				q -= step;
			}
			q += step;
			step >>= 1;
			while step > 0 {
				if q - step >= 0 && SCORE(&self.var_order[(q - step) as usize], &self) < score {
					q -= step;
				}
				step >>= 1;
			}

			self.var_order[pos as usize] = self.var_order[q as usize];
			self.var_position[v] = q;
			self.var_position[self.var_order[q as usize] as usize] = pos;
			self.var_order[q as usize] = v as i32;
		}
	}

	pub fn sort_vars(&mut self) {
		let uns = unsafe {&mut *(self as *mut CnfManager)};
		self.var_order.sort_by(|a, b| SCORE(a, &uns).cmp(&SCORE(b, &uns)));
	}
}
