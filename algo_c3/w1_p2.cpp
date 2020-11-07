//scheduling 
#include <fstream>
#include <iostream>
#include <vector>
#include <algorithm>

struct edge{
  int u;
  int v;
  int cost;
  edge(int u, int v, int cost): 
    u(u), v(v), cost(cost)
    {}

  //in increasing order
  bool operator < (const edge& rhs){
    return cost < rhs.cost;
  }
};

typedef std::pair<int, int> intpair;

int main(){
  int node1, node2, edge_cost;
  int num_nodes, num_edges;
  std::vector<int> X;
  std::vector<edge> T;
  
  std::vector<edge> edges;

  std::ifstream infile("edges.txt");
  //starting element
  infile >> num_nodes >> num_edges;
  std::cout << "number of nodes " << num_nodes << " and number of edges " << num_edges << std::endl;

  while (infile >> node1 >> node2 >> edge_cost){
    // std::cout << node1 << ", " << node2 << ", " << edge_cost << std::endl;
    edges.push_back(edge(node1, node2, edge_cost));
  }

  //sort edges
  std::sort(edges.begin(), edges.end());

  //add arbitrary vertex to X
  
  X.push_back(edges[0].u);

  std::vector<int>::iterator found_u;
  std::vector<int>::iterator found_v;

  int current_v;

  //while X is not a minimum spanning tree, increase T as cheaply as possible
  while (X.size() != num_nodes){
    
    for (edge e_ : edges){
      found_u = std::find(X.begin(), X.end(), e_.u);
      found_v = std::find(X.begin(), X.end(), e_.v);

      //found a cheapest edge
      if ( (found_u!=X.end()) ^ (found_v!=X.end()) ){
        //if u is inside X
        if (found_u!=X.end()){
          current_v = e_.v;
        }
        else{
          current_v = e_.u;
        }
        // current_edge = e_;
        T.push_back(e_);
        break;
      }
    }


    //add edges and vertices
    // T.push_back(current_edge);
    X.push_back(current_v);
  }

  //calculate overall costs
  long long total_cost = 0;
  for (edge t_ : T ){
    total_cost += t_.cost;
  }
  
  std::cout << "Total cost: " << total_cost << std::endl;

  return 0;
}
