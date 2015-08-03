#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};
typedef TreeNode *node;
/* assert p q exist, right? */
class Solution {
public:
  node lowestCommonAncestor(node root, node p, node q) {
    if(!root || root == p || root == q) return root;
    node l = lowestCommonAncestor(root->left, p, q),
         r = lowestCommonAncestor(root->right, p, q);
    if(l && r) return root;
    else return l ? l : r;
  }
};
