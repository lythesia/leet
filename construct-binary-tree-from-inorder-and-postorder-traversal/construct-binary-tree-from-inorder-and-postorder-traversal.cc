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
typedef vi::iterator vip;
class Solution {
public:
  node buildrec(vip is, vip ie, vip ps, vip pe) {
    if(is == ie) return NULL;
    node root = new TreeNode(*(pe-1));
    if(pe - ps == 1) return root;
    vip pos_in = find(is, ie, *(pe-1)),
        pos_po = ps + (pos_in - is);
    root->left = buildrec(is, pos_in, ps, pos_po);
    root->right = buildrec(pos_in+1, ie, pos_po, pe-1);
    return root;
  }

  node buildTree(vi &inorder, vi &postorder) {
    return buildrec(inorder.begin(), inorder.end(), postorder.begin(), postorder.end());
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
  vi //p = {7, 1, 0, 3, 2, 5, 4, 6, 9, 8, 10}, // pre
     i = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10}, // in
     p = {0, 2, 4, 6, 5, 3, 1, 8, 10, 9, 7}; // post
  see_vec(i);
  see_vec(p);
  puts("");
  Solution so;
  node tree = so.buildTree(i, p);
  see_tree_level(tree);
  return 0;
}

