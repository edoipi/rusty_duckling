use CnfManager;

pub struct Luby {
	pub seq : Vec<i32>,
	pub index : i32,
	pub k : i32
}

pub struct SatSolver {
	pub cnf_manager : CnfManager,
	pub luby : Luby,
	pub luby_unit : i32,
	pub next_decay : i32,
	pub next_restart : i32
}
