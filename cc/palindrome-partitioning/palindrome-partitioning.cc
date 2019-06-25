#include <bits/stdc++.h>
using namespace std;

typedef vector<string> vs;
typedef vector<vs> vvs;
class Solution {
public:
  bool isPali(string s) {
    int n = s.length();
    if(n < 2) return true;
    string t = s;
    reverse(t.begin(), t.end());
    return t == s;
  }

  void dfs(string &s, size_t p, vs &pa, vvs &ans) {
    size_t n = s.length();
    if(p == n) {
      ans.push_back(pa);
      return;
    }
    for(size_t l=1; p+l<=n; l++) {
      string p1 = s.substr(p, l);
      if(isPali(p1)) {
        pa.push_back(p1);
        dfs(s, p+l, pa, ans);
        pa.pop_back();
      }
    }
  }

  vvs partition(string s) {
    vvs ans;
    if(s.empty()) return ans;
    vs part;
    dfs(s, 0, part, ans);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  string s = "aab";
  Solution so;
  vvs ans = so.partition(s);
  for(vs &v : ans) {
    cout << "[ ";
    copy(v.begin(), v.end(), ostream_iterator<string>(cout, " "));
    cout << "]\n";
  }
  return 0;
}
