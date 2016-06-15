use AnteLocation;
use VA;

pub struct VariableInfo {
	pub uip_mark : bool,
	pub phase : bool,
	pub value : VA,
	pub decision_level : i32,
	pub ante : AnteLocation,
	pub ante_ind : usize,
	pub activity : [i32; 2],
	pub bin_imp : [Vec<i32>; 2],
	pub watch : [Vec<usize>; 2]
}

impl VariableInfo {
	pub fn new() -> VariableInfo {
		VariableInfo {
			uip_mark : false,
			phase : false,
			value : VA::Free,
			decision_level : 0,
			ante : AnteLocation::new(),
			ante_ind : 0,
			activity : [0, 0],
			bin_imp : [Vec::new(), Vec::new()],
			watch : [Vec::new(), Vec::new()]
		}
	}
}
