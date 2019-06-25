#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int numDistinct(string s, string t) {
    size_t n = s.length(), m = t.length();
    if(n < m) return 0;
    vector<int> dp(m+1, 0);
    dp[0] = 1;
    for(size_t i=0; i<n; i++) {
      size_t p = string::npos;
      do {
        p = t.rfind(s[i], p);
        if(p != string::npos) dp[p+1] += dp[p]; // reversely to avoid repeat count
        else break;
        if(!p) break;
        else p--;
      } while(true);
    }
    return dp[m];
  }
};

int main(int argc, const char *argv[])
{
  // string s = "rabbbit",
  //        t = "rabbit";
  string s = "acabcabc",
         t = "abc";
  Solution so;
  cout << so.numDistinct(s, t) << endl;
  return 0;
}
