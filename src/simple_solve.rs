use std::collections::hash_set::Iter;
use structs::Instance;

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
            println!("depth {}", depth);
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

pub fn print_solution(satisfiable: bool, instance: &Instance) {
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

pub fn simple_solve(mut instance: &mut Instance) {
    let not_set = instance.literals.clone(); //no idea how to fix that

    let satisfiable = subsets(&mut instance, &mut not_set.iter());
    print_solution(satisfiable, instance);
}
