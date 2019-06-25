#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
  int val;
  TreeNode *left, *right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};
typedef vector<int> vi;
typedef TreeNode *node;
class Solution {
public:
  vi rightSideView(node root) {
    vi ans;
    if(!root) return ans;
    queue<node> Q[2];
    Q[0].push(root);
    int curq = 0;
    while(!Q[curq].empty()) {
      node p = NULL;
      while(!Q[curq].empty()) {
        p = Q[curq].front();
        if(p->left) Q[1-curq].push(p->left);
        if(p->right) Q[1-curq].push(p->right);
        Q[curq].pop();
      }
      if(p) ans.push_back(p->val);
      curq = 1 - curq;
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  TreeNode root(1), n2(2), n3(3), n4(4), n5(5);
  // root.left = &n2, root.right = &n3;
  // n2.right = &n5, n3.right = &n4;

  root.left = &n2, root.right = &n3;
  n3.left = &n4;

  Solution so;
  vi ans = so.rightSideView(&root);
  copy(ans.begin(), ans.end(), ostream_iterator<int>(cout, " ")); cout << endl;
  return 0;
}
