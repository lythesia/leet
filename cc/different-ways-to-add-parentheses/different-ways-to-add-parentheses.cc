#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

struct Node {
    string val;
    Node *left;
    Node *right;
    Node(const string &x) : val(x), left(NULL), right(NULL) {}
};
typedef Node *node;
typedef vector<node> vn;
map<char, int (*)(int, int)> OP = {
  {'+', [](int x, int y) {return x + y;}},
  {'-', [](int x, int y) {return x - y;}},
  {'*', [](int x, int y) {return x * y;}},
};
class Solution {
public:
  vn dfs(vs &nodes, int s, int e) {
    assert(nodes[s]!="+" && nodes[s]!="-" && nodes[s]!="*");
    if(s + 1 == e) return { new Node(nodes[s]) };
    vn ans;
    for(int i=s+1; i<e; i+=2) { // i is always op
      vn left = dfs(nodes, s, i), right = dfs(nodes, i+1, e);
      for(auto l : left) {
        for(auto r : right) {
          node root = new Node(nodes[i]);
          root->left = l, root->right = r;
          ans.push_back(root);
        }
      }
    }
    return ans;
  }

  int cal(node root) {
    if(isdigit(root->val[0])) return stoi(root->val);
    else return OP[root->val[0]](cal(root->left), cal(root->right));
  }

  vi diffWaysToCompute(string s) {
    vs nodes;
    int p = 0, len = s.length();
    while(p < len) {
      if(isblank(s[p])) while(isblank(s[p])) p++;
      else if(isdigit(s[p])) {
        string ns;
        while(isdigit(s[p])) ns.push_back(s[p++]);
        nodes.push_back(ns);
      }
      else nodes.push_back(string(1, s[p++]));
    }
    vi ans;
    vn trees = dfs(nodes, 0, (int)nodes.size());
    for(node t : trees) ans.push_back(cal(t));
    return ans;
  }
};
int main(int argc, const char *argv[])
{
  Solution so;
  string input = "2*3-4*5";
  vi ans = so.diffWaysToCompute(input);
  copy(ans.begin(), ans.end(), ostream_iterator<int>(cout, " ")); cout << endl;
  return 0;
}
