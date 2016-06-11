extern crate rusty_duckling;
use rusty_duckling::*;
use std::io;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    
    let instances_count = input.trim().parse::<i32>().unwrap();
    
    for _ in 0..instances_count {
        let mut instance = preprocess();
        /*println!("\n{} {} {}\n", instance.var_count, instance.clause_count, instance.clause_sum_length);
        for vec in instance.clauses {
            for val in vec {
                print!("{} ", val);
            }
            println!("");
        }*/
    }
}

