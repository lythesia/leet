#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  inline bool decode(char c) {
    return c > '0' && c <= '9';
  }

  inline bool decode(string s) {
    assert(s.length() == 2);
    int n = atoi(s.c_str());
    return s[0] > '0' && n >=10 && n <= 26;
  }

  int numDecodings(string s) {
    size_t n = s.length();
    if(!n) return 0;
    vector<int> dp(n+1, 0);
    dp[0] = 1, dp[1] = decode(s[0]);
    for(size_t i=1; i<n; i++) {
      dp[i+1] = (decode(s[i]) ? dp[i]:0) + (decode(s.substr(i-1, 2)) ? dp[i-1]:0);
    }
    return dp[n];
  }
};

int main(int argc, const char *argv[])
{
  // string s = "12";
  string s = "112";
  // string s = "0";
  Solution so;
  cout << so.numDecodings(s) << endl;
  return 0;
}
