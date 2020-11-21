//Huffman
#include <queue>
#include <fstream>
#include <iostream>
#include <vector>
#include <algorithm>
#include <string>

class node{
  public: 
    // node (): node("-1", 0) {}

    node(std::string idx_, int weight_, node *left_ = nullptr, node *right_ = nullptr){
      this->idx = idx_;
      this->weight = weight_;
      this->left = left_;
      this->right = right_;
    }


  public: 
    std::string idx;
    int weight;
    node *left;
    node *right;

};


class compNodes {
  public: 
    //in decreasing order
    bool operator() (const node* lhs, const node* rhs) const{
      return lhs->weight > rhs->weight; 
    }
};

typedef std::priority_queue<node*, std::vector<node*>, compNodes> node_pq;

void Huffman(node_pq& nodes);
void print_pq(node_pq nodes);
int getMinDepth(node* n_);
int getMaxDepth(node* n_);

int main(){
  //file input
  std::string filepath = "huffman.txt";
  std::ifstream inFile(filepath);
  if (!inFile){
    std::cerr << "unable to open file at " << filepath << std::endl;
    return -1;
  }

  int num_sym;
  int weight;

  inFile >> num_sym;

  //PQ of weights
  node_pq nodes;

  //extract data from file
  int idx = 1;
  while (inFile >> weight){
    std::string idx_str = std::to_string(idx);
    nodes.push( new node(idx_str, weight, nullptr, nullptr));
    idx++;
  }
  //debug
  print_pq(nodes);

  Huffman(nodes);

  printf("min depth: %d, max depth: %d \n", getMinDepth(nodes.top()), getMaxDepth(nodes.top()));

  return 0;
}

void Huffman(node_pq& nodes){
  while (nodes.size() > 1){
    node* a = nodes.top();
    nodes.pop();
    node* b = nodes.top();
    nodes.pop();
    // printf("a.w(%d), b.w(%d) \n", a.weight, b.weight);
    nodes.push(new node(a->idx + " " + b->idx, a->weight+b->weight, a, b));
  }

  printf("completed huffman \n");
}

void print_pq(node_pq nodes){
  while(!nodes.empty()){
    node* n_ = nodes.top();
    nodes.pop();
    std::cout << "idx: " << n_->idx << " weight: " << n_->weight << std::endl;
  }
}


//get the depth of the tree for the minimum and maximum elements
int getMinDepth(node* n_){
  int depth_l, depth_r;
  if (n_->left)
    depth_l = 1 + getMinDepth(n_->left);
  else
    depth_l = 0;
  if (n_->right)
    depth_r = 1 + getMinDepth(n_->right);
  else
    depth_r = 0;
  
  return std::min(depth_l, depth_r);  
}

//get the depth of the tree for the minimum and maximum elements
int getMaxDepth(node* n_){
  int depth_l, depth_r;
  if (n_->left)
    depth_l = 1 + getMaxDepth(n_->left);
  else
    depth_l = 0;
  if (n_->right)
    depth_r = 1 + getMaxDepth(n_->right);
  else
    depth_r = 0;
  
  return std::max(depth_l, depth_r);  
}