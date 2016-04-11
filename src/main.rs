#[macro_use] extern crate text_io;
extern crate regex;
use std::io;
use regex::Regex;

struct Clause {
    literals : Vec<i32>,
}

fn main() {
    let comment = Regex::new("^c.*").unwrap();
    let mut input : String;
    loop {
        input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => 
                match comment.is_match(&input) {
                    true => continue,
                    false => break,
                },
            Err(error) => println!("error: {}", error),
        };
    }
    
    let (nbvar, nbclauses) : (i32, i32);
    let re = Regex::new("p cnf.*?([0-9]+).*?([0-9]+)").unwrap();
    //print!("{}", input);
    match re.captures(&input) {
        Some(caps) => {
            nbvar = caps[1].parse::<i32>().unwrap();
            nbclauses = caps[2].parse::<i32>().unwrap();
        },
        None => panic!("Invalid input"),
    };
    
    let mut clauses = Vec::<Clause>::new();
    for i in 0..nbclauses {
        clauses.push(Clause{literals : Vec::new()});
        loop {
            let tmp : i32 = read!();
            match tmp {
                0 => break,
                x => clauses[i as usize].literals.push(x),   
            };
        }
    }
    
    for i in 0..clauses.len() {
        let lits = &clauses[i].literals;
        for j in 0..lits.len() {
            print!("{} ", lits[j]);
        }
        println!("");
    }
}

