#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
class Solution {
public:
  // pos ver
  // void dfs(vvi &ans, vi s, vi &S, int i) {
  //   if(i == (int)S.size()) ans.push_back(s);
  //   else {
  //     int cur = S[i];
  //     int j = find_if_not(S.begin()+i, S.end(), [&cur](int t){ return t == cur; }) - S.begin();
  //     for(int p=i; p<=j; p++) {
  //       int n = s.size();
  //       copy(S.begin()+i, S.begin()+p, back_inserter(s));
  //       dfs(ans, s, S, j);
  //       s.erase(s.begin()+n, s.end()); // restore!
  //     }
  //   }
  // }

  // iter ver
  template <typename I>
  void dfs(vvi &ans, vi s, vi &S, I i) {
    if(i == S.end()) ans.push_back(s);
    else {
      auto j = find_if_not(i, S.end(), [&i](int t){ return t == *i; });
      for(auto p=i; p-j<=0; p++) {
        copy(i, p, back_inserter(s));
        dfs(ans, s, S, j);
        s.erase(s.end() - (p-i), s.end());
      }
    }
  }

  vvi subsetsWithDup(vi &S) {
    sort(S.begin(), S.end());
    vvi ans;
    dfs(ans, vi(), S, S.begin());
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  // vi S = {1,2,2};
  vi S = {5,5,5,5,5};
  Solution so;
  for(vi s : so.subsetsWithDup(S)) {
    cout << "[ ";
    copy(s.begin(), s.end(), ostream_iterator<int>(cout, " "));
    cout << "]" << endl;
  }
  return 0;
}
