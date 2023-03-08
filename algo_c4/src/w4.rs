use std::collections::{HashMap, HashSet};
use std::fs;
/// Undirected graph
pub struct Graph {
    pub edge_list: Vec<Vec<usize>>,
    pub edge_list_rev: Vec<Vec<usize>>,
    // pub clauses: Vec<(i32, i32)>,
    pub num_v: usize,
}

impl Graph {
    pub fn construct_scc(input_filepath: &str) -> Graph {
        println!("Constructing Graph object (For SCC) from {input_filepath}");

        let contents = fs::read_to_string(input_filepath).unwrap();
        let mut content_itr = contents.split("\n").map(|s| s.to_string());

        let num_v = content_itr.next().unwrap().parse::<u32>().unwrap() as usize;

        println!("Number of (vertices) = ({num_v})");

        let mut edge_list: Vec<Vec<usize>> = vec![Vec::new(); num_v];
        let mut edge_list_rev: Vec<Vec<usize>> = vec![Vec::new(); num_v];

        while let Some(line) = content_itr.next() {
            if line.is_empty() {
                break;
            }

            let mut line_itr = line.split(" ").map(|s| s.to_string());
            let a = line_itr.next().unwrap().parse::<usize>().unwrap() - 1;
            let b = line_itr.next().unwrap().parse::<usize>().unwrap() - 1;

            edge_list[a].push(b);
            edge_list_rev[b].push(a);
        }

        Graph {
            edge_list,
            edge_list_rev,
            num_v,
        }
    }

    pub fn construct(input_filepath: &str) -> Graph {
        println!("Constructing Graph object from {input_filepath}");

        let contents = fs::read_to_string(input_filepath).unwrap();
        let mut content_itr = contents.split("\n").map(|s| s.to_string());

        let num_v = content_itr.next().unwrap().parse::<u32>().unwrap() as usize;

        println!("Number of (vertices) = ({num_v})");

        let mut edge_list: Vec<Vec<usize>> = vec![Vec::new(); num_v * 2];
        let mut edge_list_rev: Vec<Vec<usize>> = vec![Vec::new(); num_v * 2];

        while let Some(line) = content_itr.next() {
            if line.is_empty() {
                break;
            }

            let mut line_itr = line.split(" ").map(|s| s.to_string());
            let a = line_itr.next().unwrap().parse::<i32>().unwrap();
            let b = line_itr.next().unwrap().parse::<i32>().unwrap();

            // Append to graph adjacent matrix
            let (a_prime, b_prime) = (-a, -b);

            let (a_idx, b_idx) = (signed_node_to_idx(a, num_v), signed_node_to_idx(b, num_v));
            let (a_prime_idx, b_prime_idx) =
                (signed_node_to_idx(a_prime, num_v), signed_node_to_idx(b_prime, num_v));

            edge_list[a_prime_idx].push(b_idx);
            edge_list[b_prime_idx].push(a_idx);

            edge_list_rev[b_idx].push(a_prime_idx);
            edge_list_rev[a_idx].push(b_prime_idx);
        }

        Graph {
            edge_list,
            edge_list_rev,
            num_v,
        }
    }
}

// Converts a signed node (-A or A) into a unsigned index
pub fn signed_node_to_idx(signed_node: i32, num_v: usize) -> usize {
    if signed_node < 0 {
        return (num_v- 1) + signed_node.abs() as usize ;
    }
    return signed_node as usize - 1;
}

// Converts an unsigned index (-A or A) into it's signed equivalent
pub fn unsigned_idx_to_same_node(unsigned_idx: usize, num_v: usize) -> usize {
    if unsigned_idx > (num_v -1) {
        return unsigned_idx - num_v ;
    }
    return unsigned_idx;
}

pub fn dfs_first_pass(
    edge_list: &Vec<Vec<usize>>,
    source_v: usize,
    closed_list: &mut HashSet<usize>,
    f_t: &mut Vec<u32>,
    t: &mut u32,
) -> () {
    let mut open_list = Vec::new();
    open_list.push(source_v);

    let mut visited_list = Vec::new();

    while !open_list.is_empty() {
        let cur_node = open_list.pop().unwrap();
        if closed_list.contains(&cur_node) {
            continue;
        }
        closed_list.insert(cur_node);
        visited_list.push(cur_node);

        for j in edge_list[cur_node].clone() {
            if !closed_list.contains(&j) {
                open_list.push(j);
            }
        }
    }

    for idx in visited_list.iter().rev() {
        //     println!("    vertex {} has finished {}", idx, *t);
        f_t[*idx] = *t;
        *t += 1;
    }


}

