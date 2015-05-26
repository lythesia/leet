#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  void dfs(vvi &ans, vi &seg, int s, int k, int n) {
    if(!n) ans.push_back(seg);
    else if(!k) return;
    else {
      // i i+1 .. i+k-1: k items least sum <= n
      for(int i=s; i<10 && (i+i+k-1)*k/2<=n; i++) {
        seg.push_back(i);
        dfs(ans, seg, i+1, k-1, n-i);
        seg.pop_back();
      }
    }
  }

  vvi combinationSum3(int k, int n) {
    vvi ans;
    vi seg;
    dfs(ans, seg, 1, k, n);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  vvi ans = so.combinationSum3(3, 9);
  for(vi &v : ans) {
    cout << "[ ";
    copy(v.begin(), v.end(), ostream_iterator<int>(cout," "));
    cout << "]\n";
  }
  return 0;
}
