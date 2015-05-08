#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
class Solution {
public:
  template <typename I>
  void dfs(vvi &ans, vi &s, int sum, int t, vi &can, I p) {
    if(p == can.end()) return;
    else {
      int ss = sum + *p;
      if(ss <= t) {
        auto j = find_if_not(p, can.end(), [&p](int tt){return tt == *p;});
        if(ss < t) {
          for(auto i=p; i-j<=0 && sum + (*p)*(i-p) <= t; i++) {
            copy(p, i, back_inserter(s));
            if(sum + (*p)*(i-p) == t) {
              ans.push_back(s);
              s.erase(s.end() - (i-p), s.end());
              return;
            }
            dfs(ans, s, sum + (*p)*(i-p), t, can, j);
            s.erase(s.end() - (i-p), s.end());
          }
        }
        else if(ss == t) {
          s.push_back(*p);
          ans.push_back(s);
          s.pop_back();
          return;
        }
      }
      else return;
    }
  }

  vvi combinationSum2(vi &can, int t) {
    sort(can.begin(), can.end());
    vvi ans;
    vi s;
    dfs(ans, s, 0, t, can, can.begin());
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  // vi can = {10,1,2,7,6,1,5};
  // int t = 8;
  // vi can = {2,5,2,1,2};
  // int t = 5;
  vi can = {1,1};
  int t = 2;
  Solution so;
  vvi ans = so.combinationSum2(can, t);
  for(auto &v : ans) {
    cout << "[ ";
    copy(v.begin(), v.end(), ostream_iterator<int>(cout, " "));
    cout << "]\n";
  }
  return 0;
}
