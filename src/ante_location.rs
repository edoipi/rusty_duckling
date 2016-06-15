#[derive(Clone)]
pub struct AnteLocation {
	pub is_null: bool,
	pub is_lit_pool: bool,
	pub var_index: usize,
	pub positive: usize
}

impl AnteLocation {
	pub fn new() -> AnteLocation {
		AnteLocation {
			is_null: true,
			is_lit_pool: false,
			var_index: 0,
			positive: 0,
		}
	}

	pub fn ctor(islp : bool) -> AnteLocation {
		let mut nt = AnteLocation::new();
		nt.is_null = false;
		nt.is_lit_pool = islp;
		nt
	}

	pub fn ctor2(vari : usize, pos : usize) -> AnteLocation {
		let mut nt = AnteLocation::ctor(false);
		nt.var_index = vari;
		nt.positive = pos;
		nt
	}
}
