#include <bits/stdc++.h>
using namespace std;

class Solution {
private:
  vector<int> dp;
public:
  int refn(int n, int k) {
    if(dp[k]) return dp[k];
    int ans = 0;
    for(int i=1; i<=k; i++) ans += refn(n, i-1) * refn(n, k-i);
    dp[k] = ans;
    return dp[k];
  }

  int numTrees(int n) {
    dp.resize(n+1, 0);
    dp[0] = dp[1] = 1;
    return refn(n, n);
  }
};

int main(int argc, const char *argv[])
{
  int n = 19;
  Solution so;
  cout << so.numTrees(n) << endl;
  return 0;
}
