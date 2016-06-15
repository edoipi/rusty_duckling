//based on Luby algorithm
pub struct Restarter {
	pub threshold_sequence : Vec<i32>,
	pub i : i32,
	pub exp : i32
}

impl Restarter {
	pub fn new() -> Restarter {
		Restarter {threshold_sequence : Vec::new(), i : 0, exp : 1}
	}

	pub fn next_threshold(& mut self) -> i32 {
		self.i += 1;
		if self.i == (1 << self.exp) - 1 {
			self.threshold_sequence.push(1 << (self.exp - 1));
			self.exp += 1;
		} else {
			let val = self.threshold_sequence[(self.i - (1 << (self.exp - 1))) as usize];
			self.threshold_sequence.push(val);
		}
		self.threshold_sequence.last().unwrap().clone()
	}
}
