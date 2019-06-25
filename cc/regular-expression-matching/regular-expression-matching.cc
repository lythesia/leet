#include <bits/stdc++.h>
using namespace std;

typedef vector<string> vs;
typedef vector<bool> vb;
typedef vector<vb> vvb;
class Solution {
public:
  inline bool match(const char *s, const char *p) {
    return (*p == *s) || (*p == '.' && *s);
  }

  bool isMatch(const char *s, const char *p) {
    if(!*p) return !*s;

    if(*(p+1) != '*') { // ww
      if(!match(s, p)) return false;
      else return isMatch(s+1, p+1);
    }
    else {  // w*
      if(isMatch(s, p+2)) return true; // since we can ignore w*
      while(match(s, p)) if(isMatch(++s, p+2)) return true; // keep consuming w, and try match after w*
    }
    return false;
  }
  
  bool isMatch(string s, string t) { return isMatch(s.c_str(), t.c_str()); }
};

int main(int argc, const char *argv[])
{
  vs s = { "ab", "bbbba", "a",      "aaa",      "aab",   "baabbbaccbccacacc", "aaa",  "aaba",     "",   "abcaaaaaaabaabcabac"},
     p = { ".*", ".*a*a", ".*..a*", "ab*a*c*a", "c*aab", "c*..b*a*a.*a..*c",  "ab*a", "ab*a*c*a", ".*", ".*ab.a.*a*a*.*b*b*"};
        // 1     1         0         1           1        1                    0      0           1     1
  Solution so;
  for(size_t i=0; i<s.size(); i++)
    cout << boolalpha << so.isMatch(s[i], p[i]) << endl;
  return 0;
}
