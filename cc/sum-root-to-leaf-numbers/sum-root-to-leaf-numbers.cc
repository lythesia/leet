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
  inline int to_i(int suffix, int d) {
    if(!d) return suffix;
    string s(1, (char)(d + '0'));
    s += to_string(suffix);
    return stoi(s);
  }

  /* // different meaning of sum
  int sumNumbers(node root) {
    if(!root->left && !root->right) return root->val;
    int left = 0, right = 0;
    if(root->left) left = to_i(sumNumbers(root->left), root->val);
    if(root->right) right = to_i(sumNumbers(root->right), root->val);
    return left+right;
  }
  */

  int sumNumbers(node root) {
    int res = 0;
    strng s;
    dfs(root, s, res);
    return res;
  }

  void dfs(node root, string s, int &sum) {
    if(!root) return;
    s.push_back((char)(root->val + '0'));
    if(!root->left && !root->right) {
      sum += stoi(s);
      return;
    }
    dfs(root->left, s, sum);
    dfs(root->right, s, sum);
  }
};

#define N 9
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
  //int res = so.sumNumbers(nodes[1]);
  int res = 0;
  string s;
  so.dfs(nodes[1], s, res);
  printf("%d", res); puts("");
  return 0;
}
