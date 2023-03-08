use algo_c4::w1::{bellman_ford, djikstra, johnson_algo, johnson_algo_p1, johnson_algo_p2, Graph};
use std::collections::{HashMap, HashSet};

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

// #[cfg(test)]
// mod tests_w4 {
//   use super::*;

//   #[test]
//   fn test_construct_clauses() {
//     let clauses = algo_c4::w4::Clauses::construct("test_input/2sat_test/10_20_false.txt");
//     assert_eq!(clauses.num_v, 20);

//     assert_eq!(clauses.clauses[0], (-3, -10));
//     assert_eq!(clauses.clauses[5], (-13, -17));
//   }

//   #[test]
//   fn test_randomized_flip() {
//     let mut assignments = algo_c4::w4::generate_random_assignments(3);

//     let (asn_0, asn_2) = (assignments[0], assignments[2]);

//     algo_c4::w4::flip_assignment(&mut assignments, (0,2));

//     assert_ne!((asn_0, asn_2) ,(assignments[0], assignments[2]));
//   }

//   // #[test]
//   // fn test_check_satifactibility() {
//   //   let clauses_1 = algo_c4::w4::Clauses {
//   //     num_v: 3,
//   //     clauses: vec![(1,-2), (-3,-2)],
//   //   };

//   //   let assignments = vec![true, false, false];
//   //   let (satisfied, (unsat_idx_l, unsat_idx_r)) = clauses_1.check_satisfied(&assignments);

//   //   assert_eq!(satisfied, true);

//   //   let assignments = vec![true, false, true];
//   //   let (satisfied, (unsat_idx_l, unsat_idx_r)) = clauses_1.check_satisfied(&assignments);

//   //   assert_eq!(satisfied, false);
//   //   assert_eq!((unsat_idx_l, unsat_idx_r), (2, 1));
//   // }

//   #[test]
//   fn test_papadimitriou_2sat() {
//     let clauses_1 = algo_c4::w4::Clauses::construct("test_input/2sat_test/1_2_true.txt");
//     let clauses_2 = algo_c4::w4::Clauses::construct("test_input/2sat_test/2_2_false.txt");

//     let clauses_3 = algo_c4::w4::Clauses::construct("test_input/2sat_test/10_20_false.txt");
//     let clauses_4 = algo_c4::w4::Clauses::construct("test_input/2sat_test/11_40_true.txt");
//     let clauses_5 = algo_c4::w4::Clauses::construct("test_input/2sat_test/12_40_false.txt");

//     assert_eq!(algo_c4::w4::papadimitriou_2_sat(&clauses_1), true);
//     assert_eq!(algo_c4::w4::papadimitriou_2_sat(&clauses_2), false);

//     assert_eq!(algo_c4::w4::papadimitriou_2_sat(&clauses_3), false);
//     assert_eq!(algo_c4::w4::papadimitriou_2_sat(&clauses_4), true);
//     assert_eq!(algo_c4::w4::papadimitriou_2_sat(&clauses_5), false);

//   }
// }

#[cfg(test)]
mod tests_w4 {
    use super::*;

    #[test]
    fn test_construct_scc_graph() {
        let graph = algo_c4::w4::Graph::construct_scc("test_input/scc/lecture_example.txt");

        assert_eq!(graph.num_v, 9);

        assert_eq!(graph.edge_list[6], vec![0]);
        assert_eq!(graph.edge_list[0], vec![3]);

        assert_eq!(graph.edge_list_rev[0], vec![6]);
        assert_eq!(graph.edge_list_rev[6], vec![3, 8]);
    }

    #[test]
    fn test_first_pass() {
        let graph = algo_c4::w4::Graph::construct_scc("test_input/scc/lecture_example.txt");

        // Variable used to track finishing times
        let mut t = 0;
        // Finishing time of each node
        let mut f_t = vec![std::u32::MAX; graph.num_v];

        let mut closed_list = HashSet::new();

        // Get finishing times
        for i in (0..graph.num_v).rev() {
            if !closed_list.contains(&i) {
                algo_c4::w4::dfs_first_pass(
                    &graph.edge_list_rev,
                    i,
                    &mut closed_list,
                    &mut f_t,
                    &mut t,
                );
            }
        }

        assert_eq!(f_t, vec![6, 2, 0, 7, 1, 4, 8, 3, 5]);
    }

