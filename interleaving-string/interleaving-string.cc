#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  /*
   * dp(i,j): a(0..i) b(0..j) interleaves s(0..i+j)
   */
  bool isInterleave(string s1, string s2, string s3) {
    size_t m = s1.length(), n = s2.length();
    if(m + n != s3.length()) return false;
    vector<vector<bool>> dp(m+1, vector<bool>(n+1, false));
    dp[0][0] = true;
    for(size_t i=1; i<=n; i++) dp[0][i] = s2[i-1] == s3[i-1] && dp[0][i-1];
    for(size_t i=1; i<=m; i++) dp[i][0] = s1[i-1] == s3[i-1] && dp[i-1][0];
    for(size_t i=1; i<=m; i++) {
      for(size_t j=1; j<=n; j++) {
        if(dp[i-1][j]) dp[i][j] = s1[i-1] == s3[i+j-1];
        else if(dp[i][j-1]) dp[i][j] = s2[j-1] == s3[i+j-1];
      }
    }
    return dp[m][n];
  }
};

int main(int argc, const char *argv[])
{
  string s1 = "aabcc",
         s2 = "dbbca",
         // s3 = "aadbbcbcac";
         s3 = "aadbbbaccc";
  Solution so;
  cout << boolalpha << so.isInterleave(s1, s2, s3) << endl;
  return 0;
}
