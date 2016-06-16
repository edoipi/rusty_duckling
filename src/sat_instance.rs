use std::io;
use std::collections::HashSet;

pub struct Reader {
    pub buffer : Vec<String>
}

impl Reader {
    pub fn read_line() -> String {
        let mut input = String::new();
        while input.len() == 0 {
            input = String::new();
            let _ = io::stdin().read_line(&mut input);
            input = String::from(input.trim());
        }
        input
    }
    
    pub fn new() -> Reader {
        Reader{ buffer: Vec::new() }
    }
    
    pub fn next(&mut self) -> String{
        if self.buffer.len() > 0 {
            return self.buffer.pop().unwrap();
        }
        let input = Reader::read_line();
        self.buffer = input.split_whitespace().map(|x| { String::from(x) }).collect();
        self.buffer.reverse();
        self.next()
    }
}

pub struct SatInstance {
	pub var_count : usize,
	pub clause_count : usize,
	pub clauses : Vec<Vec<i32>>
}

impl SatInstance {
    pub fn read_line() -> String {
        let mut input = String::new();
        while input.len() == 0 {
            input = String::new();
            let _ = io::stdin().read_line(&mut input);
            input = String::from(input.trim());
        }
        input
    }

	pub fn read(reader : &mut Reader) -> SatInstance {
		//let mut input = String::new();
		//let _ = io::stdin().read_line(&mut input);
        //let mut input;// = SatInstance::read_line();
		let mut instance = SatInstance {var_count: 0, clause_count: 0, clauses: Vec::new()};
		/*{
			let mut iter = input.split_whitespace();
			iter.next();
			iter.next();
			instance.var_count = iter.next().unwrap().parse::<usize>().unwrap();
			instance.clause_count = iter.next().unwrap().parse::<usize>().unwrap();
            
            match iter.next() {
                Some(_) => panic!("wolololo"),
                None => ()
            }
		}*/
        
        assert_eq!(reader.next(), "p");
        assert_eq!(reader.next(), "cnf");
        instance.var_count = reader.next().parse::<usize>().unwrap();
        instance.clause_count = reader.next().parse::<usize>().unwrap();
        
        let mut counter = 0;
        let mut vec = Vec::new();
        let mut set = HashSet::new();
        let mut omit = false;
        while counter < instance.clause_count {
            let lit = reader.next().parse::<i32>().unwrap();
            if lit != 0 {
                if set.contains(&(-lit)) {
                    omit = true;
                    //break;
                }
                if set.contains(&lit) {
                    continue;
                }
                vec.push(lit);
                set.insert(lit);
            } else {
                if !omit {
                    instance.clauses.push(vec);
                }
                vec = Vec::new();
                set = HashSet::new();
                counter += 1;
                omit = false;
            }
        }
		/*for _ in 0..instance.clause_count {
			input = SatInstance::read_line();
			
            let mut vec = Vec::<i32>::new();
            let mut set = HashSet::new();
            let mut omit = false;
            for it in input.split_whitespace() {
                if it != "0" {
                    let lit = it.parse::<i32>().unwrap(); 
                    vec.push(lit);
                    if set.contains(&(-lit)) {
                        omit = true;
                        break;
                    }
                    set.insert(lit);
                }
            }
            if omit {
                continue;
            }
            vec.sort();
            vec.dedup();
            instance.clauses.push(vec);
		}*/
        
        instance.clause_count = instance.clauses.len();
		instance
	}
}
