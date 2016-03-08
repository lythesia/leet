#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

const int MIN = -0x3f3f3f3f;

class Solution {
public:
  int coinChange(vi &coins, int amount) {
    vi dp(amount + 1, MIN);
    dp[0] = 0;
    int N = coins.size();
    for(int i=0; i<N; i++) {
      for(int j=coins[i]; j<=amount; j++) {
        dp[j] = max(dp[j], dp[j - coins[i]] - 1);
      }
    }
    return dp[amount] == MIN ? -1 : -dp[amount];
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  // vi c = {1,2,5};
  // int a = 11;
  vi c = {2};
  int a = 3;
  cout << so.coinChange(c, a) << endl;
  return 0;
}
