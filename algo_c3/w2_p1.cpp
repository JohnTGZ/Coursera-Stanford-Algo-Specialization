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

void Union(int& i, int& j, node* nodes, std::vector<int>* clusters_arr){
  //fuse 2 groups ci and cj into a single one
  // printf("comparing nodes i(%d) and j(%d)\n", i.idx, j.idx);

  //its a cycle, therefore just skip
  if (Find(nodes[i]) == Find(nodes[j])){
    return;
  }
  //different component -> merge
  else{
    //add nodes from i's cluster in to j's cluster
    for (int node_idx : clusters_arr[nodes[i].parent->idx]){
      clusters_arr[nodes[j].parent->idx].push_back(node_idx);
      nodes[node_idx].parent = nodes[i].parent;
    }
    clusters_arr[nodes[i].parent->idx].clear();
    clusters_arr[nodes[i].parent->idx].push_back(-1);
  }
}

bool checkClustersTarget(std::vector<int>* clusters_arr, int target_k, int num_nodes){
  int sum = 0;
  for (int i= 1; i < num_nodes +1; ++i){
    if (clusters_arr[i][0] == -1){
      sum +=1;
    }
  }
  return (num_nodes - sum) == target_k;
}

int main(){

  std::ifstream infile("clustering.txt");
  int num_nodes;
  infile >> num_nodes;

  std::cout << "number of nodes: " << num_nodes << std::endl;

  int edge_u, edge_v, edge_cost;
  int target_k = 4;

  std::vector<edge> edge_arr;

  //nodes array
  node* nodes;
  nodes = new node[num_nodes+1]();
  //clusters array
  std::vector<int>* clusters_arr;
  clusters_arr = new std::vector<int>[num_nodes+1];
  //populate arrays with initial values
  for (int m =1; m < num_nodes+1; ++m){
    nodes[m].idx = m;
    nodes[m].parent = &nodes[m];
    clusters_arr[m].push_back(m);
  }

  // for (int i = 1; i < num_nodes+1; ++i){
  //   for (int idx : clusters_arr[i]){
  //     printf("%d \n", idx);
  //   }
  // }

  while (infile >> edge_u >> edge_v >> edge_cost){
    edge_arr.push_back(edge(edge_u, edge_v, edge_cost)) ;
  }

  //sort edge_arr in increasing cost
  std::sort(edge_arr.begin(), edge_arr.end(), comp_increasing());

  int num_clusters = num_nodes; 

  while (!checkClustersTarget(clusters_arr, target_k, num_nodes)){
    //let p,q be the closest PAIR of seperated points
    // this will be the last element of the sorted array
    
    edge edge_ = edge_arr.back();
    edge_arr.pop_back();
    std::cout << "edge cost: " << edge_.cost << std::endl;
    
    Union(edge_.u, edge_.v, nodes, clusters_arr);
  }

  // print_edge_arr(edge_arr);

  edge max_edge1 = edge_arr.back();
  edge max_edge2 = edge_arr[0];

  std::cout << "max distance " << max_edge2.cost << std::endl;
  // std::cout << "num_clusters " << num_clusters << std::endl;

  
  return 0;
}

