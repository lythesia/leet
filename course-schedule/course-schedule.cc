#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

typedef pair<int,int> pii;
typedef vector<pii> vp;
typedef unordered_set<int> si;
typedef vector<si> vsi;
class Solution {
public:
  bool canFinish(int n, vp &pq) {
    // [pre] -> [post ..]
    // ..
    vsi g(n);
    for(auto &p : pq) g[p.second].insert(p.first);

    // indegree
    vi ind(n);
    for(auto &s : g) for(int i : s) ind[i]++;

    int mmax = 0;
    for(int j=0; j<n; j++) {
      for(mmax=0; mmax<n && ind[mmax]!=0; mmax++); // find a maximal node
      if(mmax == n) return false; // no maximal there's a circle
      // remove the maximal
      ind[mmax] = -1;
      for(int i : g[mmax]) ind[i]--;
    }
    return true;
  }
};

int main(int argc, const char *argv[])
{
  int n = 2;
  vp v = {
    {1,0},
    {0,1}
  };
  Solution so;
  cout << boolalpha << so.canFinish(n, v) << endl;
  return 0;
}
