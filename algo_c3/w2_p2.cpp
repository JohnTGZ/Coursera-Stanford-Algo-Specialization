//clustering
#include <fstream>
#include <iostream>
#include <vector>
#include <algorithm>
#include <map>

int compare(){

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
  std::string line;
  std::map<int, std::string> hamming_map; //ordered map
  
  //extract data from file
  inFile >> num_nodes >> num_bits;
  printf("num_nodes(%d), num_bits(%d)\n", num_nodes, num_bits);
  int node_idx= 1;
  std::getline(inFile, line, '\n'); //skip first line
  while (std::getline(inFile, line, '\n')){
    hamming_map[node_idx] = line;
    node_idx++;
  }

  //debug print
  // for (auto i = hamming_map.begin(); i != hamming_map.end(); i++){
  //     printf("hamming[%d] = %s \n", i->first, i->second.c_str());
  // }

  //cluster nodes with less than 2 bits different
  
  //nodes with more than 2 bits different are split

  //MAIN LOOP

  //arrange all the hamming distances in increasing order of binary 
  //i.e. for 3 bits: 000 001 010 011 100 ... 101 111

  //24 choose 2 through all the binaries to find  O(24 * n) running time
  //1. keep 2 arrays (A1 keeps track of the same elements, A2 keep tracks of different elements)
  //2. any difference exceeding 2 is kicked out 

  //24 choose 1
  //24 choose 2


  return 0;
}

