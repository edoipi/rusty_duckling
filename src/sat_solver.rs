use cnf_manager::*;

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
