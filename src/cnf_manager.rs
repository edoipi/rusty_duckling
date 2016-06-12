use std::collections::VecDeque;
use Cnf;
use std::ptr;

#[derive(PartialEq)]
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
	pub conflict_clause : Vec<i32>
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
			conflict_clause : Vec::new()
		};
		//TODO initialize fields
		ret
	}

	pub fn setLiteral(&self, lit : i32, ante : &Vec<i32>) -> () {
		//TODO implement
	}

	pub fn assertLiteral(&self, lit : i32, ante : &Vec<i32>) -> bool {
		//TODO implement
		false
	}

	pub fn assertUnitClauses(&self) -> bool {
		//TODO implement
		false
	}

	pub fn decide(&self, lit : i32) -> bool {
		//TODO implement
		false
	}

	pub fn learnClause(&self, first_lit : &Vec<i32>) -> () {
		//TODO implement
	}

	pub fn addClause(&self) -> () {
		//TODO implement
	}

	pub fn assertCL(&self) -> bool {
		//TODO implement
		false
	}

	pub fn backtrack(&self, level : i32) -> () {
		//TODO implement
	}

	pub fn scoreDecay(&mut self) -> () {
		for i in 1..(self.var_count + 1) as usize {
			self.vars[i].activity[0] >>= 1;
			self.vars[i].activity[1] >>= 1;
		}
	}

	pub fn updateScores(&self, first : &Vec<i32>) -> () {
		//TODO implement
	}

	pub fn sort_vars(&mut self) {
		let uns = unsafe {&mut *(self as *mut CnfManager)};
		self.var_order.sort_by(|a, b| SCORE(a, &uns).cmp(&SCORE(b, &uns)));
	}
}
