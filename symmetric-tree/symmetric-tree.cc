#include <bits/stdc++.h>
using namespace std;
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};
typedef TreeNode *node;
typedef vector<node> vp;
class Solution {
public:
  bool symsub(node p1, node p2) {
    return (!p1 || !p2) ? (p1==p2) : (p1->val==p2->val) && symsub(p1->left, p2->right) && symsub(p1->right, p2->left);
    //                             1. checks current pair  2. down oen level
  }

  bool isSymmetric(node root) {
    return root ? symsub(root->left, root->right) : true;
  }
};

const int N = 3;
int main(int argc, const char *argv[])
{
#if 0
  int A[N+1] = 
  { 0,
    5,
    4,8,
    11,0,13,4,
    7,2,0,0,0,0,5,1
  };
#endif
  int A[N+1] = {0,1,2,2};
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
  cout << so.isSymmetric(nodes[1]) << endl;
  return 0;
}
