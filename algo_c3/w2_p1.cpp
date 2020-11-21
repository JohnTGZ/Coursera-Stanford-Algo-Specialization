//clustering
#include <fstream>
#include <iostream>
#include <vector>
#include <algorithm>

struct node{
  int idx;
  node* parent;
  node(int idx_, node* parent_): idx(idx_), parent(parent_)
    {}
  node(): node(-1, nullptr) {}
  // bool operator == (const node& rhs){
  //   return idx == rhs.idx;
  // }
};

struct edge{
  int u;
  int v;
  int cost;
  edge(int u_, int v_, int cost_): u(u_), v(v_), cost(cost_) {}

  // //in increasing cost
  // bool operator < (const edge& rhs){
  //   return cost <= rhs.cost();
  // }

};

struct comp_increasing{
  bool operator () (const edge& lhs, const edge& rhs){
    return lhs.cost > rhs.cost;
  }
};

struct comp_decreasing{
  bool operator () (const edge& lhs, const edge& rhs){
    return lhs.cost < rhs.cost;
  }
};

void print_edge_arr(std::vector<edge>& edge_arr){
  for (edge e : edge_arr){
    std::cout << "(" << e.u << ", " << e.v << ") = " << e.cost << std::endl;
  }
}


int Find(node& i){
  //return name of group that x belongs to
  //the name of group would be the parent idx
  return i.parent->idx;
}

void Union(node& i, node& j, node* nodes, std::vector<int>* clusters_arr, int& num_clusters){
  //fuse 2 groups ci and cj into a single one
  // printf("comparing nodes i(%d) and j(%d)\n", i.idx, j.idx);

  // printf("BEFORE: u(%d)->parent(%d), v(%d)->parent(%d) \n", i.idx, i.parent->idx, 
  //     j.idx, j.parent->idx);

  //its a cycle, therefore just skip
  if (Find(i) == Find(j)){
    printf("  CYCLE");
    return;
  }
  //different component -> merge
  else{
    //add nodes from i's cluster in to j's cluster
    int i_prev_parent_idx_ = i.parent->idx;
    // printf("  Transferring out from cluster[%d] \n", i_prev_parent_idx_);
    for (int node_idx : clusters_arr[i_prev_parent_idx_]){
      // printf("    %d\n", node_idx);
      clusters_arr[j.parent->idx].push_back(node_idx);
      nodes[node_idx].parent = j.parent;
    }
    // printf("  added to cluster[%d]: \n", j.parent->idx);
    // for (int node_idx : clusters_arr[j.parent->idx]){
      // printf("    %d \n", node_idx);
    // }
    num_clusters--;
  }

  // printf("AFTER: u(%d)->parent(%d), v(%d)->parent(%d) \n", i.idx, i.parent->idx, 
  //     j.idx, j.parent->idx);

}


int main(){

  int num_nodes;
  int edge_u, edge_v, edge_cost;
  int target_k = 4;
  
  //file input
  std::ifstream infile("clustering.txt");
  infile >> num_nodes;
  printf("number of nodes = %d, target_k = %d \n", num_nodes, target_k);

  //data structures
  std::vector<edge> edge_arr;
  node* nodes;
  nodes = new node[num_nodes+1]();
  std::vector<int>* clusters_arr;
  clusters_arr = new std::vector<int>[num_nodes+1];

  //populate arrays with initial values
  for (int m =1; m < num_nodes+1; ++m){
    nodes[m].idx = m;
    nodes[m].parent = &nodes[m];
    clusters_arr[m].push_back(m);
  }

  while (infile >> edge_u >> edge_v >> edge_cost){
    edge_arr.push_back(edge(edge_u, edge_v, edge_cost));
  }

  //sort edge_arr in increasing cost
  std::sort(edge_arr.begin(), edge_arr.end(), comp_increasing());

  //main clustering loop
  printf("size of edge array before clustering: %zu \n", edge_arr.size());
  int num_clusters = num_nodes; 
  while (num_clusters != target_k){
    edge edge_ = edge_arr.back();
    edge_arr.pop_back();
    // std::cout << "edge cost: " << edge_.cost << std::endl;
    
    Union(nodes[edge_.u], nodes[edge_.v], nodes, clusters_arr, num_clusters);
    // printf("num_clusters= %d, target_k = %d \n", num_clusters, target_k);
  }

  //extract min distance between clusters 
  int max = 0;
  int min = std::numeric_limits<int>::max();
  printf("size of edge array after clustering: %zu \n", edge_arr.size());
  for (edge edge_ : edge_arr) {
    //in the same cluster
    if (Find(nodes[edge_.u]) == Find(nodes[edge_.v])){
      printf("u(%d)->parent(%d), v(%d)->parent(%d) \n", edge_.u, edge_.v, 
            nodes[edge_.u].parent->idx, nodes[edge_.v].parent->idx);
      continue;
    }
    //different cluster
    else{
      if (min > edge_.cost ){
        min = edge_.cost;
      }
      if (max < edge_.cost){
        max = edge_.cost;
        // printf("min cost between clusters: %d \n", min);
      }
      // printf("edge cost(%d), max(%d), min(%d) \n", edge_.cost, max, min);
    }
  }

  // print_edge_arr(edge_arr);

  std::cout << "min distance between clusters " << min<< std::endl;
  std::cout << "max distance between clusters " << max<< std::endl;
  // std::cout << "num_clusters " << num_clusters << std::endl;

  
  return 0;
}

