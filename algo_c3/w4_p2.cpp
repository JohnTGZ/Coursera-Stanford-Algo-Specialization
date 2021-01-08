//Huffman
#include <queue>
#include <fstream>
#include <iostream>
#include <vector>
#include <algorithm>
#include <string>
#include <unordered_map>

//file input
std::string filepath = "knapsack_big.txt";
//t1: 147, t2:  210
std::ifstream inFile(filepath);

int W;
int num_items;
int* values; 
int* weights;

std::unordered_map<long, long> A;

long rec(int i, int x);

int main(){

  inFile >> W >> num_items;
  printf("num_items = %d and knapsack size = %d \n", num_items, W);

  int value, weight;
  values = new int[num_items+1];
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
  

  long current = rec(num_items, W);

  printf(" Max optimal value = %ld \n", current);

  return 0;
}


long rec(int i, int x){

  auto find_idx = A.find(i+x*num_items);

  //if not found
  if (find_idx == A.end()){
    //base case
    if (i == 0){
      A[i+x*num_items] = 0;
    }
    //current item cannot fit inside
    else if (x < weights[i]){
      A[i+x*num_items] = rec(i-1, x);
    }
    else
      //recurse on case 1 and case 2
      A[i+x*num_items] = std::max( rec(i-1, x), (rec(i-1, x- weights[i]) + values[i]) );
  }
  return A[i+x*num_items];
}