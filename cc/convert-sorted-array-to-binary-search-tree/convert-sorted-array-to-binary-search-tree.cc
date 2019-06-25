#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
  int val;
  TreeNode *left, right;
  TreeNode(int x): val(x), left(NULL), right(NULL) {}
};

typedef TreeNode *node;
class Solution {
public:
  node sa2bst(vector<int> &num, int s, int e) {
    if(s == e) return NULL;
    int mid = (s + e) / 2;
    node left = sa2bst(num, s, mid), right = sa2bst(num, mid+1, e);
    node root = new TreeNode(num[mid]);
    root->left = left, root->right = right;
    return root;
  }
  node sortedArrayToBST(vector<int> &num) {
    return sa2bst(num, 0, num.size());
  }
};
