#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};
typedef TreeNode *node;
typedef vector<node> vn;
typedef vector<int> vi;
class Solution {
public:
  vn gentree(int *nodes, int s, int e) {
    if(s >= e) return { NULL };
    vn ans;
    for(int i=s; i<e; i++) {
      vn lefts = gentree(nodes, s, i), rights = gentree(nodes, i+1, e);
      for(auto l : lefts) {
        for(auto r : rights) {
          node root = new TreeNode(nodes[i]);
          root->left = l, root->right = r;
          ans.push_back(root);
        }
      }
    }
    return ans;
  }

  vn generateTrees(int n) {
    int *arr = new int[n];
    for(int i=0; i<n; i++) arr[i] = i + 1;
    return gentree(arr, 0, n);
  }
};

void ptree(node root) {
  assert(root);
  queue<node> Q;
  Q.push(root);
  printf(" {");
  while(!Q.empty()) {
    auto p = Q.front();
    if(p) {
      printf(" %d", p->val);
      Q.push(p->left);
      Q.push(p->right);
    }
    else {
      printf(" #");
    }
    Q.pop();
  }
  printf(" }");
}

int main(int argc, const char *argv[])
{
  // vi nodes = { 1, 2, 3 };
  Solution so;
  auto ps = so.generateTrees(3);
  for(auto p : ps) ptree(p);
  return 0;
}
