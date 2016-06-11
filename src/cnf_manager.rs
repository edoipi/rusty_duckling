use std::collections::VecDeque;

pub enum VA {
	Neg = 0,
	Pos = 1,
	Free = 2
} 

pub struct Variable {
	pub uip_mark : bool,
	pub phase : bool,
	pub value : VA,
	pub decision_level : i32,
	pub ante : Vec<i32>,
	pub activity : [i32; 2],
	pub bin_imp : [Vec<i32>; 2],
	pub watch : [Vec<Vec<i32>>; 2]
}

impl Variable {
    pub fn new() -> Variable {
    	Variable {
    		uip_mark : false,
    		phase : false,
    		value : VA::Free,
    		decision_level : 0,
    		ante : Vec::new(),
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
	pub clauses : Vec<i32>,
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