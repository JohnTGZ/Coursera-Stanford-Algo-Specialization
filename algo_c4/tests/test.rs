use algo_c4::w1::{Graph, bellman_ford, djikstra, johnson_algo_p1, johnson_algo_p2, johnson_algo};

// #[cfg(test)]
// mod tests_w1 {
//   use super::*;

//   #[test]
//   fn test_construct_adj_matrix() {
//     let g1 = Graph::construct_adj_matrix("test_input/g1.txt");
//     let g2 = Graph::construct_adj_matrix("test_input/g2.txt");
//     let g3 = Graph::construct_adj_matrix("test_input/g3.txt");
  
//     assert_eq!(g1.num_v, 1000);
//     assert_eq!(g1.num_m, 47978);

//     assert_eq!(g1.adj_matrix.len(), 1001);
//     assert_eq!(g3.adj_matrix[0].len(), 1001);

//     assert_eq!(g2.adj_matrix[0][0], 0);
//     assert_eq!(g2.adj_matrix[0][1], 2);
//     assert_eq!(g2.adj_matrix[698][0], 41);
//   }

//   #[test]
//   fn test_bellman_ford_algorithm() {
//     let g4 = Graph::construct_adj_matrix("test_input/g4.txt");

//     let (ss_arr, has_negative_cycles) = bellman_ford(&g4, 0);

//     assert_eq!(ss_arr[0][0], 0);
//     assert_eq!(ss_arr[0][1], std::i32::MAX);

//     assert_eq!(ss_arr[g4.num_v][0], 0);
//     assert_eq!(ss_arr[g4.num_v][1], 2);
//     assert_eq!(ss_arr[g4.num_v][2], 4);
//     assert_eq!(ss_arr[g4.num_v][3], 6);
//     assert_eq!(ss_arr[g4.num_v][4], 3);
    
//     assert_eq!(has_negative_cycles, false);

//     let g5 = Graph::construct_adj_matrix("test_input/g5_negative_cycle.txt");

//     let (ss_arr, has_negative_cycles) = bellman_ford(&g5, 0);

//     assert_eq!(ss_arr[0][0], 0);
//     assert_eq!(ss_arr[0][1], std::i32::MAX);
    
//     assert_eq!(has_negative_cycles, true);

//   }


//   #[test]
//   fn test_johnson_algo_p1() {
//     let mut g6 = Graph::construct_adj_matrix("test_input/g6_reweighing.txt");

//     let (ss_arr, has_negative_cycles) = johnson_algo_p1(&mut g6);

//     assert_eq!(has_negative_cycles, false);

//     for v_i in 0..=g6.num_v - 1 {
//       for v_j in 0..=g6.num_v{
//         println!("ss_arr[{}][{}] == {}", v_i, v_j, ss_arr[v_i][v_j]);
//       }
//     }

//     assert_eq!(ss_arr[g6.num_v][0], 0);
//     assert_eq!(ss_arr[g6.num_v][1], -2);
//     assert_eq!(ss_arr[g6.num_v][2], -3);
//     assert_eq!(ss_arr[g6.num_v][3], -1);
//     assert_eq!(ss_arr[g6.num_v][4], -6);
//     assert_eq!(ss_arr[g6.num_v][5], 0);

//     assert_eq!(g6.adj_matrix_prime[g6.num_v][0], 0);
//     assert_eq!(g6.adj_matrix_prime[0][1], 0);
//     assert_eq!(g6.adj_matrix_prime[2][0], 1);
//     assert_eq!(g6.adj_matrix_prime[1][2], 0);

//     assert_eq!(g6.adj_matrix_prime[2][4], 0);
//     assert_eq!(g6.adj_matrix_prime[2][3], 0);

//     assert_eq!(g6.adj_matrix_prime[5][3], 2);

//   }

  
//   #[test]
//   fn test_djikstra() {
//     let g7 = Graph::construct_adj_matrix("test_input/g7_djikstra.txt");

//     let mut apsp_arr: Vec<Vec<i32>> = vec![vec![std::i32::MAX; g7.num_v]; g7.num_v]; // All pair shortest path array
//     djikstra(&g7, 0, &mut apsp_arr);

//     assert_eq!(apsp_arr[0][0], 0);
//     assert_eq!(apsp_arr[0][1], 1);
//     assert_eq!(apsp_arr[0][2], 3);
//     assert_eq!(apsp_arr[0][3], 6);

//   }

//   #[test]
//   fn test_johnson_algo_p2() {
//     let g7 = Graph::construct_adj_matrix("test_input/g7_djikstra.txt");

//     let ss_arr = vec![vec![std::i32::MAX; g7.num_v]; g7.num_v + 1];

//     let (apsp_arr, shortest_length_btw_pair) = johnson_algo_p2(&g7, &ss_arr);
   
//     assert_eq!(apsp_arr[0][0], 0);
//     assert_eq!(apsp_arr[0][1], 1);
//     assert_eq!(apsp_arr[0][2], 3);
//     assert_eq!(apsp_arr[0][3], 6);

