extern crate rusty_duckling;
use rusty_duckling::*;

fn main() {
    let mut instance = preprocess();
    if unit_propagation(&mut instance) {
        simple_solve(&mut instance);
    } else {
        print_solution(false, &instance);
    }
}

