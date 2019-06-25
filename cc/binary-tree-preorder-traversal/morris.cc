#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};
typedef TreeNode *node;
typedef vector<int> vi;
class Solution {
public:
  vi preorderTraversal(node root) {
    if(!root) return vi();
    vi res;
    node curr = root;
    while(curr) {
      if(!curr->left) {
        res.push_back(curr->val);
        curr = curr->right;
      }
      else {
        node prev = curr->left;
        while(prev->right && prev->right != curr) prev = prev->right;
        if(!prev->right) {
          res.push_back(curr->val);
          prev->right = curr, curr = curr->left;
        }
        else prev->right = NULL, curr = curr->right;
      }
    }
    return res;
  }
};

// #define N 7
#define N 5
int main(int argc, const char *argv[])
{
  // int A[] = {0,1,2,3,4,5,0,7};
  int A[] = {0,3,1,0,0,2};
  node nodes[N] = { 0 };
  for(int i=1; i<N+1; i++) {
    if(A[i]) nodes[i] = A[i] ? new TreeNode(A[i]) : NULL;
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
  vi res = so.preorderTraversal(nodes[1]);
  for(int i : res) printf("%d ", i); puts("");
  return 0;
}
