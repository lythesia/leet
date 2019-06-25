#include <bits/stdc++.h>
using namespace std;

typedef vector<bool> vb;
typedef vector<vb> vvb;
class Solution {
public:
  bool isMatch(const char *s, const char *p) {
    const char *prev_star = NULL, *next_s = s;
    while(*s) {
      // match immediately
      if((*p == *s) || (*p == '?')) { ++s, ++p; continue; }
      // not match but *
      if(*p == '*') { prev_star = p++, next_s = s; continue; }
      // not match no *
      if(prev_star) { p = prev_star + 1, s = ++next_s; continue; }
      return false;
    }
    // s is over, p must be all *
    while(*p == '*') p++;
    return !*p;
  }

  bool isMatch(string s, string p) {
    return isMatch(s.c_str(), p.c_str());
  }
};

int main(int argc, const char *argv[])
{
  int cnt = 11;
  string s[] = {
    "aaabbbaabaaaaababaabaaabbabbbbbbbbaabababbabbbaaaaba",
    "",
    "",
    "",
    "a",
    "aa",
    "aa",
    "aaa",
    "aa",
    "aa",
    "ab",
    "aab"
  }, p[] = {
    "a*******b",
    "*",
    "?",
    "a",
    "",
    "a",
    "aa",
    "aa",
    "*",
    "a*",
    "?*",
    "c*a*b"
  };
  bool ans[] = {
    0,
    1,
    0,
    0,
    0,
    0,
    1,
    0,
    1,
    1,
    1,
  };
  Solution so;
  for(int i=0; i<cnt; i++) cout << boolalpha << so.isMatch(s[i], p[i]) << ":" << ans[i] << endl;
  // string ss, pp;
  // cin >> ss >> pp;
  // cout << boolalpha << so.isMatch(ss, pp) << endl;
  return 0;
}
