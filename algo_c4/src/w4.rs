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
            let idx_l = clause.0.abs() as usize;
            let idx_r= clause.1.abs() as usize;

            let mut l_val = assignments[idx_l];
            let mut r_val = assignments[idx_r];

            if clause.0 < 0 {
                l_val = !l_val;
            }

            if clause.1 < 0 {
                r_val = !r_val;
            }

            if !(l_val && r_val) {
                return (false, (idx_l, idx_r))
            }
        }

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

pub fn papadimitriou_2_sat(clauses: &Clauses) -> bool {

    let n = clauses.num_v;

    for _ in 0..log_2(n){
        // Choose random initial assignment
        let mut assignments = generate_random_assignments(n);

        for _ in 0..2*n*n{
            // If current assignment satisfies all clauses, halt and report
            let (satisfied, (unsat_idx_l, unsat_idx_r)) = clauses.check_satisfied(&assignments);
            if satisfied {
                return true;
            }
            // else, pick arbitrary unsatisfied clause and flip the value of one of it's variables [choose one of 2 uniformly at random]
            else {
                //pick the next unsatisfied clause reported at idx_unsat
                flip_assignment(&mut assignments, (unsat_idx_l, unsat_idx_r));
            }
        }
    }

    return false
}