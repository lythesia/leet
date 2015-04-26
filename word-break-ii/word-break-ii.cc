#include <bits/stdc++.h>
using namespace std;

typedef vector<string> vs;
typedef unordered_set<string> us;
typedef vector<int> vi;
typedef vector<vi> vvi;
class Solution {
public:
  void gao(vs &ans, string sol, int p, string &s, vvi &dp) {
    if(p < 0) ans.push_back(sol);
    else {
      for(int ss : dp[p]) {
        string w = s.substr(ss, p-ss+1);
        if(p != (int)s.size()-1) w += " ";
        gao(ans, w+sol, ss-1, s, dp);
      }
    }
  }

  vs wordBreak(string s, us &dict) {
    int n = s.size();
    vvi dp(n, vi());
    bool flag = false;
    for(int i=0; i<n; i++) {
      for(int j=0; j<=i; j++)
        if(dict.find(s.substr(j,i-j+1)) != dict.end()){
          dp[i].push_back(j); // j --> i (via a word)
          if(!j) flag = true; // ensure solution to avoid needless backtracking
        }
    }
    vs ans;
    if(flag) gao(ans, "", n-1, s, dp);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  string s = "catsanddog";
  // string s = "baaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
  us d;
  vs vd = {"cat", "cats", "and", "sand", "dog"};
  // vs vd = {"a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"};
  for(auto &ss : vd) d.insert(ss);
  Solution so;
  for(auto &ss : so.wordBreak(s, d)) cout << "[" << ss << "]" << endl;
  return 0;
}
