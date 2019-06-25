#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  int rob(vi &num) {
    int last = 0, nolast = 0;
    for(int i=1; i<=num.size(); i++) {
      /*
       * x  x  x  x  x  k ...
       *          |  |  `-> current
       *          |  `-> last (at most)
       *          `----> nolast (at most)
       */
      int t = nolast;
      nolast = max(nolast, last);
      last = t + num[i-1];
    }
    return max(nolast, last);
  }
};
