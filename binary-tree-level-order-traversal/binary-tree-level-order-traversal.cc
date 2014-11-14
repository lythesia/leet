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
class Solution {
public:
  vvi levelOrder(node root) {
    vvi res;
    if(!root) return res;
    queue<node> Q[2];
    int sw = 0;
    Q[sw].push(root);
    vi level;
    while(!Q[0].empty() || !Q[1].empty()) {
      assert(!Q[sw].empty());
      node top = Q[sw].front();
      Q[sw].pop();
      level.push_back(top->val);
      if(top->left) Q[1-sw].push(top->left);
      if(top->right) Q[1-sw].push(top->right);
      if(Q[sw].empty()) {
        sw = 1-sw;
        res.push_back(level);
        level.clear();
      }
    }
    return res;
  }
};

const int N = 7;
int main(int argc, const char *argv[])
{
  int A[N+1] = 
  { 0,
    3,
    9,20,
    0,0,15,7,
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
  auto res = so.levelOrder(nodes[1]);
  cout << res.size() << endl;
  for(auto ii : res) {
    for(auto i : ii) printf("%d ", i);
    puts("");
  }
  return 0;
}
