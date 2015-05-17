#include <bits/stdc++.h>
using namespace std;

typedef pair<char,int> pci;
class Solution {
public:
  string minWindow(string s, string t) {
    if(!s.length() || !t.length() || s.length() < t.length()) return "";
    int need[128] = {0}, has[128] = {0};
    for(char c : t) need[(int)c]++;
    int l = 0, r = 0, n = s.length(), count = t.length();
    for(; r<n && count; r++) {
      int c = s[r];
      has[c]++;
      if(need[c] && has[c] <= need[c]) count--;
    }
    if(count) return "";
    // shrink left
    while(true) {
      if(need[(int)s[l]] < has[(int)s[l]]) has[(int)s[l]]--, l++;
      else break;
    }
    int mmin = r-l, minl = l;
    cout << "1st win: " << s.substr(minl, mmin) << endl;

    for(; r<n; r++) {
      has[(int)s[r]]++;
      if(s[r] == s[l]) {  // match
        // remove extra
        do {
          has[(int)s[l]]--, l++;
        } while(need[(int)s[l]] < has[(int)s[l]]);
      }
      if(mmin > r-l+1) mmin = r-l+1, minl = l;
      cout << endl;
    }
    return s.substr(minl, mmin);
  }
};

int main(int argc, const char *argv[])
{
  string s = "AD0BEC0DEBANC",
         t = "ABC";
  Solution so;
  cout << so.minWindow(s, t) << endl;
  return 0;
}


