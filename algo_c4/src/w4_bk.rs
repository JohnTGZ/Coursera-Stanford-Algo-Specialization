use std::fs;
use rand::Rng;

#[derive(Debug)]
pub struct Clauses {
    pub num_v: u32,
    pub clauses: Vec<(i32, i32)>,
}

impl Clauses {
    pub fn construct(input_filepath: &str) -> Clauses {
        println!("Constructing Assignment from {input_filepath}");
        let contents = fs::read_to_string(input_filepath).unwrap();

        let mut content_itr = contents.split("\n").map(|s| s.to_string());
        
        let num_v = content_itr.next().unwrap().parse::<u32>().unwrap();

        println!("Number of (vertices) = ({num_v})");

        let mut clauses: Vec<(i32, i32)> = Vec::new();

        while let Some(line) = content_itr.next() {
            if line.is_empty(){
                break;
            }
            let mut line_itr = line.split(" ").map(|s| s.to_string());
            clauses.push((
                line_itr.next().unwrap().parse::<i32>().unwrap(),
                line_itr.next().unwrap().parse::<i32>().unwrap(),
            ));
        }

        Clauses {
            num_v,
            clauses,
        }

    }

    pub fn check_satisfied(&self, assignments: &Vec<bool>) -> (bool, (usize,usize)) {
        for clause in &self.clauses {
            let idx_l = (clause.0.abs() - 1) as usize;
            let idx_r= (clause.1.abs() - 1) as usize;

            let mut val_l = assignments[idx_l];
            let mut val_r = assignments[idx_r];

            let mut cdn_l = "".to_string();
            let mut cdn_r = "".to_string();
    
            if clause.0 < 0 {
                val_l = !val_l;
                cdn_l += "!";
            }
    
            if clause.1 < 0 {
                val_r = !val_r;
                cdn_r += "!";
            }

            if !(val_l || val_r) {
                // println!("UNSATISFIED CLAUSE: ({cdn_l}{idx_l} OR {cdn_r}{idx_r}) = ({val_l},{val_r})");
                return (false, (idx_l, idx_r))
            }
        }

        println!("ALL SATISFIED!");
        return (true, (0,0));
    }

}

pub const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

pub fn log_2(x: u32) -> u32 {
    assert!(x > 0);
    num_bits::<u32>() as u32 - 1 - x.leading_zeros()
}

pub fn generate_random_assignments(num_v: u32) -> Vec<bool>{
    let mut assignments = Vec::new();
    for _ in 0..num_v{
        assignments.push(rand::random::<bool>());
    }
    return assignments;
}

pub fn flip_assignment(assignments: &mut Vec<bool>, unsat_idx :(usize, usize) ) -> (){
    // randomly flip one of the 2 variables
    match rand::random::<bool>() {
        true => assignments[unsat_idx.0] = !assignments[unsat_idx.0],
        false => assignments[unsat_idx.1] = !assignments[unsat_idx.1],
    }
}

pub fn print_assignments(assignments: &Vec<bool>, clauses: &Vec<(i32, i32)>) -> () {
    println!("Printing assignments");

    for clause in clauses {
        let idx_l = (clause.0.abs() - 1) as usize;
        let idx_r= (clause.1.abs() - 1) as usize;

        let mut val_l = assignments[idx_l];
        let mut val_r = assignments[idx_r];

        let mut cdn_l = "".to_string();
        let mut cdn_r = "".to_string();

        if clause.0 < 0 {
            val_l = !val_l;
            cdn_l += "!";
        }

        if clause.1 < 0 {
            val_r = !val_r;
            cdn_r += "!";
        }

        println!("Checking ({cdn_l}{idx_l} OR {cdn_r}{idx_r}) = ({val_l},{val_r})");
    }
}

pub fn papadimitriou_2_sat(clauses: &Clauses) -> bool {

    let n = clauses.num_v;

    let inner_loop: u64 = 2*(n as u64) * (n as u64);
    // let inner_loop: u64 = 2*(n as u64);

    // for _ in 0..log_2(n){
        // Choose random initial assignment
        let mut assignments = generate_random_assignments(n);
        // let mut assignments: Vec<bool> = vec![true; n as usize];

        for j in 0..inner_loop {
            if j%1000 == 0{
                println!("inner itr {j}");
            }
            // If current assignment satisfies all clauses, halt and report
            let (satisfied, (unsat_idx_l, unsat_idx_r)) = clauses.check_satisfied(&assignments);
            if satisfied {
                print_assignments(&assignments, &clauses.clauses);
                return true;
            }
            // else, pick arbitrary unsatisfied clause and flip the value of one of it's variables [choose one of 2 uniformly at random]
            else {
                //pick the next unsatisfied clause reported at idx_unsat
                flip_assignment(&mut assignments, (unsat_idx_l, unsat_idx_r));
            }
        }
    // }
    // print_assignments(&assignments, &clauses.clauses);

    return false
}