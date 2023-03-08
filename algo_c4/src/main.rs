// use algo_c4::w1::{Graph, johnson_algo};

// Week 1: Johnson Algorithm

// fn main() {
//     // let mut g1 = Graph::construct_adj_matrix("test_input/g1.txt");
//     // let mut g2 = Graph::construct_adj_matrix("test_input/g2.txt");
//     let mut g3 = Graph::construct_adj_matrix("test_input/g3.txt");

//     println!("Starting johnson algo");
//     // let (g1_s_len, g1_has_neg_cycles) = johnson_algo(&mut g1);
//     // let (g2_s_len, g2_has_neg_cycles) = johnson_algo(&mut g2);
//     let (g3_s_len, g3_has_neg_cycles) = johnson_algo(&mut g3);

//     // println!("G1, length: {g1_s_len}, negative cycles? {g1_has_neg_cycles}");
//     // println!("G2, length: {g2_s_len}, negative cycles? {g2_has_neg_cycles}");
//     println!("G3, length: {g3_s_len}, negative cycles? {g3_has_neg_cycles}");
// }   

// Week 2: TSP

// fn main () {
//     let tsp_g = algo_c4::w2::Graph::construct_adj_matrix("test_input/tsp.txt");

//     tsp_g.plot_vertices("test_plots/tsp_map.png");

//     let tsp_dist = algo_c4::w2::tsp_algorithm(&tsp_g.adj_matrix, tsp_g.num_v);
//     println!("TSP dist: {}", tsp_dist);
// }

// // Week 4: 2SAT
// fn main() {
//     let clauses_1 = algo_c4::w4::Clauses::construct("test_input/2sat/2sat1.txt");
//     // let clauses_2 = algo_c4::w4::Clauses::construct("test_input/2sat/2sat2.txt");
//     // let clauses_3 = algo_c4::w4::Clauses::construct("test_input/2sat/2sat3.txt");
//     // let clauses_4 = algo_c4::w4::Clauses::construct("test_input/2sat/2sat4.txt");
//     // let clauses_5 = algo_c4::w4::Clauses::construct("test_input/2sat/2sat5.txt");
//     // let clauses_6 = algo_c4::w4::Clauses::construct("test_input/2sat/2sat6.txt");

//     println!("2Sat test 1: {} ", algo_c4::w4::papadimitriou_2_sat(&clauses_1));
//     // println!("2Sat test 2: {} ", algo_c4::w4::papadimitriou_2_sat(&clauses_2));
//     // println!("2Sat test 3: {} ", algo_c4::w4::papadimitriou_2_sat(&clauses_3));
//     // println!("2Sat test 4: {} ", algo_c4::w4::papadimitriou_2_sat(&clauses_4));
//     // println!("2Sat test 5: {} ", algo_c4::w4::papadimitriou_2_sat(&clauses_5));
//     // println!("2Sat test 6: {} ", algo_c4::w4::papadimitriou_2_sat(&clauses_6));

// }



// Week 4: 2SAT
fn main() {
    let graph_1 = algo_c4::w4::Graph::construct("test_input/2sat/2sat1.txt");
    let graph_2 = algo_c4::w4::Graph::construct("test_input/2sat/2sat2.txt");
    let graph_3 = algo_c4::w4::Graph::construct("test_input/2sat/2sat3.txt");
    let graph_4 = algo_c4::w4::Graph::construct("test_input/2sat/2sat4.txt");
    let graph_5 = algo_c4::w4::Graph::construct("test_input/2sat/2sat5.txt");
    let graph_6 = algo_c4::w4::Graph::construct("test_input/2sat/2sat6.txt");

    println!("2sat 1: {}", algo_c4::w4::compute_satisfactibility(&graph_1));
    println!("2sat 2: {}", algo_c4::w4::compute_satisfactibility(&graph_2));
    println!("2sat 3: {}", algo_c4::w4::compute_satisfactibility(&graph_3));
    println!("2sat 4: {}", algo_c4::w4::compute_satisfactibility(&graph_4));
    println!("2sat 5: {}", algo_c4::w4::compute_satisfactibility(&graph_5));
    println!("2sat 6: {}", algo_c4::w4::compute_satisfactibility(&graph_6));

}