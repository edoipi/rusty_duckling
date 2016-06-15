use std::collections::VecDeque;
use std::process::exit;
use SatInstance;
use utils::*;

#[derive(Clone)]
pub struct ArrTuple {
	pub is_null: bool,
	pub is_lit_pool: bool,
	pub var_index: usize,
	pub positive: usize
}

impl ArrTuple {
	pub fn new() -> ArrTuple {
		ArrTuple {
			is_null: true,
			is_lit_pool: false,
			var_index: 1000,
			positive: 1000,
		}
	}

	pub fn ctor(islp : bool) -> ArrTuple {
		let mut nt = ArrTuple::new();
		nt.is_null = false;
		nt.is_lit_pool = islp;
		nt
	}

	pub fn ctor2(vari : usize, pos : usize) -> ArrTuple {
		let mut nt = ArrTuple::ctor(false);
		nt.var_index = vari;
		nt.positive = pos;
		nt
	}
}

pub struct Variable {
	pub uip_mark : bool,
	pub phase : bool,
	pub value : VA,
	pub decision_level : i32,
	pub ante : ArrTuple,
	pub ante_ind : usize,
	pub activity : [i32; 2],
	pub bin_imp : [Vec<i32>; 2],
	pub watch : [Vec<usize>; 2]
}

impl Variable {
	pub fn new() -> Variable {
		Variable {
			uip_mark : false,
			phase : false,
			value : VA::Free,
			decision_level : 0,
			ante : ArrTuple::new(),
			ante_ind : 0,
			activity : [0, 0],
			bin_imp : [Vec::new(), Vec::new()],
			watch : [Vec::new(), Vec::new()]
		}
	}
}

pub struct CnfManager {
	pub var_count : i32,
	pub vars : Vec<Variable>,
	pub var_order : Vec<i32>,
	pub var_position : Vec<i32>,
	pub next_var : i32,

