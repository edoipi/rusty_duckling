#[macro_use] extern crate text_io;
extern crate regex;
use regex::Regex;
use std::io;

struct Clause {
    literals: Vec<i32>,
}

struct Instance {
    nbvar: i32,
    nbclauses: i32,
    clauses: Vec<Clause>
}

fn id(x: bool) -> bool {
    x
}

fn not(x: bool) -> bool {
    !x
}

fn check(instance: &Instance, vec: &Vec<bool>) -> bool {
    for it in 0..instance.clauses.len() {
        let clause = &instance.clauses[it];
        let mut satisfied = false;
        for it2 in 0..clause.literals.len() {
            let literal = clause.literals[it2];
            let fun: fn(bool) -> bool = if literal > 0 {id} else {not};
            let val = vec[(literal.abs() - 1) as usize];
            satisfied = fun(val);
            if satisfied {
                break;
            }
        }
        if !satisfied {
            return false;
        }
    }
    return true;
}

fn subsets(instance: &Instance, vec: &mut Vec<bool>, depth: usize) -> bool {
    if depth == vec.len() {
        return check(instance, vec);
    }
    vec[depth] = false;
    if subsets(instance, vec, depth + 1) {
        return true;
    }
    vec[depth] = true;
    if subsets(instance, vec, depth + 1) {
        return true;
    }
    return false;
}

fn simple_solve(instance: Instance) {
    let mut vec = vec![false; instance.nbvar as usize];
    let val = subsets(&instance, &mut vec, 0);
    println!("{}", val);
    for i in vec {
        print!("{} ", i);
    }
    println!("");
}

fn main() {
    let comment = Regex::new("^c.*").unwrap();
    let mut input: String;
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
    
    let mut instance = Instance {nbvar: 0, nbclauses: 0, clauses: Vec::new()};
    let re = Regex::new("p cnf.*?([0-9]+).*?([0-9]+)").unwrap();
    match re.captures(&input) {
        Some(caps) => {
            instance.nbvar = caps[1].parse::<i32>().unwrap();
            instance.nbclauses = caps[2].parse::<i32>().unwrap();
        },
        None => panic!("Invalid input"),
    };
    
    for i in 0..instance.nbclauses {
        instance.clauses.push(Clause{literals: Vec::new()});
        loop {
            let tmp: i32 = read!();
            match tmp {
                0 => break,
                x => instance.clauses[i as usize].literals.push(x),   
            };
        }
    }
    
    simple_solve(instance);
    
    /*for i in 0..instance.clauses.len() {
        let lits = &instance.clauses[i].literals;
        for j in 0..lits.len() {
            print!("{} ", lits[j]);
        }
        println!("");
    }*/
}

