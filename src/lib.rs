#[macro_use] extern crate text_io;
extern crate regex;

pub use self::preprocess::preprocess;

pub use self::cnf::Cnf;

pub mod preprocess;
pub mod cnf;
