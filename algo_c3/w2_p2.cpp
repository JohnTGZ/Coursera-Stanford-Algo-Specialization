//clustering
#include <fstream>
#include <iostream>
#include <vector>
#include <algorithm>
#include <map>
#include <list>


int binaryToInt(std::string bin_str){
  return std::stoi(bin_str, 0, 2);
}

//n: number 
//p: position
//b: binary value
int modifyBitPos(int n, int p, int b){
  int mask = 1 << p; //create the mask
  return (n & ~mask) | ((b << p) & mask);
}

//
int binXOR(int x, int y){
  int res = 0; // Initialize result
    
  // Assuming 32-bit Integer 
  for (int i = 31; i >= 0; i--)                     
  {
      // Find current bits in x and y
      bool b1 = x & (1 << i);
      bool b2 = y & (1 << i);
      
      // If both are 1 then 0 else xor is same as OR
      bool xoredBit = (b1 & b2) ? 0 : (b1 | b2);          

      // Update result
      res <<= 1;
      res |= xoredBit;
  }
  return res;
}

struct node{
  int idx;
  node* parent;
  node(int idx_, node* parent_): idx(idx_), parent(parent_)
    {}
  node(): node(-1, nullptr) {}
};

int Find(node& i){
  return i.parent->idx;
}

// void Union(node& i, node& j, 
//           node* nodes, std::vector<int>* clusters_arr, int& num_clusters){
//   int i_idx = i.idx;
//   int j_idx = j.idx;
//   int i_parent_idx = i.parent->idx;
//   int j_parent_idx = j.parent->idx;

//   // printf("BEFORE: i(%d)->parent(%d), j(%d)->parent(%d) \n", 
//   //     i_idx, i_parent_idx, 
//   //     j_idx, j_parent_idx);

//   if (Find(i) == Find(j)){
//     // printf("  cycle \n");
//     return;
//   }
//   else {
//     // printf("  NOT cycle \n");
//     for (int node_idx : clusters_arr[i_parent_idx]){
//       clusters_arr[j_parent_idx].push_back(node_idx); //add indexes to the cluster of (j)
//       nodes[node_idx].parent = j.parent; //Each node of the other cluster (i), will inherit parent of (j)
//     }

//     num_clusters--;    
//   }   

//   // printf("  AFTER: i(%d)->parent(%d), j(%d)->parent(%d) \n", 
//   //     i.idx, i.parent->idx,
//   //     j.idx, j.parent->idx);
// }

void Union(std::vector<node*>& i, std::vector<node*>& j, 
          node* nodes, std::vector<int>* clusters_arr, int& num_clusters){

  // printf("Start Union \n");
  int i_back_idx = i.back()->idx;
  int j_back_idx = j.back()->idx;
  int i_parent_idx = i.back()->parent->idx;
  int j_parent_idx = j.back()->parent->idx;

  // printf("BEFORE: i_back_idx(%d)->parent(%d), j_back_idx(%d)->parent(%d) \n", 
  //     i_back_idx, i_parent_idx, 
  //     j_back_idx, j_parent_idx);

  if (Find(*i.back()) == Find(*j.back())){
    // printf("  cycle \n");
    return;
  }
  else {
    // printf("  NOT cycle \n");
    for (int node_idx : clusters_arr[i_parent_idx]){
      clusters_arr[j_parent_idx].push_back(node_idx); //add indexes to the cluster of (j)
      nodes[node_idx].parent = j.back()->parent; //Each node of the other cluster (i), will inherit parent of (j)
    }

    num_clusters--;    
  }   

  // printf("  AFTER: i_back_idx(%d)->parent(%d), j_back_idx(%d)->parent(%d) \n", 
  //     i.back()->idx, i.back()->parent->idx,
  //     j.back()->idx, j.back()->parent->idx);
}

