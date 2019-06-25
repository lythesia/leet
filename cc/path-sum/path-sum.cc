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
  /*
  // elegant version
  bool hasPathSum(node root, int sum) {
    if(!root) return false;
    if(!root->left && !root->right && sum==root->val) return true;
    return hasPathSum(root->left, sum-root->val) || hasPathSum(root->right,sum-root->val);
  }
  */

  bool dfs(node root, int sum, int target) {
    sum += root->val;
    if(!root->left && !root->right) return sum == target;
    else if(root->left && !root->right) return dfs(root->left, sum, target);
    else if(!root->left && root->right) return dfs(root->right, sum, target);
    else return (dfs(root->left, sum, target) || dfs(root->right, sum, target));
  }
  bool hasPathSum(node root, int sum) {
    if(!root) return false;
    return dfs(root, 0, sum);
  }
};

//const int N = ((1<<5)-1);
const int N = 1;
int main(int argc, const char *argv[])
{
  int A[N+1] = {0};
  A[1] = 1;
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
  bool res = so.hasPathSum(nodes[1], 1);
  printf("%d", res); puts("");
  return 0;
    //if(!root) return sum == target;
}
