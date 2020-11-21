//Huffman
#include <queue>
#include <fstream>
#include <iostream>
#include <vector>
#include <algorithm>
#include <string>



int main(){
  //file input
  std::string filepath = "mwis.txt";
  std::ifstream inFile(filepath);
  if (!inFile){
    std::cerr << "unable to open file at " << filepath << std::endl;
    return -1;
  }

  int num_v;
  long weight;

  inFile >> num_v;
  printf("Number of vertices = %d \n", num_v);

  long weights[num_v+1];
  long long A[num_v + 1];

  //extract data from file
  int idx = 1;
  while (inFile >> weight){
    weights[idx] = weight;
    idx++;
  }

  printf("Added weights \n");

  //Initialization
  A[0] = 0;
  A[1] = weights[1];

  //Populate A with Max WIS of G
  for (int i =2; i < num_v +2; i++){
    A[i] = std::max(A[i-1], A[i-2]+weights[i]);
  }


  //debug print
  // for (int i =0; i < num_v +1; i++){
  //   printf("a[%d] = %lld \n", i, A[i]);
  // }

  // printf("Max Weight Independent Set of Gi = %lld \n", A[num_v+1]);

  //reconstruction algorithm
  std::vector<int> S;
  int i = num_v + 1;
  while (i >= 1){
    if (A[i-1] >= (A[i-2] + weights[i])){ //case 1 wins
      i--;
    }
    else{ //case 2 wins
      //add vi to S, decrease i by 2
      S.push_back(i);
      i -= 2;
    }
  }

  //debug print
  int i_ = 0;
  for (int s : S){
    printf("S[%d] = %d \n", i_, S[i_]);
    i_++;
  }

  //check if desired vertices are inside S
  std::vector<int> verts{1, 2, 3, 4, 17, 117, 517, 997};
  std::vector<int> v_idx{0,1,2,3,4,5,6,7};
  int check[8] = {0};
  for (int i = 0; i < S.size(); ++i ){
    for (int idx_check : v_idx){
      if (S[i] == verts[idx_check]){
        check[idx_check] = 1;
        v_idx.erase(v_idx.begin()+idx_check);
      } 
    }
  }

  for (int c_ : check){
    std::cout << c_;
  }

  
  return 0;
}

