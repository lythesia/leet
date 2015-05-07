#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  // ensure one-to-one!
  bool isIsomorphic(string s, string t) {
    if(s.length() != t.length()) return false;
    unordered_map<char, char> h[2];
    for(int i=0; i<(int)s.length(); i++) {
      if(h[0].find(s[i]) != h[0].end()) {
        if(h[0][s[i]] != t[i]) return false;
      }
      else {
        if(h[1].find(t[i]) != h[1].end()) return false;
        else h[0][s[i]] = t[i], h[1][t[i]] = s[i];
      }
    }
    return true;
  }
};

int main(int argc, const char *argv[])
{
  string s, t;
  Solution so;
  while(cin >> s >> t)
    cout << boolalpha << so.isIsomorphic(s, t) << endl;
  return 0;
}
