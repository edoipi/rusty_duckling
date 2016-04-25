use std::collections::HashMap;
use std::collections::HashSet;

pub struct Clause {
    pub literals: HashSet<i32>,
}

pub struct Instance {
    pub nbvar: i32,
    pub nbclauses: i32,
    pub assignment: Vec<bool>,
    pub literals: HashSet<usize>,
    pub clauses: HashMap<i32, Clause>
}
