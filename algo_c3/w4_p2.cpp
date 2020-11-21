//Huffman
#include <queue>
#include <fstream>
#include <iostream>
#include <vector>
#include <algorithm>
#include <string>

long rec(int i, int x, int* weights, int* values, int** A);

int main(){
  //file input
  std::string filepath = "knapsack_big.txt";
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

  int* values;
  values = new int[num_items+1];
  int* weights;
  weights = new int[num_items+1];

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
  A = new int*[num_items+1];
  for (int i=0; i < num_items+1; ++i){
    A[i] = new int*[W+1];
  }

  int i = num_items+1, x = W+1;
  long current = rec(i, x, weights, values, A);

  printf(" Max optimal value = %ld \n", current);

  return 0;
}


long rec(int i, int x, int* weights, int* values, int** A){
  //base case
  if (i == 0){
    A[i][x] = 0;
  }
  if (x - weights[i] < 0){
    A[i][x] = rec(i-1, x, weights, values, A);
  }

  //recurse on case 1 and case 2
  A[i][x] = std::max( rec(i-1, x, weights, values, A), (rec(i-1, x- weights[i], weights, values, A) + values[i]) );

  return A[i][x]
}