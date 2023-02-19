use itertools::Itertools;
use plotters::prelude::*;
use std::fs;

/// Undirected graph
pub struct Graph {
    pub adj_matrix: Vec<Vec<f32>>,
    pub locations: Vec<(f32, f32)>,
    pub num_v: usize,
    pub num_m: usize,
}

impl Graph {
    /// Read input file and create adjacency matrix
    pub fn construct_adj_matrix(input_filepath: &str) -> Graph {
        println!("Constructing adjacency matrix from {input_filepath}");

        let contents = fs::read_to_string(input_filepath).unwrap();
        let content_vec: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

        // Get num_v and num_m
        let mut line_1 = content_vec[0].split(" ").map(|s| s.to_string());
        let num_v = line_1.next().unwrap().parse::<usize>().unwrap();

        let num_m = (0.5 * (num_v as f32) * (num_v as f32 - 1.0)) as usize;

        println!("Number of (vertices, edges) = ({num_v}, {num_m})");

        let mut locations = vec![(0.0, 0.0); num_v];

        // Create an empty adjacent matrix (with an extra row and column as the starting vertex to form G')
        let mut adj_matrix: Vec<Vec<f32>> = vec![vec![std::f32::MAX; num_v]; num_v];

        for v_i in 0..num_v {
            let line = content_vec[v_i + 1].clone();
            if !line.is_empty() {
                let mut line_itr = line.split(" ").map(|s| s.to_string());
                locations[v_i] = (
                    line_itr.next().unwrap().parse::<f32>().unwrap(),
                    line_itr.next().unwrap().parse::<f32>().unwrap(),
                );
            }
            adj_matrix[v_i][v_i] = 0.0;
        }

        for v_i in 0..num_v {
            for v_j in 0..num_v {
                if v_i != v_j {
                    let edge_cost = ((locations[v_i].0 - locations[v_j].0)
                        .hypot(locations[v_i].1 - locations[v_j].1))
                    .abs();
                    adj_matrix[v_i][v_j] = edge_cost;
                    adj_matrix[v_j][v_i] = edge_cost;
                }
            }
        }
        Graph {
            adj_matrix,
            locations,
            num_v,
            num_m,
        }
    }

    pub fn plot_vertices(&self, file_path: &str) -> () {
        let img_width: u32 = 1280;
        let img_height: u32 = 840;

        let root = BitMapBackend::new(&file_path, (img_width, img_height)).into_drawing_area();

        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .margin(5)
            .build_cartesian_2d(20000f32..28000.0f32, 9000f32..18000.0f32)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(
                self.locations
                    .iter()
                    .map(|(x, y)| Circle::new((*x, *y), 4, RGBAColor(255, 0, 0, 1.0).filled())),
            )
            .unwrap();

        root.present()
            .expect("Unable to write result to file, please make sure directory exists");
    }
}

pub fn get_arr_idx(bitmask: usize, dest_v: usize, num_v: usize) -> usize {
    return bitmask * num_v + dest_v;
}

/// Convert subset to an index integer
pub fn ss_to_idx(subset: &Vec<usize>) -> usize {
    let mut bin_num = 0;
    for idx in subset {
        bin_num |= 0b1 << *idx;
    }

    return bin_num ;
}

/// Get subset with the given element removed
pub fn ss_remove_elem(subset: &Vec<usize>, elem: usize) -> Vec<usize> {
    let mut new_subset = subset.clone();
    new_subset.retain(|v| *v != elem);
    new_subset
}

fn gen_subset_from_bitmasks(bitmask_comb: &Vec<u32>, n: usize) -> Vec<Vec<usize>> {
    let mut combinations = Vec::new();
    for mask in bitmask_comb {
  
      let mut comb = Vec::new(); // Stores a combination
  
      /* Check if the bit is set at each position in the mask.
         i.e If the mask is [ 1 0 1 1 ], we check for set bit at the below positions
         Pos  : 3 2 1 0
         Mask : 1 0 1 1
         --------------
                Y N Y Y
  
        Bit shifting is done as below
        1 0 1 1                1 0 1 1                 1 0 1 1                 1 0 1 1
            &                     &                       &                      &
        0 0 0 1 (left shift)   0 0 1 0 (left shift)    0 1 0 0  (left shift)   1 0 0 0
        --------               -------                 -------                 --------
        0 0 0 1 (0'th set )    0 0 1 0 (1'st set)      0 0 0 0 (2'nd NOT set)  1 0 0 0 (3'rd set) */
      
      // This is used to add the elements that are "1" in the bitmasks
      for pos in 0..n {
          if mask & (1 << pos) != 0{
              comb.push(pos);
          }   
      }   
      combinations.push(comb);
    }   
  
    combinations
  
}
  

fn gen_subset_from_bitmask(bitmask: &u32, n: usize) -> Vec<usize> {
    let mut comb = Vec::new(); // Stores a combination
  
    // This is used to add the elements that are "1" in the bitmasks
    for pos in 0..n {
        if bitmask & (1 << pos) != 0{
            comb.push(pos);
        }   
    }   
  
    comb
}
  

// Generate a bitmask combination of m number of "1"s fromn a bitmask of size n
fn gen_bitmasks(n: usize, m: usize) -> Vec<u32>{
let mut bitmask_comb = Vec::new();

for mask in 0u32..(1 << n) {

    if mask.count_ones() == (m as u32) {
    bitmask_comb.push(mask);
    } 
}   

bitmask_comb
}

pub fn tsp_algorithm(adj_matrix: &Vec<Vec<f32>>, num_v: usize) -> f32 {
    println!("Starting TSP Algorithm");
    let n_subsets = u32::pow(2, num_v as u32) as usize;

    // Create 2D array A,
    //  outer indexed by subsets S in {0, 1, 2, ..., n-1},
    //    let the integers be a binary representation of sets, meaning 0 === {0}, 1 === {1}, and 3 === {1,2}
    //  and inner indexed by destinations j in {0, 1, 2, ..., n-1}
    // let mut tsp_arr = vec![std::f32::MAX; n_subsets * num_v];
    let mut tsp_arr = vec![vec![std::f32::MAX; num_v]; n_subsets];
    println!("Created TSP Array with n_subsets = {n_subsets}");
    
    // Set the array value outer indexed by set {0}, and inner indexed by destination 1, to be 0
    tsp_arr[1][0] = 0.0;

    for m in 2..num_v+1 {
        //For each possible set size

        println!("Generating subsets of size {m}");
        let bitmasks = gen_bitmasks(num_v, m);
        println!("Generated subsets of size {m}");

        for bitmask in &bitmasks {
            if bitmask.trailing_ones() < 1 { // Skip subsets without {0}
                continue;
            }

            for j in 0..num_v {
                if bitmask & (1 << j) == 0{ 
                    continue;
                }   

                let mut min_cost = std::f32::MAX;
                let bitmask_j = bitmask ^ (1 << j);
                
                for k in 0..num_v {
                    if bitmask_j & (1 << k) == 0{
                        continue;
                    }   
                    min_cost = f32::min(
                        min_cost,
                        tsp_arr[bitmask_j as usize][k] + adj_matrix[k][j],
                    );
                }
                tsp_arr[*bitmask as usize][j] = min_cost;
            }
        }
    }

    let mut min_cost_tsp = std::f32::MAX;
    for j in 1..num_v {
        min_cost_tsp = f32::min(
            min_cost_tsp,
            tsp_arr[n_subsets - 1][j] + adj_matrix[j][0],
        );
    }

    return min_cost_tsp;
}