    #[test]
    fn test_second_pass() {
        let graph = algo_c4::w4::Graph::construct_scc("test_input/scc/lecture_example.txt");

        let f_t = vec![6, 2, 0, 7, 1, 4, 8, 3, 5];

        // Each index is the position to
        // Eg. index 0 is the max node, up until index n which is the lowest node
        let mut nodes_idx_ranked: Vec<usize> = vec![0; f_t.len()];

        for (idx, pos) in f_t.iter().enumerate() {
            nodes_idx_ranked[graph.num_v - *pos as usize - 1] = idx;
        }

        let mut closed_list = HashSet::new();

        let mut leaders_map: HashMap<usize, Vec<usize>> = HashMap::new();

        println!("nodes_idx_ranked: {:?}", nodes_idx_ranked);

        // Get finishing times
        for i in nodes_idx_ranked {
            if !closed_list.contains(&i) {
                algo_c4::w4::dfs_second_pass(
                    &graph.edge_list,
                    i,
                    &mut closed_list,
                    &mut leaders_map,
                );
            }
        }

        assert_eq!(leaders_map[&6], vec![6, 0, 3]);
        assert_eq!(leaders_map[&8], vec![8, 2, 5]);
        assert_eq!(leaders_map[&7], vec![7, 4, 1]);

        let scc_sizes = algo_c4::w4::get_scc_sizes(leaders_map);

        assert_eq!(scc_sizes, vec![3, 3, 3]);
    }

    #[test]
    fn test_scc() {
      let graph_1 = algo_c4::w4::Graph::construct_scc("test_input/scc/input_mostlyCycles_1_8.txt");
      let graph_2 = algo_c4::w4::Graph::construct_scc("test_input/scc/input_mostlyCycles_2_8.txt");
      let graph_3 = algo_c4::w4::Graph::construct_scc("test_input/scc/input_mostlyCycles_10_32.txt");
      let graph_4 = algo_c4::w4::Graph::construct_scc("test_input/scc/input_mostlyCycles_18_128.txt");
      let graph_5 = algo_c4::w4::Graph::construct_scc("test_input/scc/input_mostlyCycles_32_800.txt");
      
      let scc_1 = algo_c4::w4::compute_scc(&graph_1);
      let scc_2 = algo_c4::w4::compute_scc(&graph_2);
      let scc_3 = algo_c4::w4::compute_scc(&graph_3);
      let scc_4 = algo_c4::w4::compute_scc(&graph_4);
      let scc_5 = algo_c4::w4::compute_scc(&graph_5);

      assert_eq!(scc_1, vec![4,2,2]);
      assert_eq!(scc_2, vec![8]);
      assert_eq!(scc_3, vec![11,10,5,4,1,1]);
      assert_eq!(scc_4, vec![71,37,14,6]);
      assert_eq!(scc_5, vec![507,119,109,28,21, 16]);
    }

    #[test]
    fn test_scc_2sat() {
      let graph_1 = algo_c4::w4::Graph::construct("test_input/2sat_test/1_2_true.txt");
      let graph_2 = algo_c4::w4::Graph::construct("test_input/2sat_test/2_2_false.txt");
      let graph_3 = algo_c4::w4::Graph::construct("test_input/2sat_test/3_4_true.txt");
      let graph_10 = algo_c4::w4::Graph::construct("test_input/2sat_test/10_20_false.txt");
      let graph_11 = algo_c4::w4::Graph::construct("test_input/2sat_test/11_40_true.txt");
      let graph_12 = algo_c4::w4::Graph::construct("test_input/2sat_test/12_40_false.txt");
      let graph_32 = algo_c4::w4::Graph::construct("test_input/2sat_test/32_10000_false.txt");


      assert_eq!(algo_c4::w4::compute_satisfactibility(&graph_1), true);
      assert_eq!(algo_c4::w4::compute_satisfactibility(&graph_2), false);
      assert_eq!(algo_c4::w4::compute_satisfactibility(&graph_3), true);
      assert_eq!(algo_c4::w4::compute_satisfactibility(&graph_10), false);
      assert_eq!(algo_c4::w4::compute_satisfactibility(&graph_11), true);
      assert_eq!(algo_c4::w4::compute_satisfactibility(&graph_12), false);
      assert_eq!(algo_c4::w4::compute_satisfactibility(&graph_32), false);
    }

}
