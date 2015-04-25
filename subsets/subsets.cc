#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
class Solution {
public:
  void dfs(vvi &ans, vi s, vi &S, size_t i) {
    if(i == S.size()) ans.push_back(s);
    else {
      dfs(ans, s, S, i+1);
      s.push_back(S[i]);
      dfs(ans, s, S, i+1);
    }
  }

  vvi subsets(vi &S) {
    vvi ans;
    sort(S.begin(), S.end());
    dfs(ans, vi(), S, 0);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  vi S = {4,1,0};
  Solution so;
  for(vi s : so.subsets(S)) {
    cout << "[ ";
    copy(s.begin(), s.end(), ostream_iterator<int>(cout, " "));
    cout << "]" << endl;
  }
  return 0;
}
