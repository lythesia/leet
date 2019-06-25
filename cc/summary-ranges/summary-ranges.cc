#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  static bool not_seq(long x, long y) { return y - x > 1; }
  vs summaryRanges(vi n) {
    vs ans;
    if(n.empty()) return ans;
    auto prev = n.begin();
    for(auto it=adjacent_find(n.begin(), n.end(), not_seq);
        it!=n.end();
        it=adjacent_find(it, n.end(), not_seq)) {
      if(it != prev) ans.emplace_back(to_string(*prev) + "->" + to_string(*it));
      else ans.emplace_back(to_string(*prev));
      prev = ++it;
    }
    if(prev + 1 == n.end()) ans.emplace_back(to_string(*prev));
    else ans.emplace_back(to_string(*prev) + "->" + to_string(n.back()));
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  // vi n = {0,1,2,4,5,7};
  // vi n = {1,2,3,5,7};
  vi n = {-2147483648,-2147483647,2147483647};
  Solution so;
  vs ans = so.summaryRanges(n);
  copy(ans.begin(), ans.end(), ostream_iterator<string>(cout, " ")); cout << endl;
  return 0;
}
