#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<bool> vb;
typedef vector<vb> vvb;
class Solution {
public:
  bool isPali(string &s) {
    size_t n = s.length();
    if(n < 2) return true;
    string t = s;
    reverse(t.begin(), t.end());
    return t == s;
  }

  // TLE
  void dfs(string &s, size_t p, int k, int &K) {
    if(!K) return;
    size_t n = s.length();
    if(p == n) return;
    for(size_t l=n-p; l>0; l--) {
      string p1 = s.substr(p, l), p2 = s.substr(p+l);
      bool f1 = isPali(p1), f2 = isPali(p2);
      if(f1 && f2) {
        if(l < n-p) ++k;
        K = min(k, K);
        return;
      }
      else if(f1) {
        dfs(s, p+l, k+1, K);
      }
    }
  }

  /*
   * dp(i): min cut for s[0..i)
   * dp(i) = min(dp(i), dp(j) + 1) for j < i
   */
  int minCut(string s) {
    int n = s.length();
    if(n < 2) return 0;
    vi dp(n+1);
    vvb pali(n+1, vb(n+1, false));
    for(int i=0; i<=n; i++) dp[i] = i-1; // init, since length of i at most split i-1 times
    for(int i=2; i<=n; i++) {
      for(int j=i-1; j>=0; j--) {
        if(s[i-1]==s[j] && (i-1-j<2 || pali[j+1][i-1])) { // opt for check pali, pali(i,j) = s[i-1]==s[j] && pali(j+1, i-1), the <2 exp is a shortcut(since 0 or 1 is trivial pali), so no need to init a full pali[n][n]
          pali[j][i] = true, dp[i] = min(dp[i], dp[j]+1);
        }
      }
    }
    return dp[n];
  }
};

int main(int argc, const char *argv[])
{
  // string s = "abb";
  // string s = "fifgbeajcacehiicccfecbfhhgfiiecdcjjffbghdidbhbdbfbfjccgbbdcjheccfbhafehieabbdfeigbiaggchaeghaijfbjhi"; // 75
  // string s = "seeslaveidemonstrateyetartsnomedievalsees";
  string s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
  Solution so;
  cout << boolalpha << so.isPali(s) << endl;
  cout << so.minCut(s) << endl;
  return 0;
}
