#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

// referenced: base stol(left)
// what if stupid left / right divide?
class Solution {
public:
  void dfs(vs &ans, long acc, string s, string &n, size_t p, long lastv, char lastop, int t) {
    if(p == n.size() && acc == t) ans.push_back(s);
    else {
      for(size_t i=p+1; i<=n.size(); i++) {
        string left = n.substr(p, i - p);
        long leftn = stol(left);
        if(to_string(leftn).size() != left.size()) continue;
        dfs(ans, acc + leftn, s + '+' + left, n, i, leftn, '+', t);
        dfs(ans, acc - leftn, s + '-' + left, n, i, leftn, '-', t);
        long nacc = (lastop == '-') ? acc + lastv - lastv*leftn :
                    (lastop == '+') ? acc - lastv + lastv*leftn : acc * leftn;
        dfs(ans, nacc, s + '*' + left, n, i, lastv*leftn, lastop, t);
      }
    }
  }
  vs addOperators(string n, int t) {
    vs ans;
    if(n.empty()) return ans;
    for(size_t i=1; i<=n.size(); i++) {
      string left = n.substr(0, i);
      long leftn = stol(left);
      if(to_string(leftn).size() != left.size()) continue;
      dfs(ans, leftn, left, n, i, leftn, ' ', t);
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  string s = "105";
  int n = 5;
  vs ans = so.addOperators(s, n);
  copy(ans.begin(), ans.end(), ostream_iterator<string>(cout, " ")); cout << endl;
  return 0;
}
