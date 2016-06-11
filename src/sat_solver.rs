use CnfManager;

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
