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
   * inorder traversal, then swapped case must be:
   * . . . . x a1 a2 .. ak y . . .
   * where a1 < a2 < .. ak, y < a1 < ak and x > ak > a1
   */
  void searchswap(node root, node &prev, node &x, node &y) {
    if(!root) return;
    searchswap(root->left, prev, x, y);
    if(root->val < prev->val) {
      if(!x) x = prev; // catch x
      y = root;        // update y
    }
    prev = root;
    searchswap(root->right, prev, x, y);
  }

  void recoverTree(node root) {
    TreeNode prev(INT_MIN);
    node x = NULL, y = NULL, p = &prev;
    searchswap(root, p, x, y);
    if(x && y) swap(x->val, y->val);
  }
};

int main(int argc, const char *argv[])
{
  
  return 0;
}
