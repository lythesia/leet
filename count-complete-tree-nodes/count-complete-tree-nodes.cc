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

class Solution {
public:
  int countNodes(node root) {
    int left = 0, right = 0;
    for(node p=root; p; p=p->left) left++;
    for(node p=root; p; p=p->right) right++;
    if(left == right) return (1 << left) - 1;
    else return countNodes(root->left) + countNodes(root->right) + 1;
  }
};

int main(int argc, const char *argv[])
{
  
  return 0;
}
