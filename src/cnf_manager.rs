use std::collections::VecDeque;
use Cnf;
use std::ptr;

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
	pub watch : [Vec<Vec<i32>>; 2]
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
	pub lit_pool_size_orig : i32,
	pub clauses : Vec<Vec<i32>>,
	pub next_clause : i32,

	pub decision_stack : Vec<i32>,
	pub assertion_level : i32,
	pub decision_level : i32,
	pub decision_count : i32,
	pub conflict_count : i32,
	pub restart_count : i32,
	pub conflict_lit : VecDeque<i32>,
	pub tmp_conflict_lit : VecDeque<i32>,
	pub conflict_clause : Option<&'w Vec<i32>>,
    pub conflict_clause_ind : i32
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
			decision_level : 0,
			decision_count : 0,
			conflict_count : 0,
			restart_count : 0,
			conflict_lit : VecDeque::new(),
			tmp_conflict_lit : VecDeque::new(),
			conflict_clause : None,
            conflict_clause_ind : 0
		};
		//TODO initialize fields
		ret
	}

	pub fn setLiteral(&mut self, lit : i32, ante : &'w Vec<i32>, ind : usize) -> () {
		self.vars[VAR(&lit)].value = SIGN(&lit);
		self.vars[VAR(&lit)].ante = Some(ante);
		self.vars[VAR(&lit)].ante_ind = ind;
		self.vars[VAR(&lit)].decision_level = self.decision_level;
	}

	pub fn assertLiteral(&self, lit : i32, ante : Option<&Vec<i32>>, ante_ind : i32) -> bool {
		//TODO implement
		false
	}

	pub fn assertUnitClauses(&self) -> bool {
		//TODO implement
		false
	}

	pub fn decide(&mut self, lit : i32) -> bool {
        self.decision_count += 1;
        self.decision_level += 1;
        return self.assertLiteral(lit, None, 0);
    }

	pub fn learnClause(&self, first_lit : &Vec<i32>) -> () {
		//TODO implement
	}

	pub fn addClause(&self) -> () {
		//TODO implement
	}

	pub fn assertCL(&self) -> bool {
		return self.assertLiteral(self.conflict_clause.unwrap()[0], self.conflict_clause, self.conflict_clause_ind+1);
	}

	pub fn backtrack(&mut self, level : i32) -> () {
		let mut var = VAR(&self.decision_stack.last().unwrap());
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
			var = VAR(&self.decision_stack.last().unwrap());
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
