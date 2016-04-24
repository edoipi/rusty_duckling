#[macro_use] extern crate text_io;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::linked_list::LinkedList;
use std::io;
use std::collections::hash_set::Iter;

struct Clause {
    literals: HashSet<i32>,
}

struct Instance {
    nbvar: i32,
    nbclauses: i32,
    assignment: Vec<bool>,
    literals: HashSet<usize>,
    clauses: HashMap<i32, Clause>
}

fn id(x: bool) -> bool {
    x
}

fn not(x: bool) -> bool {
    !x
}

fn lit_num(var: usize, pos: bool) -> i32 {
    if pos {var as i32} else {-(var as i32)}
}

fn check(instance: &Instance) -> bool {
    for (_, ref clause) in &instance.clauses {
        let mut satisfied = false;
        for &literal in &clause.literals {
            let fun: fn(bool) -> bool = if literal > 0 {id} else {not};
            let val = instance.assignment[(literal.abs() - 1) as usize];
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

fn subsets(instance: &mut Instance, iter: &mut Iter<usize>) -> bool {
    match iter.next() {
        None => { 
            return check(instance)
        },
        Some(&depth) => {
            instance.assignment[depth] = false;
            if subsets(instance, iter) {
                return true;
            }
            instance.assignment[depth] = true;
            if subsets(instance, iter) {
                return true;
            }
            return false;
        }
    };
}

fn print_solution(satisfiable: bool, instance: &Instance) {
    if satisfiable {
        println!("s SATISFIABLE");
        print!("v ");
        for (i, &val) in instance.assignment.iter().enumerate() {
            print!("{} ", lit_num(i + 1, val));
        }
        println!("0");
    } else {
        println!("s UNSATISFIABLE");
    }
}

fn simple_solve(mut instance: &mut Instance) {
    let not_set = instance.literals.clone(); //no idea how to fix that

    let satisfiable = subsets(&mut instance, &mut not_set.iter());
    
    print_solution(satisfiable, instance);
}

fn unit_propagation(instance: &mut Instance) -> bool {
    let mut queue = LinkedList::new();
    let mut literal_map : HashMap<i32, Vec<i32>> = HashMap::new(); 
    for (&clause_id, ref clause) in &instance.clauses {
        if clause.literals.len() == 1 {
            queue.push_back(clause.literals.iter().next().unwrap().clone());
        }
        for &literal in &clause.literals {
            literal_map.entry(literal.abs()).or_insert(Vec::new()).push(clause_id);
        }
    }
    while !queue.is_empty() {
        let literal = queue.pop_front().unwrap();
        for &clause_id in &literal_map[&literal.abs()] {
            {
                let clause = unsafe {&mut *(
                    match instance.clauses.get_mut(&clause_id) {
                        Some(val) => val,
                        None => continue,
                    } as *mut Clause)}; //don't ask
                
                if clause.literals.remove(&literal) {
                    instance.clauses.remove(&clause_id);
                }
                
                if clause.literals.remove(&(-literal)) && clause.literals.len() == 0 {
                    return false;
                }
                
                if clause.literals.len() == 1 {
                    queue.push_back(clause.literals.iter().next().unwrap().clone());
                }
                
                instance.assignment[(literal.abs() - 1) as usize] = if literal > 0 {true} else {false};
            }
        }
    }
    
    instance.literals.clear();
    for (_, ref clause) in &mut instance.clauses {
        for &literal in &clause.literals {
            instance.literals.insert(literal.abs() as usize);
        }
    }
    
    return true;
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
    if unit_propagation(&mut instance) {
        simple_solve(&mut instance);
    } else {
        print_solution(false, &instance);
    }
}

