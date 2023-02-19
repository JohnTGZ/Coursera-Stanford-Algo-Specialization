use std::fs;
use std::cmp::{min, Ordering, Reverse};
use std::collections::{BinaryHeap};

pub struct Graph {
    pub adj_matrix: Vec<Vec<i32>>,
    pub adj_matrix_prime: Vec<Vec<i32>>,
    // a vector indexed by vertex id containing the vertices that leads into itself.
    pub vertex_in_edge_list: Vec<Vec<usize>>, 
    // a vector indexed by vertex id containing the vertices that it leads to.
    pub vertex_out_edge_list: Vec<Vec<usize>>, 
    pub num_v: usize,
    pub num_m: usize,
}

impl Graph {
    //0. Read input file and create adjacency matrix
    pub fn construct_adj_matrix(input_filepath: &str) -> Graph {
        println!("Constructing adjacency matrix from {input_filepath}");

        let contents = fs::read_to_string(input_filepath).unwrap();
        let content_vec: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

        // Get num_v and num_m
        let mut line_1 = content_vec[0].split(" ").map(|s| s.to_string());
        let (num_v, num_m) = (
            line_1.next().unwrap().parse::<usize>().unwrap(),
            line_1.next().unwrap().parse::<usize>().unwrap(),
        );

        println!("Number of (vertices, edges) = ({num_v}, {num_m})");

        // Ways to represent a graph:
        // 1. Adjacency matrix:
        //   Pros
        //      Fast lookup
        //   Cons
        //      n^2 space complexity
        // 2. Adjacency list:
        //   Pros
        //      Uses a linked list (saves space)
        //      Helps to find all vertices adjacent to a vertex easily
        //   Cons
        //      Lookup is slower than adjacency matrix
        // Alternative data structures to store graphs: https://stackoverflow.com/questions/49612663/is-there-any-other-data-structure-to-represent-graph-other-than-adjacency-list-o

        // Create an empty adjacent matrix (with an extra row and column as the starting vertex to form G')
        let mut adj_matrix: Vec<Vec<i32>> = vec![vec![std::i32::MAX; num_v + 1]; num_v + 1];
        let mut vertex_in_edge_list: Vec<Vec<usize>> = vec![Vec::new(); num_v+1];
        let mut vertex_out_edge_list: Vec<Vec<usize>> = vec![Vec::new(); num_v+1];

        for line in &content_vec[1..] {
            if !line.is_empty() {
                let mut line_itr = line.split(" ").map(|s| s.to_string());
                let (v_i, v_j, edge_cost) = (
                    line_itr.next().unwrap().parse::<usize>().unwrap(),
                    line_itr.next().unwrap().parse::<usize>().unwrap(),
                    line_itr.next().unwrap().parse::<i32>().unwrap(),
                );

                adj_matrix[v_i - 1][v_j - 1] = edge_cost;
                vertex_in_edge_list[v_j - 1].push(v_i - 1);
                vertex_out_edge_list[v_i - 1].push(v_j - 1);
            }
        }

        // Set diagonal to zero and edge costs between g' source to all other vertices to 0
        for v in 0..=num_v - 1 {
            adj_matrix[v][v] = 0;
            adj_matrix[num_v][v] = 0;
            vertex_in_edge_list[v].push(num_v);
            vertex_out_edge_list[num_v].push(v);
        }

        let adj_matrix_prime = adj_matrix.clone();

        Graph {
            adj_matrix,
            adj_matrix_prime,
            vertex_in_edge_list,
            vertex_out_edge_list,
            num_v,
            num_m,
        }
    }
}

pub fn bellman_ford(graph: &Graph, source_v: usize) -> (Vec<Vec<i32>>, bool) {
    // Single source length array, here we initialize all costs as infinity
    let mut ss_arr = vec![vec![std::i32::MAX; graph.num_v+1]; graph.num_v + 1];
    // Starting vertex set to 0 cost
    ss_arr[0][source_v] = 0;

    // Edge budget (Run for 1 extra iteration to check for negative cycles)
    for i in 1..=graph.num_v {
        // For each destination vertex
        for v in 0..=graph.num_v - 1 {

        // Iterate through all edges to v to get the minimum prospective cost
        let mut min_prospective_cost = std::i32::MAX;
        for w in graph.vertex_in_edge_list[v].clone() {
          min_prospective_cost = min(
                ss_arr[i - 1][w].saturating_add(graph.adj_matrix[w][v]),
                min_prospective_cost,
            );
        }

          ss_arr[i][v] = min(ss_arr[i - 1][v], min_prospective_cost);
        }
    }

    let mut has_negative_cycle = false;

    // Check for negative cycle
    // If no negative cycle, ss_arr[n-1, v] == ss_arr[n, v] for all vertices v. 
    for v in 0..=graph.num_v - 1 {
        if ss_arr[graph.num_v][v] != ss_arr[graph.num_v - 1][v] {
            has_negative_cycle = true;
            break;
        }
    }

    (ss_arr, has_negative_cycle)
}

