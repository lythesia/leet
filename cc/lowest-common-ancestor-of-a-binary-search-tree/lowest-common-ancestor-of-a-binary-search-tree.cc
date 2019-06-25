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
class Solution {
public:
  node lowestCommonAncestor(node root, node p, node q) {
    if(p->val > q->val) swap(p, q);
    if(root->val >= p->val && root->val <= q->val) return root;
    else if(root->val > p->val && root->val > q->val) return lowestCommonAncestor(root->left, p, q);
    else return lowestCommonAncestor(root->right, p, q);
  }
};

int main(int argc, const char *argv[])
{
  TreeNode node[2] = {TreeNode(2), TreeNode(1)};
  node[0].left = &node[1];
  Solution so;
  cout << so.lowestCommonAncestor(&node[0], &node[0], &node[1])->val << endl;
  return 0;
}
