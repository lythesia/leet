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
  bool isBalanced(node root) {
    if(!root) return true;
    bool left = isBalanced(root->left), right = isBalanced(root->right);
    int left_dep = root->left ? root->left->val : 0,
        right_dep = root->right ? root->right->val : 0;
    root->val = max(left_dep, right_dep) + 1;
    return left && right && abs(left_dep-right_dep)<2;
  }
};

const int N = 7;
int main(int argc, const char *argv[])
{
  int A[N+1] = 
  { 0,
    1,
    0,2,
    0,0,0,3,
  };
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
  cout << so.isBalanced(nodes[1]) << endl;
  return 0;
}
