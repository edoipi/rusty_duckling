use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std;
use structs::Clause;
use structs::Instance;

pub fn preprocess() -> Instance {
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
    
    let mut instance = Instance {nbvar: 0, nbclauses: 0, assignment: Vec::new(), literals: HashSet::new(), clauses: HashMap::new()};
    let re = Regex::new("p cnf.*?([0-9]+).*?([0-9]+)").unwrap();
    match re.captures(&input) {
        Some(caps) => {
            instance.nbvar = caps[1].parse::<i32>().unwrap();
            instance.nbclauses = caps[2].parse::<i32>().unwrap();
        },
        None => panic!("Invalid input"),
    };
    
    for i in 0..instance.nbclauses {
        instance.clauses.insert(i, Clause{literals: HashSet::new()});
        loop {
            let tmp: i32 = read!();
            match tmp {
                0 => break,
                x => instance.clauses.get_mut(&i).unwrap().literals.insert(x),   
            };
        }
    }
    
    instance.assignment = vec![false; instance.nbvar as usize];
    instance.literals = (0..instance.nbvar as usize).collect();
    instance
}
