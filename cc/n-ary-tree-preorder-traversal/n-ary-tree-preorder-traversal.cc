#include <bits/stdc++.h>
using namespace std;
struct Node {
  int val;
  vector<Node*> children;
  Node(int x) : val(x) {}
  Node(int x, vector<Node*> ch): val(x), children(ch) {}
};
class Solution {
public:
  void helper(Node* root, vector<int> &ans) {
    if(!root) return;
    ans.push_back(root->val);
    for(auto ch: root->children) {
      helper(ch, ans);
    }
  }

  vector<int> preorder(Node* root) {
    vector<int> ans;
    ans.reserve(10000);
    helper(root, ans);
    return ans;
  }
};

int main(int argc, char *argv[])
{

  return 0;
}
