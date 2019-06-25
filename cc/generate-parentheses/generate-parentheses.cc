#include <bits/stdc++.h>
using namespace std;

typedef vector<string> vs;
typedef vector<char> vc;
class Solution {
public:
  void gao(vs &a, vc &s, int l, int r, int n) {
    if(l == r && l == n) {
      a.push_back(string(s.begin(), s.end()));
      return;
    }
    if(l == n || (l && l > r)) {
      s.push_back(')');
      gao(a, s, l, r+1, n);
      s.pop_back();
    }
    if(l < n) {
      s.push_back('(');
      gao(a, s, l+1, r, n);
      s.pop_back();
    }
  }

  vs generateParenthesis(int n) {
    vs ans;
    vc s;
    gao(ans, s, 0, 0, n);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  int n = 2;
  Solution so;
  for(auto &i : so.generateParenthesis(n)) cout << i << endl;
  return 0;
}
