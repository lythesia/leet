#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  bool check(vi &b, int r, int c) {
    for(int i=0; i<r; i++) if(b[i] == c || abs(b[i]-c) == r-i) return false;
    return true;
  }

  void dfs(int &ans, vi &b, int r, int n) {
    if(r == n) ans++;
    else {
      for(int i=0; i<n; i++) {
        b[r] = i;
        if(check(b, r, i)) dfs(ans, b, r+1, n);
      }
    }
  }
  int totalNQueens(int n) {
    int ans = 0;
    vi b(n);
    dfs(ans, b, 0, n);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  int n = 4;
  Solution so;
  int ans = so.totalNQueens(n);
  cout << ans << endl;
  return 0;
}
