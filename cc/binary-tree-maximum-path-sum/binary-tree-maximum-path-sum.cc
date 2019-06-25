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
  int maxPathSum(TreeNode *root) {
    int res = INT_MIN;
    maxsub(root, res);
    return res;
  }

  int maxsub(node root, int &maxval) {
    if(!root) return 0;
    int left = max(0, maxsub(root->left, maxval)), right = max(0, maxsub(root->right, maxval)); // since there maybe negative
    maxval = max(maxval, left + right + root->val);
    return max(left, right) + root->val;
  }
};
