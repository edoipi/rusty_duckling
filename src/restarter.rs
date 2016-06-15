//based on Luby algorithm
pub struct Restarter {
	pub threshold_sequence : Vec<usize>,
	pub i : usize,
	pub exp : usize
}

impl Restarter {
	pub fn new() -> Restarter {
		Restarter {threshold_sequence : Vec::new(), i : 0, exp : 1}
	}

	pub fn next_threshold(& mut self) -> usize {
		self.i += 1;
		if self.i == (1 << self.exp) - 1 {
			self.threshold_sequence.push(1 << (self.exp - 1));
			self.exp += 1;
		} else {
			let val = self.threshold_sequence[(self.i - (1 << (self.exp - 1)))];
			self.threshold_sequence.push(val);
		}
		self.threshold_sequence.last().unwrap().clone()
	}
}
