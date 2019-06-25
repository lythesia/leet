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
  bool isValidBST(TreeNode *root) {
    return valid(root, INT_MIN, INT_MAX); // INT_MIN/MAX maybe included its not good sol, hacking with LONG_{} ? 
  }
  bool valid(node root, int mmin, int mmax) {
    if(!root) return true;
    if(root->val < mmin || root->val > mmax) return false;
    return valid(root->left, mmin, root->val-1) && valid(root->right, root->val+1, mmax);
  }
};
