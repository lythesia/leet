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
typedef vector<vi> vvi;
typedef vector<node> vp;
class Solution {
public:
  vvi zigzagLevelOrder(node root) {
    vvi res;
    if(!root) return res;
    vp Q[2];
    vi level;
    int sw = 0;
    Q[sw].push_back(root);
    while(!Q[0].empty() || !Q[1].empty()) {
      assert(!Q[sw].empty());
      node top = Q[sw].back();
      Q[sw].pop_back();
      level.push_back(top->val);
      // first visited always puts its children at old place
      if(sw) {
        if(top->right) Q[1-sw].push_back(top->right);
        if(top->left) Q[1-sw].push_back(top->left);
      }
      else {
        if(top->left) Q[1-sw].push_back(top->left);
        if(top->right) Q[1-sw].push_back(top->right);
      }
      if(Q[sw].empty()) {
        sw = 1-sw;
        res.push_back(level);
        level.clear();
      }
    }
    return res;
  }
};

const int N = 15;
int main(int argc, const char *argv[])
{
  int A[N+1] = 
  { 0,
    5,
    4,8,
    11,0,13,4,
    7,2,0,0,0,0,5,1
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
  vvi res = so.zigzagLevelOrder(nodes[1]);
  cout << res.size() << endl;
  for(auto ii : res) {
    for(auto i : ii) cout << i << " "; cout << endl;
  }
  return 0;
}
