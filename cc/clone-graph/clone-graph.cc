#include <bits/stdc++.h>
using namespace std;

struct UndirectedGraphNode {
  int label;
  vector<UndirectedGraphNode*> neighbors;
  UndirectedGraphNode(int x) : label(x) {}
};
typedef UndirectedGraphNode Node;
typedef vector<node> vn;
class Solution {
public:
  node dfs(node root) {
    if(v[root->label]) return v[root->label];
    else {
      node ans = new UndirectedGraphNode(root->label);
      v[ans->label] = ans;
      for(node n : root->neighbors) ans->neighbors.push_back(dfs(n));
      return ans;
    }
  }

  node cloneGraph(node root) {
    return dfs(root);
  }
private:
  unordered_map<int, node> v; // avoid ref to original
};

void pgraph(node root) {
  queue<node> Q;
  unordered_map<int, bool> v;
  Q.push(root);
  while(!Q.empty()) {
    node n = Q.front();
    Q.pop();
    if(v[n->label]) continue;
    v[n->label] = true;
    printf("%i(%p) has: ", n->label, n);
    for(auto p : n->neighbors) {
      printf("%d(%p) ", p->label, p);
      Q.push(p);
    }
    puts("");
  }
}

int main(int argc, const char *argv[])
{
  // Node n0(0), n1(1), n2(2);
  // n0.neighbors.push_back(&n1);
  // n0.neighbors.push_back(&n2);
  // n1.neighbors.push_back(&n0);
  // n1.neighbors.push_back(&n2);
  // n2.neighbors.push_back(&n2);
  // pgraph(&n0);

  Node n0(0);
  n0.neighbors.push_back(&n0);
  n0.neighbors.push_back(&n0);
  pgraph(&n0);
  puts("----");

  Solution so;
  node clone = so.cloneGraph(&n0);
  pgraph(clone);
  return 0;
}
