#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int nthUglyNumber(int n) {
    vi dp(n + 1, 1);
    int a = 1, b = 1, c = 1;
    for(int i=2; i<=n; i++) {
      dp[i] = min({ dp[a]*2, dp[b]*3, dp[c]*5 });
      if(dp[i] == dp[a]*2) a++;
      if(dp[i] == dp[b]*3) b++;
      if(dp[i] == dp[c]*5) c++;
    }
    return dp[n];
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << so.nthUglyNumber(10) << endl;
  return 0;
}
