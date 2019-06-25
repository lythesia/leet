#include <bits/stdc++.h>
using namespace std;

typedef vector<string> vs;
const char tab[][5] = {
  "",
  "",
  "abc",
  "def",
  "ghi",
  "jkl",
  "mno",
  "pqrs",
  "tuv",
  "wxyz"
};
class Solution {
public:
  void dfs(vs &ans, string &s, const char *p) {
    if(!*p) ans.push_back(s);
    else {
      int d = *p - '0', n = strlen(tab[d]);
      for(int i=0; i<n; i++) {
        s.push_back(tab[d][i]);
        dfs(ans, s, p+1);
        s.pop_back();
      }
    }
  }

  vs letterCombinations(string digits) {
    vs ans;
    if(digits.empty()) return ans;
    string s;
    dfs(ans, s, digits.c_str());
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  string ds = "23";
  Solution so;
  vs ans = so.letterCombinations(ds);
  copy(ans.begin(), ans.end(), ostream_iterator<string>(cout, " ")); cout << endl;
  return 0;
}
