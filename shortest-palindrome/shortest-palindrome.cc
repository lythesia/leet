#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  bool ispali(string &s) {
    int len = s.length(), h = 0, t = len - 1;
    if(len < 2) return true;
    for(;h<t && s[h]==s[t]; h++,t--);
    return h > t;
  }
  string shortestPalindrome(string s) {
    int len = s.length();
    if(len < 2) return s;
    // Manachar
    vi p(2*s.length());
    for(int i=0, id=0, mx=0; i<=2*(len-1); i++) {
      int d = i < mx ? min(p[2*id-i], (mx-i)/2) : 0;
      int a = i/2 - d, b = (i+1)/2 + d;
      while(0<=a && b<len && s[a]==s[b]) a--, b++, d++;
      p[i] = d;
      if(mx < 2*b-1) id = i, mx = 2*b-1;
    }
    int mlen = 0, index = 1, clen = 0, ti = 0;
    // find larget pali that starts from 0
    for(int i=0; i<=2*(len-1); i++) {
      if(i & 1) {
        ti = (i - p[i]*2 + 1) / 2, clen = p[i]*2;
        if(!ti && clen > mlen) mlen = clen, index = ti;
      }
      else {
        ti = (i - p[i]*2 + 2) / 2, clen = p[i]*2 - 1;
        if(!ti && clen > mlen) mlen = clen, index = ti;
      }
    }
    if(index) return string(s.rbegin(), s.rend()-1) + s.substr(index);
    else return string(s.rbegin(), s.rbegin()+len-mlen) + s;
  }
};

int main(int argc, const char *argv[])
{
  // string s = "aacecaaa";
  string s = "abcd";
  // string s = "ababbbabbaba";
  Solution so;
  // cin >> s;
  cout << so.shortestPalindrome(s) << endl;
  return 0;
}
