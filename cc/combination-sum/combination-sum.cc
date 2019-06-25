#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
class Solution {
public:
  void dfs(vvi &ans, vi &s, int sum, int t, vi &can, size_t p) {
    if(p == can.size()) return;
    else {
      for(size_t i=p; i<can.size(); i++) {
        int ss = sum + can[i];
        if(ss < t) {
          s.push_back(can[i]);
          dfs(ans, s, ss, t, can, i);
          s.pop_back();
        }
        else if(ss == t) {
          s.push_back(can[i]);
          ans.push_back(s);
          s.pop_back();
          return;
        }
        else return;
      }
    }
  }

  vvi combinationSum(vi &can, int t) {
    sort(can.begin(), can.end());
    auto it = unique(can.begin(), can.end());
    can.resize(distance(can.begin(), it));
    vvi ans;
    vi s;
    dfs(ans, s, 0, t, can, 0);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  vi can = {3,2,3,6,7};
  int t = 7;
  Solution so;
  vvi ans = so.combinationSum(can, t);
  for(auto &v : ans) {
    cout << "[ ";
    copy(v.begin(), v.end(), ostream_iterator<int>(cout, " "));
    cout << "]\n";
  }
  return 0;
}
