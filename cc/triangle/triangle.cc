#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
class Solution {
public:
  int minimumTotal(vvi &tri) {
    int n = tri.size();
    vi dp = tri[n-1];
    for(int i=n-2; i>=0; i--) // row
      for(int j=0; j<=i; j++) // upper row
        dp[j] = tri[i][j] + min(dp[j], dp[j+1]);
    return dp[0];
  }
};

int main(int argc, const char *argv[])
{
  vvi tri = {
    {2},
    {3,4},
    {6,5,7},
    {4,1,8,3}
  };
  Solution so;
  cout << so.minimumTotal(tri) << endl;
  return 0;
}
