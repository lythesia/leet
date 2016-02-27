#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int nthSuperUglyNumber(int n, vi &primes) {
    assert(primes[0] != 1);
    int k = primes.size();
    vi dp(n + 1, INT_MAX), c(k, 1);
    dp[1] = 1;
    for(int i=2; i<=n; i++) {
      for(int j=0; j<k; j++) dp[i] = min(dp[i], dp[c[j]]*primes[j]);
      for(int j=0; j<k; j++) if(dp[i] == dp[c[j]]*primes[j]) c[j]++;
    }
    return dp[n];
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  vi p = {2,7,13,19};
  cout << so.nthSuperUglyNumber(12, p) << endl;
  return 0;
}
