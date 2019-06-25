#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;
typedef pair<int,int> pii;
typedef unordered_set<int> si;
typedef vector<si> vsi;

class Solution {
public:
  vi findMinHeightTrees(int n, vector<pii> &e) {
    vi ind(n);
    vsi g(n, si());
    si v;
    for(auto &p : e) ind[p.first]++, ind[p.second]++, g[p.first].insert(p.second), g[p.second].insert(p.first);
    for(int i=0; i<n; i++) v.insert(i);
    // ind = 1
    queue<int> Q;
    for(size_t i=0; i<ind.size(); i++) if(ind[i] == 1) Q.push(i);
    vi ans;
    while(v.size() > 2) {
      for(int i=0, k=Q.size(); i<k; i++) {
        int t = Q.front(); Q.pop();
        int nb = *(g[t].begin());
        if(--ind[nb] == 1) Q.push(nb);
        g[nb].erase(t), v.erase(t);
      }
    }
    for(int i : v) ans.push_back(i);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  // vector<pii> e = {};
  // vector<pii> e = {{1,0}, {1,2}, {1,3}};
  // vector<pii> e = {{0, 3}, {1, 3}, {2, 3}, {4, 3}, {5, 4}};
  // vector<pii> e = {{1,0}, {0,2}};
  vector<pii> e = {{0,1},{0,2},{0,3},{3,4},{4,5}};
  vi ans = so.findMinHeightTrees(e.size()+1, e);
  copy(ans.begin(), ans.end(), ostream_iterator<int>(cout, " ")); cout << endl;
  return 0;
}
