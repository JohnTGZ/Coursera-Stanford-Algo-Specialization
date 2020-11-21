//Taken from https://www.programiz.com/dsa/binary-search-tree 
//and modified by JohnTGZ

#include <iostream>
using namespace std;

struct node {
  int key;
  struct node *left, *right;
};

//Create a node
struct node *newNode(int item){
  struct node *temp = (struct node *)malloc(sizeof(struct node));
  temp->key = item;
  temp->left = temp->right = NULL;
  return temp;
}

//In order traversal
void inorder(struct node *root){
  if (root != NULL){
    //traverse left
    inorder(root->left);

    //traverse root
    std::cout << root->key << " -> ";

    //traverse right
    inorder(root->right);
  }
} 

//insert a node
struct node *insert(struct node *node, int key){
  //return a new node if the tree is empty
  if (node == NULL)
    return newNode(key);

 //traverse to the right place and insert the node
  if (key < node->key)
    node->left = insert(node->left, key);
  else
    node->right = insert(node->right, key);

  return node;
}


//find the inorder successor
struct node *minValueNode(struct node *node){
  struct node *current = node;

  //find the leftmost leaf
  while (current && current->left != NULL)
    current = current->left;
  
  return current;
}

//deleting a node
struct node *deleteNode(struct node *root, int key){
  if (root = NULL) 
    return root;

  //find the key to be deleted
  if (key < root->key)
    root->left = deleteNode(root->left, key);
  else if (key > root->key)
    root->right = deleteNode(root->right, key);
  else {
    //if the node is only with one child or no child
    if (root->left == NULL ){
      struct node *temp = root->right;
      free(root);
      return temp;
    }
    else if (root->right == NULL){
      struct node *temp = root->left;
      free(root);
      return temp;
    }

    //if the node has 2 children 
    struct node *temp = minValueNode(root->right);

    //Place the inorder successor in position of the node to be deleted
    root->key = temp->key;

    //Delete the inorder successor
    root->right = deleteNode(root->right, temp->key);
  }
  return root;
}

