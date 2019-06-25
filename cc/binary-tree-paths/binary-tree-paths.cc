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
  vs binaryTreePaths(node root) {
    vs ans;
    if(!root) return ans;
    vs left = binaryTreePaths(root->left), right = binaryTreePaths(root->right);
    if(left.empty() && right.empty()) ans.push_back(to_string(root->val));
    else {
      for(auto l : left) ans.push_back(to_string(root->val) + "->" + l);
      for(auto r : right) ans.push_back(to_string(root->val) + "->" + r);
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  TreeNode t[4] = {TreeNode(1), TreeNode(2), TreeNode(3), TreeNode(5)};
  t[0].left = &t[1], t[0].right = &t[2], t[1].right = &t[3];
  Solution so;
  vs ans = so.binaryTreePaths(&t[0]);
  copy(ans.begin(), ans.end(), ostream_iterator<string>(cout, " ")); cout << endl;
  return 0;
}
