#include <bits/stdc++.h>
using namespace std;
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};
typedef TreeNode *node;
class Solution {
public:
  void dfs(node root, int &dep) {
    if(!root) return;
    if(!root->left && !root->right) {
      dep = max(dep, root->val);
      return;
    }
    if(root->left) {
      root->left->val = root->val + 1;
      dfs(root->left, dep);
    }
    if(root->right) {
      root->right->val = root->val + 1;
      dfs(root->right, dep);
    }
  }

  /*
  int maxDepth(node root) {
    if(!root) return 0;
    root->val = 1;
    int res = 1;
    dfs(root, res);
    return res;
  }
  */

  // elegent
  int maxDepth(node root) {
    return root ? 1+max(maxDepth(root->left), maxDepth(root->right)):0;
  }
};

//const int N = ((1<<5)-1);
const int N = 15;
int main(int argc, const char *argv[])
{
  int A[N+1] = 
  { 0,
    5,
    4,8,
    11,0,13,4,
    7,2,0,0,0,0,5,1
  };
  //A[1] = 1;
  //A[2] = 2;
  //A[4] = 3;
  //A[8] = 4;
  //A[16] = 5;
  node nodes[N+1] = { 0 };
  for(int i=1; i<N+1; i++) {
    if(A[i]) nodes[i] = new TreeNode(A[i]);
  }
  for(int i=1; i<N+1; i++) {
    if(2*i < N+1 && nodes[i]) nodes[i]->left = nodes[2*i];
    if(2*i+1 < N+1 && nodes[i]) nodes[i]->right = nodes[2*i+1];
  }
  puts("level order");
  int lv = 0;
  while((1<<lv) <= N) ++lv;
  for(int i=0; i<lv; i++) {
    for(int j=0; j<(1<<i) && (1<<i)+j < N+1; j++) 
      if(nodes[(1<<i)+j]) printf("%d ", nodes[(1<<i)+j]->val);
      else printf("# ");
    puts("");
  }
  puts("--------");

  Solution so;
  cout << so.maxDepth(nodes[1]) << endl;
  return 0;
}
