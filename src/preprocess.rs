use regex::Regex;
use std::io;
use cnf::Cnf;

pub fn preprocess() -> Cnf {
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
    
    let mut instance = Cnf {var_count: 0, clause_count: 0, clause_sum_length: 0, clauses: Vec::new()};
    {
        let re = Regex::new("p cnf.*?([0-9]+).*?([0-9]+)").unwrap();
        match re.captures(&input) {
            Some(caps) => {
                instance.var_count = caps[1].parse::<i32>().unwrap();
                instance.clause_count = caps[2].parse::<i32>().unwrap();
            },
            None => panic!("Invalid input"),
        };
    }
    
    for _ in 0..instance.clause_count {
        input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut vec = Vec::<i32>::new();
                for it in input.split_whitespace() {
                    if it != "0" {
                        vec.push(it.parse::<i32>().unwrap());
                    }
                }
                instance.clause_sum_length += vec.len() as i32;
                instance.clauses.push(vec);
            },
            Err(error) => println!("error: {}", error),
        };
    }

    instance
}
