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
  vi findOrder(int n, vp &pq) {
    vi ans;
    vsi g(n);
    for(auto &p : pq) g[p.second].insert(p.first);

    vi ind(n);
    for(auto &s : g) for(int i : s) ind[i]++;

    int mmax = 0, cnt = 0;
    queue<int> Q;
    for(mmax=0; mmax<n; mmax++) if(!ind[mmax]) Q.push(mmax);
    if(Q.empty()) return ans;
    while(!Q.empty()) {
      mmax = Q.front(), cnt++;
      Q.pop();
      ans.push_back(mmax);
      for(int i : g[mmax]) if(!--ind[i]) Q.push(i);
    }
    if(cnt != n) ans.clear();
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  // int n = 3;
  // vp v = {
  //   {1,0},
  //   {2,0},
  //   {0,1},
  // };
  int n = 2;
  vp v = {};
  Solution so;
  vi ans = so.findOrder(n, v);
  copy(ans.begin(), ans.end(), ostream_iterator<int>(cout, " ")); cout << endl;
  return 0;
}
