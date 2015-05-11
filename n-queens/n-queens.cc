#include <bits/stdc++.h>
using namespace std;

typedef vector<string> vs;
typedef vector<vs> vvs;
typedef vector<int> vi;
class Solution {
public:
  bool check(vi &b, int r, int c) {
    for(int i=0; i<r; i++) if(b[i] == c || abs(b[i]-c) == r-i) return false;
    return true;
  }

  void dfs(vvs &ans, vi &b, int r, int n) {
    if(r == n) {
      vs sol(n, string(n, '.'));
      for(int i=0; i<n; i++) sol[i][b[i]] = 'Q';
      ans.push_back(sol);
    }
    else {
      for(int i=0; i<n; i++) {
        b[r] = i;
        if(check(b, r, i)) dfs(ans, b, r+1, n);
      }
    }
  }
  vvs solveNQueens(int n) {
    vvs ans;
    vi b(n);
    dfs(ans, b, 0, n);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  int n = 4;
  Solution so;
  vvs ans = so.solveNQueens(n);
  for(auto &v : ans) {
    cout << "----\n";
    copy(v.begin(), v.end(), ostream_iterator<string>(cout, "\n"));
    cout << "----\n\n";
  }
  return 0;
}
