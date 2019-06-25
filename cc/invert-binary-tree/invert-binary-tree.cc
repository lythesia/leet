#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

// I think it's a joke
typedef TreeNode *node;
class Solution {
public:
  node invertTree(node root) {
    if(root) {
      node l = root->left, r = root->right;
      root->left = invertTree(r), root->right = invertTree(l);
    }
    return root;
  }
};
