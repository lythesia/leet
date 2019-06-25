#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
class Solution {
public:
  int calculateMinimumHP(vvi &dn) {
    int m = dn.size(), n = dn[0].size();
    vvi dp(m, vi(n, INT_MAX));
    dp[m-1][n-1] = dn[m-1][n-1] > 0 ? 1 : 1 - dn[m-1][n-1];
    // last row
    for(int i=n-2; i>=0; i--) dp[m-1][i] = 1+dn[m-1][i] >= dp[m-1][i+1] ? 1 : dp[m-1][i+1]-dn[m-1][i];
    // last col
    for(int i=m-2; i>=0; i--) dp[i][n-1] = 1+dn[i][n-1] >= dp[i+1][n-1] ? 1 : dp[i+1][n-1]-dn[i][n-1];
    for(int i=m-2; i>=0; i--) for(int j=n-2; j>=0; j--) 
        dp[i][j] = min(max(dp[i+1][j]-dn[i][j], 1), max(dp[i][j+1]-dn[i][j], 1));
    return dp[0][0];
  }
};

int main(int argc, const char *argv[])
{
  vvi dn = {
    {-2, -3, 3},
    {-5, -10, 1},
    {10, 30, -5}
  };
  Solution so;
  cout << so.calculateMinimumHP(dn) << endl;
  return 0;
}
