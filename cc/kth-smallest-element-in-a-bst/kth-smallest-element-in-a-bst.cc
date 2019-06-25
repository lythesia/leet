#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

struct TreeNode {
  int val;
  TreeNode *left, *right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};
typedef TreeNode *node;
/*
 * just iterate over bst in inorder
 */
class Solution {
public:
  int kthSmallest(node root, int k) {
    stack<node> st;
    for(node p=root; p; p=p->left) st.push(p);
    int ans = st.top()->val;
    while(k--) {
      node p = st.top(); st.pop();
      ans = p->val, p = p->right;
      for(; p; p=p->left) st.push(p);
    }
    return ans;
  }
};
int main(int argc, const char *argv[])
{
  
  return 0;
}
