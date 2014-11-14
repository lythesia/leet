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
  vi postorderTraversal(node root) {
    if(!root) return vi();
    vi res;
    if(root->left) {
      vi l = postorderTraversal(root->left);
      res.insert(res.end(), l.begin(), l.end());
    }
    if(root->right) {
      vi r = postorderTraversal(root->right);
      res.insert(res.end(), r.begin(), r.end());
    }
    res.push_back(root->val);
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
  vi res = so.postorderTraversal(nodes[1]);
  for(int i : res) printf("%d ", i); puts("");
  return 0;
}
