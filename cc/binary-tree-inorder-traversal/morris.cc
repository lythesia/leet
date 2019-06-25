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
  vi inorderTraversal(node root) {
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
        if(!prev->right) prev->right = curr, curr = curr->left;
        else {
          res.push_back(curr->val);
          prev->right = NULL, curr = curr->right;
        }
      }
    }
    return res;
  }
};

#define N 7
int main(int argc, const char *argv[])
{
  int A[] = {0,1,2,3,4,5,0,7};
  node nodes[N] = { 0 };
  for(int i=1; i<N+1; i++) {
    if(A[i]) nodes[i] = new TreeNode(A[i]);
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
  vi res = so.inorderTraversal(nodes[1]);
  for(int i : res) printf("%d ", i); puts("");
  return 0;
}
