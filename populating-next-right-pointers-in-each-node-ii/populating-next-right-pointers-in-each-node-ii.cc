#include <cstdio>
#include <iostream>
#include <cmath>
#include <queue>
using namespace std;
struct TreeLinkNode {
 int val;
 TreeLinkNode *left, *right, *next;
 TreeLinkNode(int x) : val(x), left(NULL), right(NULL), next(NULL) {}
};
typedef TreeLinkNode *node;

class Solution {
public:
  void connect(TreeLinkNode *root) {
    node prev = NULL, next_level_root = NULL;
    while(root) {
      if(!next_level_root) {
        if(root->left) next_level_root = root->left;
        else if(root->right) next_level_root = root->right;
      }
      if(root->left) {
        if(prev) prev->next = root->left;
        prev = root->left;
      }
      if(root->right) {
        if(prev) prev->next = root->right;
        prev = root->right;
      }
      if(root->next) root = root->next;
      else root = next_level_root, next_level_root = NULL, prev = NULL;
    }
  }
};

#define N 7
int main(int argc, const char *argv[])
{
  int A[] = {0,1,2,3,4,5,0,7};
  node nodes[N] = { 0 };
  for(int i=1; i<N+1; i++) {
    if(A[i]) nodes[i] = new TreeLinkNode(A[i]);
  }
  for(int i=1; i<N+1; i++) {
    if(2*i < N+1) nodes[i]->left = nodes[2*i];
    if(2*i+1 < N+1) nodes[i]->right = nodes[2*i+1];
  }
  puts("level order");
  int lv = 0;
  while((1<<lv) <= N) ++lv;
  for(int i=0; i<lv; i++) {
    for(int j=0; j<(1<<i); j++) 
      if(nodes[(1<<i)+j]) printf("%d ", nodes[(1<<i)+j]->val);
      else printf("# ");
    puts("");
  }
  puts("--------");

  Solution so;
  so.connect(nodes[1]);
  for(int i=0;i<lv; i++) {
    node first = nodes[1<<i];
    while(first) { 
      printf("%d -> ", first->val);
      first = first->next; 
    }
    puts("NULL");
  }
  return 0;
}