	pub lit_pool : Vec<i32>,
	pub lit_pool_size_orig : usize,
	pub clauses : Vec<usize>,
	pub next_clause : i32,

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

impl CnfManager {
	pub fn new(sat_instance : &SatInstance) -> CnfManager {
		let mut ret = CnfManager {
			var_count : sat_instance.var_count,
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
		for _ in 0..ret.var_count+1 {
			ret.vars.push(Variable::new());
			ret.var_position.push(0);
			imp[0].push(Vec::new());
			imp[1].push(Vec::new());
		}

		ret.decision_stack.push(0);
		ret.vars[0].decision_level = 0;
		ret.vars[0].value = VA::Free;

		for i in 0..sat_instance.clause_count as usize {
			if sat_instance.clauses[i].len() == 1 {
				let lit = sat_instance.clauses[i][0];
				if ret.free(&lit) {
					ret.decision_stack.push(lit);
					ret.set_literal(lit, ArrTuple::new(), 0);
				} else if ret.bad(&lit) {
					println!("UNSAT");
					exit(0);
				}
			} else if sat_instance.clauses[i].len() == 2 {
				let lit0 = sat_instance.clauses[i][0];
				let lit1 = sat_instance.clauses[i][1];
				imp[sign(&lit0) as usize][to_var(&lit0)].push(lit1);
				imp[sign(&lit1) as usize][to_var(&lit1)].push(lit0);
				ret.vars[to_var(&lit0)].activity[sign(&lit0) as usize] += 1;
				ret.vars[to_var(&lit1)].activity[sign(&lit1) as usize] += 1;
			} else {
				let lit0 = sat_instance.clauses[i][0];
				let lit1 = sat_instance.clauses[i][1];
				ret.vars[to_var(&lit0)].watch[sign(&lit0) as usize].push(ret.lit_pool.len());
				ret.vars[to_var(&lit1)].watch[sign(&lit1) as usize].push(ret.lit_pool.len());
				for j in sat_instance.clauses[i].iter() {
					ret.vars[to_var(j)].activity[sign(j) as usize] += 1;
					ret.lit_pool.push(j.clone());
				}
				ret.lit_pool.push(0);
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

		ret.assert_unit_clauses();
		ret
	}

	pub fn set_literal(&mut self, lit : i32, ante : ArrTuple, ind : usize) -> () {
		self.vars[to_var(&lit)].value = sign(&lit);
		self.vars[to_var(&lit)].ante = ante;
		self.vars[to_var(&lit)].ante_ind = ind;
		self.vars[to_var(&lit)].decision_level = self.decision_level;
	}

	pub fn assert_literal(&mut self, mut lit : i32, ante : ArrTuple, ante_ind : usize) -> bool {
		let self2 = unsafe {&mut *(self as *mut CnfManager)};
		let self3 = unsafe {&mut *(self as *mut CnfManager)};

		let mut new_stack : Vec<i32> = Vec::new();
		let mut new_stack_it = 0;

		new_stack.push(lit);
		self.set_literal(lit, ante, ante_ind);

		while new_stack_it < new_stack.len() {
			lit = neg(&new_stack[new_stack_it]);
			new_stack_it += 1;
			self.decision_stack.push(-lit);

			let mut imp_ind = 3;
			{
			let imp = &mut self2.vars[to_var(&lit)].bin_imp[sign(&lit) as usize];
			while imp_ind < imp.len() {
				let imp_lit = imp[imp_ind];
				if self.free(&imp_lit) {
					if imp_lit == 0 {
						break;
					}
					new_stack.push(imp_lit);
					self3.set_literal(imp_lit, ArrTuple::ctor2(to_var(&lit), sign(&lit) as usize), 1);
				} else if self.bad(&imp_lit) {
					self3.conflict_count += 1;
					while new_stack_it < new_stack.len() {
						self3.decision_stack.push(new_stack[new_stack_it]);
						new_stack_it += 1;
					}
					imp[0] = imp_lit;
					self3.learn_clause(ArrTuple::ctor2(to_var(&lit), sign(&lit) as usize), 0);
					return false;
				}
				imp_ind += 1;
			}}

			let watchlist = &mut self2.vars[to_var(&lit)].watch[sign(&lit) as usize];
			let mut it : i32 = 0;
			while it < watchlist.len() as i32 {
				let first = watchlist[it as usize];
				let watch;
				let other_watch;
				if self.lit_pool[first] == lit {
					watch = first;
					other_watch = first + 1;
				} else {
					watch = first + 1;
					other_watch = first;
				}

				if self.good(&self.lit_pool[other_watch]) {
					it += 1;
					continue;
				}

				let mut p = first + 2;
				let mut found = true;
				while self.bad(&self.lit_pool[p]) {
					p += 1;
				}
				if self.lit_pool[p] == 0 {
					found = false;
				}

				if found {
					let plit = self.lit_pool[p];
					self.vars[to_var(&plit)].watch[sign(&plit) as usize].push(first);

					watchlist[it as usize] = watchlist.last().unwrap().clone();
					watchlist.pop();
					it -= 1;

					let x = self.lit_pool[watch];
					self.lit_pool[watch] = self.lit_pool[p];
					self.lit_pool[p] = x;
				} else {
					let olit = self.lit_pool[other_watch];
					if self.free(&olit) {
						new_stack.push(olit);
						self.set_literal(olit, ArrTuple::ctor(true), first + 1);

						if other_watch != first {
							let x = self.lit_pool[other_watch];
							self.lit_pool[other_watch] = self.lit_pool[first];
							self.lit_pool[first] = x;
						}
					} else if self.bad(&olit) {
						self.conflict_count += 1;
						while new_stack_it < new_stack.len() {
							self.decision_stack.push(new_stack[new_stack_it]);
							new_stack_it += 1;
						}
						self.learn_clause(ArrTuple::ctor(true), first);
						return false;
					}
				}
				it += 1;
			}
		}
		true
	}

	pub fn assert_unit_clauses(&mut self) -> bool {
		let self2 = unsafe {&mut *(self as *mut CnfManager)};
		for i in (1..self.decision_stack.len()).rev() {
			let lit = self.decision_stack[i];
			if i != self.decision_stack.len() - 1 {
				self.decision_stack[i] = self.decision_stack.pop().unwrap();
			}

			if !self2.assert_literal(lit, ArrTuple::ctor(true), &self.lit_pool.len().clone() - 1) {
				self2.backtrack(self.decision_level - 1);
				return false;
			}
		}
		true
	}

	pub fn decide(&mut self, lit : i32) -> bool {
		self.decision_count += 1;
		self.decision_level += 1;
		return self.assert_literal(lit, ArrTuple::new(), 0);
	}

	pub fn learn_clause(&mut self, tuple : ArrTuple, mut ind : usize) -> () {
		let self2 = unsafe {&mut *(self as *mut CnfManager)};
		let self3 = unsafe {&mut *(self as *mut CnfManager)};
		let self4 = unsafe {&mut *(self as *mut CnfManager)};

		let conflict_clause =
			if tuple.is_lit_pool {
				&self.lit_pool
			} else {
				&self.vars[tuple.var_index].bin_imp[tuple.positive]
			};

		if self.decision_level == 1 {
			self.assertion_level = 0;
			return;
		}

		self2.update_weights(tuple.clone(), ind);

		self.conflict_lit.clear();
		self.tmp_conflict_lit.clear();
		let mut cur_level_lits = 0;
		while conflict_clause[ind] != 0 {
			let lit = conflict_clause[ind];
			ind += 1;
			if self.vars[to_var(&lit)].decision_level == 1 {
				continue;
			}
			if self.vars[to_var(&lit)].decision_level < self.decision_level {
				self.tmp_conflict_lit.push_back(lit);
			} else {
				cur_level_lits += 1;
			}
			self2.vars[to_var(&lit)].uip_mark = true;
		}

		let mut lit;
		loop {
			lit = self.decision_stack.last().unwrap().clone();
			self.decision_stack.pop();
			let var = to_var(&lit);
			self2.vars[var].value = VA::Free;
			if !self.vars[var].uip_mark {
				if self.var_position[var] < self.next_var {
					self.next_var = self.var_position[var];
				}
				continue;
			}

			self2.vars[var].uip_mark = false;
			if self.vars[var].ante.is_null == false {
				self2.update_weights(self3.vars[var].ante.clone(), self4.vars[var].ante_ind - 1)
			}


			if self.var_position[var] < self.next_var {
				self.next_var = self.var_position[var];
			}

			if cur_level_lits == 1 {
				break;
			}
			cur_level_lits -= 1;


			let x = &self.vars[var].ante;
			if x.is_null == false {
				let mut z = self.vars[var].ante_ind;

				let ante =
					if x.is_lit_pool {
						&self.lit_pool
					} else {
						&self.vars[x.var_index].bin_imp[x.positive]
					};

				while ante[z] != 0 {
					let v = ante[z];
					z += 1;
					if self.vars[to_var(&v)].uip_mark || self.vars[to_var(&v)].decision_level == 1 {
						continue;
					}
					if self.vars[to_var(&v)].decision_level < self.decision_level {
						self.tmp_conflict_lit.push_back(v);
					} else {
						cur_level_lits += 1;
					}
					self2.vars[to_var(&v)].uip_mark = true;
				}
			}
		}

		self.assertion_level = 1;
		for conf_lit in self.tmp_conflict_lit.iter() {
			let mut redundant = true;
			let x = &self.vars[to_var(conf_lit)].ante;
			if x.is_null == false {
				let ante =
					if x.is_lit_pool {
						&self.lit_pool
					} else {
						&self.vars[x.var_index].bin_imp[x.positive]
					};
				let mut z = self.vars[to_var(conf_lit)].ante_ind;
				while ante[z] != 0 {
					if !self.vars[to_var(&ante[z])].uip_mark {
						redundant = false;
						break;
					}
					z += 1;
				}
			} else {
				redundant = false;
			}

			if !redundant {
				if self.vars[to_var(conf_lit)].decision_level > self.assertion_level {
					self.assertion_level = self.vars[to_var(conf_lit)].decision_level;
					self.conflict_lit.push_front(conf_lit.clone());
				} else {
					self.conflict_lit.push_back(conf_lit.clone());
				}
			}
		}

		for tlit in self.tmp_conflict_lit.iter() {
			self2.vars[to_var(tlit)].uip_mark = false;
		}

		self2.conflict_lit.push_back(-lit);
		self2.add_clause();
	}

	pub fn add_clause(& mut self) -> () {
		self.conflict_clause_ind = self.lit_pool.len();
		self.lit_pool.push(self.conflict_lit.back().unwrap().clone());
		if self.conflict_lit.len() > 1 {
			self.clauses.push(self.conflict_clause_ind);
			self.lit_pool.push(self.conflict_lit.front().unwrap().clone());
			let back_lit = self.conflict_lit.back().unwrap();
			self.vars[to_var(back_lit)].watch[sign(back_lit) as usize].push(self.conflict_clause_ind);
			let front_lit = self.conflict_lit.front().unwrap();
			self.vars[to_var(front_lit)].watch[sign(front_lit) as usize].push(self.conflict_clause_ind);

			for i in 1..self.conflict_lit.len()-1 {
				self.lit_pool.push(self.conflict_lit[i]);
			}
		}
		self.lit_pool.push(0);
	}

	pub fn assert_conflict_literal(&mut self) -> bool {
		let ind = self.conflict_clause_ind.clone();
		let lit = self.lit_pool[ind].clone();
		return self.assert_literal(lit, ArrTuple::ctor(true), ind + 1);
	}

	pub fn backtrack(&mut self, level : i32) -> () {
		let mut var = to_var(self.decision_stack.last().unwrap());
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
			var = to_var(self.decision_stack.last().unwrap());
		}
		self.decision_level = level;
	}

	pub fn score_decay(&mut self) -> () {
		for i in 1..(self.var_count + 1) as usize {
			self.vars[i].activity[0] >>= 1;
			self.vars[i].activity[1] >>= 1;
		}
	}

	pub fn update_weights(&mut self, tuple : ArrTuple, mut ind : usize) -> () {
		let self2 = unsafe {&mut *(self as *mut CnfManager)};
		let vec =
			if tuple.is_lit_pool {
				&self.lit_pool
			} else {
				&self.vars[tuple.var_index].bin_imp[tuple.positive]
			};

		while vec[ind] != 0 {
			let lit = &vec[ind];
			let v = to_var(lit);
			self2.vars[v].activity[sign(lit) as usize] += 1;
			let pos = self.var_position[v];
			ind += 1;

			if pos == 0 {
				continue;
			}

			let weight = self.weight(&(v as i32));
			if weight <= self.weight(&self.var_order[(pos - 1) as usize]) {
				continue;
			}

			let mut step = 0x400;
			let mut q = pos - step;
			while q >= 0 {
				if self.weight(&self.var_order[q as usize]) >= weight {
					break;
				}
				q -= step;
			}
			q += step;
			step >>= 1;
			while step > 0 {
				if q - step >= 0 && self.weight(&self.var_order[(q - step) as usize]) < weight {
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
		self.var_order.sort_by(|a, b| uns.weight(a).cmp(&uns.weight(b)));
	}

	pub fn free(&self, &lit : &i32) -> bool {
		self.vars[to_var(&lit)].value == VA::Free
	}

	pub fn good(&self, &lit : &i32) -> bool {
		self.vars[to_var(&lit)].value == sign(&lit)
	}

	pub fn bad(&self, &lit : &i32) -> bool {
		let neg = neg(&lit);
		self.vars[to_var(&lit)].value == sign(&neg)
	}

	pub fn weight(&self, &var : &i32) -> i32 {
		self.vars[var as usize].activity[0] + self.vars[var as usize].activity[1]
	}
}
