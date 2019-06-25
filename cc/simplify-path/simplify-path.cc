#include <bits/stdc++.h>
using namespace std;

class Solution{
public:
  int cnts(string &s, char c, int p) {
    int cnt = 0, len = s.length();
    for(int i=p; i<len; i++) if(s[i] == c) cnt++; else break;
    return cnt;
  }

  string simplifyPath(string s) {
    int len = s.length(), i = 0;
    assert(len > 0 && s[0] == '/');
    string ans;
    while(i < len) {
      if(s[i] == '/') {
        int n = cnts(s, '/', i);
        if(i+n <= len && ans.back() != '/') ans.push_back('/');
        i += n;
      }
      else if(s[i] == '.') {
        int n = cnts(s, '.', i);
        assert(n >= 1);
        if(n <= 2) {
          if((i+n<len && s[i+n]!='/') || (s[i-1]!='/')) {
            ans += string(n, '.');
            i += n;
          }
          else {
            if(n == 2) {
              size_t p = ans.rfind('/', ans.length()-2);
              assert(p != string::npos);
              ans.erase(p);
              if(ans.empty()) ans = "/";
              i += 2;
            }
            else i += (n+1);
          }
        }
        else {
          ans += string(n, '.');
          i += n;
        }
      }
      else ans.push_back(s[i++]);
    }
    if(ans.back() == '/' && ans.length() > 1) ans.pop_back();
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  vector<string> v = {
    "/home/",
    "/a/./b/../../c/",
    "/../",
    "/home//foo/",
    "/.//////abc////////../",
    "/abc/.",
    "/.h",
    "/...",
    "/..",
    "/..hid",
    "/sdf..",
    "/fuck/../../..",
    "/a/./b/../../c/",
    "/home/foo/.ssh/../.ssh2/authorized_keys/",
  };
  Solution so;
  for(auto &s : v) {
    cout << s << ": " << so.simplifyPath(s) << endl;
  }
  return 0;
}
