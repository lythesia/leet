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
typedef vector<vi> vvi;
class Solution {
public:
  void dfs(node root, int sum, vi path, vvi &res) {
    if(!root) return;
    path.push_back(root->val);
    if(!root->left && !root->right && sum==root->val) {
      res.push_back(path);
      return;
    }
    dfs(root->left, sum-root->val, path, res);
    dfs(root->right, sum-root->val, path, res);
  }

  vvi pathSum(node root, int sum) {
    vvi res;
    dfs(root, sum, vi(), res);
    return res;
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
  vvi res = so.pathSum(nodes[1], 22);
  for(auto ii : res) {
    for(auto i : ii) cout << i << " "; cout << endl;
  }
  return 0;
}