int main(){
  //file input
  std::string filepath = "clustering_big.txt";
  std::ifstream inFile(filepath);
  if (!inFile){
    std::cerr << "unable to open file at " << filepath << std::endl;
    return -1;
  }

  //data structures
  int num_nodes, num_bits;
  std::map<int, std::string> node_to_dist_bin; //node to binary distance

  std::map<int, std::vector<node*>> dist_to_node_map; //dist to node mapping 1
  
  // std::multimap<int, node*> dist_to_node_map_multi; //dist to node mapping 2

  //extract data from file
  inFile >> num_nodes >> num_bits;
  printf("num_nodes(%d), num_bits(%d)\n", num_nodes, num_bits);
  int node_idx_= 1;
   std::string line;
  std::getline(inFile, line, '\n'); //skip first line
  while (std::getline(inFile, line, '\n')){
    node_to_dist_bin[node_idx_] = line;
    node_idx_++;
  }

  //DEBUG
  // int test_itr = 0;
  // for (auto i = node_to_dist_bin.begin(); i != node_to_dist_bin.end(); i++){
  //   if (test_itr++ < 2)
  //     printf("hamming[%d] = %s \n", i->first, i->second.c_str());
  // }

  //more data structs for keeping track of parents and clusters
  node* nodes;
  nodes = new node[num_nodes+1]();
  std::vector<int>* clusters_arr;
  clusters_arr = new std::vector<int>[num_nodes+1];

  //create mapping from [distance integers] to [node IDs]
  //there will be multiple node IDs for the same distance integers
  for (auto it = node_to_dist_bin.begin(); it != node_to_dist_bin.end(); it++){
    int node_idx = it->first;
    std::string dist_bin = it->second;
    int dist = binaryToInt(dist_bin); //convert binary string to integer

    //initialize data structs for union find
    nodes[node_idx].idx = node_idx;
    nodes[node_idx].parent = &nodes[node_idx]; // set parent of each node to itself
    // clusters_arr[node_idx].push_back(node_idx);
    //add to map
    dist_to_node_map[dist].push_back(&nodes[node_idx]);
    // dist_to_node_map_multi.insert(std::pair<int, node*> (dist, &nodes[node_idx]));
  }

  std::vector<int> bit_mask_arr0 {0};
  std::vector<int> bit_mask_arr1;
  std::vector<int> bit_mask_arr2;
  
  //create bit masks for 1 positions
  int bit_mask = 1;
  bit_mask_arr1.push_back(bit_mask);
  for (int i = 0; i < num_bits-1; ++i){
    bit_mask = bit_mask << 1;
    bit_mask_arr1.push_back(bit_mask);
  }
  //create bit masks for 2 postions
  for (int i = 0; i < num_bits-1; ++i){
    int bit_mask1 = 1 << i;
    for (int j = i+1; j < num_bits; ++j){
      bit_mask_arr2.push_back(modifyBitPos(bit_mask1, j, 1));
    }
  }

  //join bit mask arrays together 
  std::vector<int> bit_mask_arr_all(bit_mask_arr0); //copy constructor
  bit_mask_arr_all.insert(bit_mask_arr_all.end(), bit_mask_arr1.begin(), bit_mask_arr1.end());
  bit_mask_arr_all.insert(bit_mask_arr_all.end(), bit_mask_arr2.begin(), bit_mask_arr2.end());

  // DEBUG
  // printf("bit_mask_arr_all.size(): %zu \n", bit_mask_arr_all.size());
  // for (int mask : bit_mask_arr0 ){
  //   printf("bitmask 0 %d \n", mask );
  // }
  // for (int mask : bit_mask_arr1 ){
  //   printf("bitmask 1 %d \n", mask );
  // }
  // for (int mask : bit_mask_arr2 ){
  //   printf("bitmask 2 %d \n", mask );
  // }
  // for (auto it = dist_to_node_map.begin(); it != dist_to_node_map.end(); it++){
  //   if (it->second.size() > 1){
  //     printf("dist %d contains: \n", it->first);
  //       for (int node_idx : it->second){
  //         printf("  %d\n", node_idx);
  //       }
  //   }
  // }

  //initialize clusters for union find
  for (auto it = dist_to_node_map.begin(); it != dist_to_node_map.end(); it++){
    int dist = it->first;
    for (node* n_ : it->second){
      n_->parent = it->second.back()->parent;
      clusters_arr[it->second.back()->parent->idx].push_back(n_->idx);
    }
  }

  printf("done assigning parents\n");


  //MAIN LOOP
  int itr = 0;
  int num_clusters = num_nodes;
  for (auto it = dist_to_node_map.begin(); it != dist_to_node_map.end(); it++){
    int dist = it->first;

    for (int mask : bit_mask_arr_all){
      int res_dist = binXOR(dist, mask);
      if (dist_to_node_map.find(res_dist) != dist_to_node_map.end() ){
        Union(dist_to_node_map[dist], dist_to_node_map[res_dist], nodes, clusters_arr, num_clusters);
      }
    }
    printf("itr: %d \n", itr++);
    printf("num of clusters: %d \n", num_clusters);
  }

  printf("num of clusters: %d \n", num_clusters);

  return 0;
}

