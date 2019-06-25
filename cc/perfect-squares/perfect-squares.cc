#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int numSquares(int n) {
    vi dp(n + 1, INT_MAX);
    dp[0] = 0, dp[1] = 1;
    for(int i=0; i<=n; i++) for(int j=1; i+j*j<=n; j++) dp[i+j*j] = min(dp[i+j*j], dp[i] + 1);
    return dp[n];
  }

  // https://en.wikipedia.org/wiki/Lagrange%27s_four-square_theorem
  // https://en.wikipedia.org/wiki/Legendre%27s_three-square_theorem
  int ns(int n) {
    // n is sum of three square iff not n = 4^a * (8b + 7)
    while(!(n % 4)) n /= 4;
    if(n % 8 == 7) return 4; // it's not three square, so it's only to be 4
    for(int i=0; i*i<=n; i++) {
      int j = sqrt(n - i*i);
      if(i*i + j*j == n) return i ? 2 : 1;  // if it's two square, 0 for special(note it will n/=4 not affect, since it's just a constant multiplier)
    }
    return 3; // now it only to be 3
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << so.numSquares(12) << endl;
  cout << so.numSquares(13) << endl;
  return 0;
}
