#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};
typedef TreeNode *node;
typedef vector<int> vi;
class Solution {
public:
  node buildrec(vi &pre, int ps, int pe, vi &in, int is, int ie) {
    if(pe == ps) return NULL;
    node root = new TreeNode(pre[ps]);
    if(pe - ps == 1) return root;
    int pos_in = find(in.begin()+is, in.begin()+ie, pre[ps]) - in.begin(),
        pos_pre = ps + 1 + (pos_in - is);
    root->left = buildrec(pre, ps+1, pos_pre, in, is, pos_in);
    root->right = buildrec(pre, pos_pre, pe, in, pos_in+1, ie);
    return root;
  }

  node buildTree(vi &preorder, vi &inorder) {
    return buildrec(preorder, 0, preorder.size(), inorder, 0, inorder.size());
  }
};

template <class T>
void see_vec(vector<T> vt) {
  for(auto t : vt) cout << t << " "; cout << endl;
}
void see_tree_level(node root) {
  if(!root) return;
  queue<node> Q;
  Q.push(root);
  while(!Q.empty()) {
    node top = Q.front();
    Q.pop();
    printf("%d ", top->val);
    if(top->left) Q.push(top->left);
    if(top->right) Q.push(top->right);
  }
  puts("");
}
int main(int argc, const char *argv[])
{
  vi p = {7, 1, 0, 3, 2, 5, 4, 6, 9, 8, 10}, // pre
     i = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10}; // in
     //p = {0, 2, 4, 6, 5, 3, 1, 8, 10, 9, 7}; // post
  see_vec(i);
  see_vec(p);
  puts("");
  Solution so;
  node tree = so.buildTree(p, i);
  see_tree_level(tree);
  return 0;
}

