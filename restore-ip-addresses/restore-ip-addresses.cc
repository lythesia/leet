// #include <bits/stdc++.h>
#include <iostream>
#include <cstring>
#include <string>
#include <iterator>
#include <vector>
#include <algorithm>
using namespace std;

typedef vector<string> vs;
class Solution {
public:
  vs restoreIpAddresses(string s) {
    vs ans;
    if(s.length() > 12) return ans;
    string ip;
    dfs(s.c_str(), ip, 0, ans);
    return ans;
  }

  inline bool valid(string s) {
    int n = atoi(s.c_str());
    return n < 10 ? s.length() == 1 :
           n < 100 ? s.length() == 2 :
           n < 256 ? s.length() == 3 : false;
  }

  void dfs(const char *s, string ip, int cnt, vs &ans) {
    if(cnt == 4) {
      if(!*s) ans.push_back(ip);
      return;
    }
    for(int i=1; i<=3 && *(s+i-1); i++) {
      int len = strlen(s);
      if(len > (4-cnt)*3 || len < (4-cnt)) return; // 8ms -> 5ms
      string seg(s, i);
      if(valid(seg)) {
        string nip = ip + seg;
        if(cnt < 3) nip += ".";
        dfs(s+i, nip, cnt+1, ans);
      }
      else return;
    }
  }
};

int main(int argc, const char *argv[])
{
  string s = "25525511135";
  // string s = "0000";
  // string s = "010010";
  Solution so;
  vs ans = so.restoreIpAddresses(s);
  copy(ans.begin(), ans.end(), ostream_iterator<string>(cout, " "));
  return 0;
}
