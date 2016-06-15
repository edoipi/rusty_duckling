#[derive(PartialEq, Clone)]
pub enum VA {
	Neg = 0,
	Pos = 1,
	Free = 2
}

pub fn sign(&lit : &i32) -> VA {
	if lit > 0 {VA::Pos} else {VA::Neg}
}

pub fn to_var(&lit : &i32) -> usize {
	lit.abs() as usize
}

pub fn neg(&lit : &i32) -> i32 {
	-lit
}