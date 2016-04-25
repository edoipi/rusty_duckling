#[macro_use] extern crate text_io;
extern crate regex;

pub use self::preprocess::preprocess;

pub use self::simple_solve::print_solution;
pub use self::simple_solve::simple_solve;

pub use self::structs::Clause;
pub use self::structs::Instance;

pub use self::unit_propagation::unit_propagation;

pub mod preprocess;
pub mod simple_solve;
pub mod structs;
pub mod unit_propagation;
