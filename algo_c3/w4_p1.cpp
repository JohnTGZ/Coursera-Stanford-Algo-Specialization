//Huffman
#include <queue>
#include <fstream>
#include <iostream>
#include <vector>
#include <algorithm>
#include <string>

int main(){
  //file input
  std::string filepath = "knapsack_test1.txt";
  //t1: 147, t2:  210
  std::ifstream inFile(filepath);
  if (!inFile){
    std::cerr << "unable to open file at " << filepath << std::endl;
    return -1;
  }

  int W;
  int num_items;
  int value, weight;

  inFile >> W >> num_items;
  printf("num_items = %d and knapsack size = %d \n", num_items, W);

  int values[num_items+1];
  int weights[num_items+1];

  //extract data from file
  int idx = 1;
  while (inFile >> value >> weight){
    values[idx] = value;
    weights[idx] = weight;
    idx++;
  }

  //debug
  // for (int i = 1; i < num_items + 1; i++){
  //   printf("idx(%d), value(%d), weight(%d) \n", i, values[i], weights[i]);
  // }

  int** A;
  A = new int*[num_items + 1];
  for (int i = 0; i < num_items+1; ++i){ //up to n
    A[i] = new int[W + 1];
  }

  //Initialize A[0,x] = 0 for x = 0,1,...,W
  for (int x = 0; x < W+1; ++x){
    A[0][x] = 0;
  }

  for (int i = 1; i < num_items + 1; ++i){ //up to num_items
    for (int x = 0; x < W+1; ++x){ //up to w
      if (weights[i] > x){
        // std::cout << "exceeded" << std::endl;
        A[i][x] = A[i-1][x];
        continue;
      }
      A[i][x] = std::max(A[i-1][x], A[i-1][x-weights[i]] + values[i]);
    }
  }

  printf(" A[num_items][W] = %d \n", A[num_items][W]);

  return 0;
}

