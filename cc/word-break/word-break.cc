#include <bits/stdc++.h>
using namespace std;

typedef unordered_set<string> us;
class Solution {
public:
  /*
   * word in dict can be used 0+ times
   * BFS-like DP: dp(i) = s(0..i) can be wrapped by dict
   * then dp(i + len(w)) = dp(i) && s(i..n) begin with w
   */
  bool wordBreak(string s, us &dict) {
    size_t n = s.length();
    vector<bool> dp(n+1, false);
    dp[0] = true;
    for(size_t i=0; i<n; i++) {
      if(dp[i]) {
        for(auto w : dict) {
          size_t wlen = w.length();
          if(s.find(w, i) == i) dp[i+wlen] = true; 
        }
      }
    }
    return dp[n];
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  unordered_set<string> dict;
  dict.insert("a");
  dict.insert("b");
  cout << boolalpha << so.wordBreak("abbb", dict) << endl;
  return 0;
}
