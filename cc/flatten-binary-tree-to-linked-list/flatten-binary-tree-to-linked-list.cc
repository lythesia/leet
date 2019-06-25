#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};
typedef TreeNode *node;
class Solution {
public:
  void postorder(node root, node &prev) {
    if(!root) return;
    postorder(root->right, prev);
    postorder(root->left, prev);
    root->right = prev;
    root->left = NULL;
    prev = root;
  }

  /* non-recursive ver */
  /*
  void flatten(node root) {
    while ( root ) {
      if ( root->left ) {
        TreeNode *ptr = root->left;
        while ( ptr->right ) ptr = ptr->right; // here visited nodes will be visited again in (1)
        ptr->right = root->right; // right sub-tree move to as the right child of current root's left-most node
        root->right = root->left; // switch left tree of current root to right
        root->left = NULL;        // empty left
      }
      root = root->right;         // down a level to right (1)
    }
  }
  */

  void flatten(node root) {
    if(!root) return;
    node prev = NULL;
    postorder(root, prev);
  }
};

const int N = 7;
int main(int argc, const char *argv[])
{
  int A[N+1] = 
  { 0,
    1,
    2,5,
    3,4,0,6,
  };
  node nodes[N+1] = { 0 };
  for(int i=1; i<N+1; i++) {
    if(A[i]) nodes[i] = new TreeNode(A[i]);
  }
  for(int i=1; i<N+1; i++) {
    if(2*i < N+1 && nodes[i]) nodes[i]->left = nodes[2*i];
    if(2*i+1 < N+1 && nodes[i]) nodes[i]->right = nodes[2*i+1];
  }
  puts("level order");
  int lv = 0;
  while((1<<lv) <= N) ++lv;
  for(int i=0; i<lv; i++) {
    for(int j=0; j<(1<<i) && (1<<i)+j < N+1; j++) 
      if(nodes[(1<<i)+j]) printf("%d ", nodes[(1<<i)+j]->val);
      else printf("# ");
    puts("");
  }
  puts("--------");

  Solution so;
  so.flatten(nodes[1]);
  node root = nodes[1];
  while(root) {
    assert(!root->left);
    printf("%d ", root->val);
    root = root->right;
  }
  puts("");
  return 0;
}