pub fn dfs_second_pass(
    edge_list: &Vec<Vec<usize>>,
    source_v: usize,
    closed_list: &mut HashSet<usize>,
    leaders_map: &mut HashMap<usize, Vec<usize>>,
) -> () {
    let mut open_list = Vec::new();
    open_list.push(source_v);

    while !open_list.is_empty() {
        let cur_node = open_list.pop().unwrap();
        if closed_list.contains(&cur_node) {
            continue;
        }
        closed_list.insert(cur_node);
        leaders_map.entry(source_v)
        .and_modify(|e| {e.push(cur_node)})
        .or_insert(vec![cur_node]);

        for j in edge_list[cur_node].clone() {
            if !closed_list.contains(&j) {
                open_list.push(j);
            }
        }
    }
}

pub fn compute_scc(graph: &Graph) -> Vec<usize> {
    let n = graph.num_v;

    // Variable used to track finishing times
    let mut t = 0;
    // Finishing time of each node
    let mut f_t = vec![std::u32::MAX; graph.num_v];

    let mut closed_list = HashSet::new();

    // Get finishing times
    for i in (0..n).rev() {
        if !closed_list.contains(&i) {
            dfs_first_pass(&graph.edge_list_rev, i, &mut closed_list, &mut f_t, &mut t);
        }
    }

    // Each index is the position to 
    // Eg. index 0 is the max node, up until index n which is the lowest node
    let mut nodes_idx_ranked: Vec<usize> = vec![0; n];

    for (idx, pos)  in f_t.iter().enumerate() {
        // println!("{} - {} - 1", graph.num_v, *pos as usize);
        let idx_pos = graph.num_v - (*pos as usize) - 1; //Try with -1 removed
        // println!("  {idx_pos}");
        nodes_idx_ranked[idx_pos] = idx;
    }

    let mut closed_list = HashSet::new();

    let mut leaders_map: HashMap<usize, Vec<usize>> = HashMap::new();

    // Get leaders and their followers
    for i in nodes_idx_ranked {
      if !closed_list.contains(&i) {
        dfs_second_pass(&graph.edge_list, i, &mut closed_list, &mut leaders_map);
      }
    }

    get_scc_sizes(leaders_map)

    // Check for cycles to same node in SCCs. e.g. A_prime => ... => A
}

// Get top n biggest SCCs
pub fn get_scc_sizes(leaders_map: HashMap<usize, Vec<usize>>) -> Vec<usize> {

    let mut scc_sizes = Vec::new();

    for (_, followers) in leaders_map {
        scc_sizes.push(followers.len());
    }
    scc_sizes.sort_by(|a, b| b.cmp(a));

    scc_sizes
}

pub fn has_cycle(leaders_map: &HashMap<usize, Vec<usize>>, num_v: usize) -> bool {

    for (_, followers) in leaders_map {
        let mut seen_hashset = HashSet::new();

        for f_idx in followers {
            let same_f_idx = unsigned_idx_to_same_node(*f_idx ,num_v);
            // println!("  f_idx to signed:{} to {}", f_idx, same_f_idx);
            // Check for cycles
            if seen_hashset.contains(&same_f_idx){
                return true;
            }
            seen_hashset.insert(same_f_idx);
        }
    }

    return false;
}

pub fn compute_satisfactibility(graph: &Graph) -> bool {
    let n = graph.num_v * 2;
    // Variable used to track finishing times
    let mut t = 0;
    // Finishing time of each node
    let mut f_t = vec![0; n];

    let mut closed_list = HashSet::new();

    // Get finishing times
    for i in (0..n).rev() {
        if !closed_list.contains(&i) {
            dfs_first_pass(&graph.edge_list_rev, i, &mut closed_list, &mut f_t, &mut t);
        }
    }

    // Each index is the position to 
    // Eg. index 0 is the max node, up until index n which is the lowest node
    let mut f_t_ranked: Vec<usize> = vec![0; n];

    for (idx, pos)  in f_t.iter().enumerate() {
        let idx_pos = n - (*pos as usize) - 1;
        f_t_ranked[idx_pos] = idx;
    }

    let mut closed_list = HashSet::new();

    let mut leaders_map: HashMap<usize, Vec<usize>> = HashMap::new();

    // Get leaders and their followers
    for i in f_t_ranked {
      if !closed_list.contains(&i) {
        dfs_second_pass(&graph.edge_list, i, &mut closed_list, &mut leaders_map);
      }
    }

    // Check for cycles to same node in SCCs. e.g. A_prime => ... => A
    !has_cycle(&leaders_map, graph.num_v)

}
