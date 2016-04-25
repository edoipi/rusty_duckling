use std::collections::HashMap;
use std::collections::linked_list::LinkedList;
use structs::Clause;
use structs::Instance;

pub fn unit_propagation(instance: &mut Instance) -> bool {
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
            instance.literals.insert((literal.abs() - 1) as usize);
        }
    }
    
    return true;
}
