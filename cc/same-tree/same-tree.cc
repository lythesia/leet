#include <cstdio>
#include <iostream>
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
  bool isSameTree(node p, node q) {
    return (!p || !q) ? (p==q) : (p->val==q->val) && isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
  }
};
