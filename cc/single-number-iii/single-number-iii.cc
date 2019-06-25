#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int lowbit(int x) { return x & (-x); }
  vi singleNumber(vi &n) {
    int s = accumulate(n.begin(), n.end(), 0, bit_xor<int>()), x = lowbit(s), a = 0, b = 0;
    for(int i : n) (i & x) ? a ^= i : b ^= i;
    return vi{a, b};
  }
};

int main(int argc, const char *argv[])
{
  vi n = {1, 2, 1, 3, 2, 5};
  Solution so;
  vi ans = so.singleNumber(n);
  cout << ans[0] << ", " << ans[1] << endl;
  return 0;
}
