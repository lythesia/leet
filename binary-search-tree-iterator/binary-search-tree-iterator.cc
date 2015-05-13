#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};
typedef TreeNode *node;
class BSTIterator {
public:
  BSTIterator(node root) : p(root) {}

  bool hasNext() {
    return !s.empty() || p != NULL;
  }

  // average O(1)
  int next() {
    while(p) {
      s.push(p);
      p = p->left;
    }
    node t = s.top();
    p = t->right;
    s.pop();
    return t->val;
  }

private:
  node p;
  stack<node> s;
};
