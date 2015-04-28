#include <bits/stdc++.h>
using namespace std;

typedef vector<bool> vb;
typedef vector<vb> vvb;
typedef vector<vvb> vvvb;
class Solution {
public:
  /*
   * dp(i, j, l): s1(i..i+l) -> s2(j..j+l)
   * dp(i, j, l) = p1: dp(i, j, k), p2: dp(i, j, l-k) of s1 if match p1: dp(i, j+l-k, k), p2: dp(i+k, j, l-k) for any k inter 1..(l-1)
   * the final state is dp(0, 0, n), so direction: i --> 0, j --> 0, then search any valid l with inter k's.
   */
  bool isScramble(string s1, string s2) {
    int n = s1.length();
    if(s1.length() != s2.length()) return false;
    string t1 = s1, t2 = s2;
    sort(t1.begin(), t1.end()); sort(t2.begin(), t2.end());
    if(t1 != t2) return false;
    vvvb dp(n, vvb(n, vb(n+1, false)));
    for(int i=n-1; i>=0; i--) {
      for(int j=n-1; j>=0; j--) {
        dp[i][j][1] = (s1[i] == s2[j]);
        for(int l=2; i+l<=n && j+l<=n; l++) {
          for(int k=1; k<l; k++) {
            dp[i][j][l] = dp[i][j][l] || (dp[i][j][k] && dp[i+k][j+k][l-k]);
            dp[i][j][l] = dp[i][j][l] || (dp[i][j+l-k][k] && dp[i+k][j][l-k]);
          }
        }
      }
    }
    return dp[0][0][n];
  }
};

int main(int argc, const char *argv[])
{
  string s1 = "great",
         s2 = "rgtae";
  // string s1 = "abb",
  //        s2 = "bba";
  // string s1 = "abcd",
  //        s2 = "bdac";
  Solution so;
  cout << boolalpha << so.isScramble(s1, s2) << endl;
  return 0;
}