//     assert_eq!(apsp_arr[1][0], std::i32::MAX);
//     assert_eq!(apsp_arr[1][1], 0);
//     assert_eq!(apsp_arr[1][2], 2);
//     assert_eq!(apsp_arr[1][3], 5);

//     assert_eq!(apsp_arr[2][0], std::i32::MAX);
//     assert_eq!(apsp_arr[2][1], std::i32::MAX);
//     assert_eq!(apsp_arr[2][2], 0);
//     assert_eq!(apsp_arr[2][3], 3);

//     assert_eq!(apsp_arr[3][0], std::i32::MAX);
//     assert_eq!(apsp_arr[3][1], std::i32::MAX);
//     assert_eq!(apsp_arr[3][2], std::i32::MAX);
//     assert_eq!(apsp_arr[3][3], 0);
//   }

//   #[test]
//   fn test_johnson_algo() {
//     let mut g6 = Graph::construct_adj_matrix("test_input/g6_reweighing.txt");

//     let (shortest_length_btw_pair, has_negative_cycles) = johnson_algo(&mut g6);
//     assert_eq!(has_negative_cycles, false);

//     assert_eq!(shortest_length_btw_pair, -6);
//   }

// }


// #[cfg(test)]
// mod tests_w2 {
//   use super::*;
  
//   #[test]
//   fn test_construct_matrix() {
//     let tsp_g = algo_c4::w2::Graph::construct_adj_matrix("test_input/tsp.txt");

//     assert_eq!(tsp_g.num_v, 25);
//     assert_eq!(tsp_g.num_m, 300);

//     assert_eq!(tsp_g.locations[0], (20833.3333, 17100.0000));
//     assert_eq!(tsp_g.locations[14], (26133.3333, 14500.0000));
//     assert_eq!(tsp_g.locations[24], (27233.3333, 10450.0000));

//     assert_eq!(tsp_g.adj_matrix[0][1], 74.53531);
//     assert_eq!(tsp_g.adj_matrix[1][0], 74.53531);

//     assert_eq!(tsp_g.adj_matrix[8][19], 4179.746154439951);
//     assert_eq!(tsp_g.adj_matrix[19][8], 4179.746154439951);
//   }

//   #[test]
//   fn test_help_functions() {
//     assert_eq!(algo_c4::w2::ss_to_idx(&vec![]), 0);
//     assert_eq!(algo_c4::w2::ss_to_idx(&vec![0]), 1);
//     assert_eq!(algo_c4::w2::ss_to_idx(&vec![1]), 2);
//     assert_eq!(algo_c4::w2::ss_to_idx(&vec![0,1]), 3);

//     assert_eq!(algo_c4::w2::ss_to_idx(&vec![0,1,2,3]), 15);

//     assert_eq!(algo_c4::w2::ss_to_idx(&vec![0,1,2,3,4,5,6,7,8,9]), 1024-1);

//     let subset = vec![1,2,3,4,5,6];
//     assert_eq!(algo_c4::w2::ss_remove_elem(&subset, 3), vec![1,2,4,5,6]);
//     assert_eq!(algo_c4::w2::ss_remove_elem(&subset, 2), vec![1,3,4,5,6]);
//   }

//   #[test]
//   fn test_tsp() {
//     let tsp_g3 = algo_c4::w2::Graph::construct_adj_matrix("test_input/tsp_test2.txt");
//     let tsp_g4 = algo_c4::w2::Graph::construct_adj_matrix("test_input/tsp_test3.txt");
//     let tsp_g5 = algo_c4::w2::Graph::construct_adj_matrix("test_input/tsp_test4.txt");

//     let dist_g3 = algo_c4::w2::tsp_algorithm(&tsp_g3.adj_matrix, tsp_g3.num_v);
//     let dist_g4 = algo_c4::w2::tsp_algorithm(&tsp_g4.adj_matrix, tsp_g4.num_v);
//     let dist_g5 = algo_c4::w2::tsp_algorithm(&tsp_g5.adj_matrix, tsp_g5.num_v);

//     assert_eq!(dist_g3, 10.2426405);
//     assert_eq!(dist_g4, 12.364804);
//     assert_eq!(dist_g5, 14.00);

//   }
  
// }


#[cfg(test)]
mod tests_w4 {
  use super::*;

  #[test]
  fn test_construct_clauses() {
    let clauses = algo_c4::w4::Clauses::construct("test_input/2sat_test/2sat_10_20.txt");
    assert_eq!(clauses.num_v, 20);

    assert_eq!(clauses.clauses[0], (-3, -10));
    assert_eq!(clauses.clauses[5], (-13, -17));
  }

  #[test]
  fn test_randomization() {
    println!("Test randomization");
    let n = 2;
    let mut assignments = algo_c4::w4::generate_random_assignments(n);
    for assignment in &assignments {
      println!("{:?}", assignment);
    }

    algo_c4::w4::flip_assignment(&mut assignments[0]);

    for assignment in &assignments {
      println!("{:?}", assignment);
    }
  }


}