pub fn djikstra(graph: &Graph, source_v: usize, apsp_arr: &mut Vec<Vec<i32>>) -> () {
  let mut g_cost = vec![std::i32::MAX; graph.num_v];
  // Set source vertex as cost 0
  g_cost[source_v] = 0;

  // Create min heap of nodes
  let mut open_list: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
  open_list.push(Reverse(Node::new(
    source_v, g_cost[source_v]
  )));

  while !open_list.is_empty(){
    let cur_node = open_list.pop().unwrap().0;
    let v_i = cur_node.vertex;
    
    for v_j in graph.vertex_out_edge_list[v_i].clone() {
      let alt_g_cost = g_cost[v_i].saturating_add( graph.adj_matrix_prime[v_i][v_j]);
      if alt_g_cost < g_cost[v_j] {
        g_cost[v_j] = alt_g_cost;
        open_list.push(Reverse(Node::new(
          v_j, g_cost[v_j]
        )));
      }
    }
  }

  // Update APSP Array
  for v_j in 0..=graph.num_v - 1 {
    apsp_arr[source_v][v_j] = g_cost[v_j];
  }

}

pub fn johnson_algo(graph: &mut Graph) -> (i32 ,bool) {
  // 2a. Run Bellman ford algorithm to get vertex weights and then
  // Reweigh vertex edges: c' = c + p_u - p_v and save in graph G'
  println!("Part 1");
  let (ss_arr, has_negative_cycles) = johnson_algo_p1(graph);

  if has_negative_cycles{
    return (0, true);
  }

  println!("Part 2");

  // 2b. For each vertex run djikstra's algorithm in G with edge lengths {c'}
  // 2c. for each pair u,v in G, return the shortest path distance d(u,v) := d'(u,v) - p_u + p_v
  let (_, shortest_length_btw_pair) = johnson_algo_p2(graph, &ss_arr);

  return (shortest_length_btw_pair, false);
}

pub fn johnson_algo_p1(graph: &mut Graph) -> (Vec<Vec<i32>>, bool) {
  // Add one more vertice for the extra source vertex in g prime
  graph.num_v += 1;
  println!("start bellman ford");

  // 2a. Run Bellman ford algorithm to get vertex weights and then
  let (ss_arr, has_negative_cycles) = bellman_ford(graph, graph.num_v-1);
  println!("Finsihed bellman ford");

  graph.num_v -= 1;

  // Iterate through adjacency list to readjust the edge costs
  for v_i in 0..=graph.num_v - 1 {
      for v_j in 0..=graph.num_v - 1 {
        // Reweigh vertex edges: c' = c + p_u - p_v and save in graph G'
        graph.adj_matrix_prime[v_i][v_j] = graph.adj_matrix_prime[v_i][v_j].saturating_add(ss_arr[graph.num_v][v_i] - ss_arr[graph.num_v][v_j]);
      }
  }
  println!("Finsihed adjustment");

  return (ss_arr, has_negative_cycles);
}

pub fn johnson_algo_p2(graph: &Graph, ss_arr: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>, i32) {
  let mut apsp_arr: Vec<Vec<i32>> = vec![vec![std::i32::MAX; graph.num_v]; graph.num_v]; // All pair shortest path array
  
  println!("Started Djikstra");

  // 2b. For each vertex run djikstra's algorithm in G with edge lengths {c'}
  for v_i in 0..=graph.num_v - 1 {
    djikstra(graph, v_i, &mut apsp_arr);
  }
    
  println!("Finished Djikstra");


  // Shortest length and their head and tail
  let (mut s_v_i, mut s_v_j, mut s_len) = (0, 0, std::i32::MAX);
  
  // 2c. for each pair u,v in G, return the shortest path distance d(u,v) := d'(u,v) - p_u + p_v
  for v_i in 0..=graph.num_v - 1 {
    for v_j in 0..=graph.num_v - 1 {
      if v_i != v_j {
        let len = apsp_arr[v_i][v_j].saturating_sub(ss_arr[graph.num_v][v_i]).saturating_add(ss_arr[graph.num_v][v_j]);
        if len < s_len {
          (s_v_i, s_v_j, s_len) = (v_i, v_j, len);
        }
      }
    }
  }
  return (apsp_arr, s_len);
}

#[derive(Eq)]
pub struct Node{
  pub vertex: usize,
  pub g_cost: i32,
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
      self.g_cost.cmp(&other.g_cost)
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
      self.g_cost == other.g_cost
  }
}

impl Node {
  fn new(vertex: usize, g_cost: i32) -> Node {
    Node { vertex, g_cost }
  }
}
