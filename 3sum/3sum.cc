#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <vector>
#include <algorithm>
#include <set>

using namespace std;

#define see(x) cout << #x": " << (x) << endl
#define see_vec(v) for(auto __i : (v)) cout << __i << ' '; cout << endl
#define __pos(i) ((i) - num.begin())

typedef vector<int> vi;
typedef vector<vi> vvi;

class Solution {
public:
  vvi threeSum(vi &num) {
    vvi res;
    if(num.size() < 3) return res;
    sort(num.begin(), num.end());
    int prev_x = num[0] - 1; // x !== prev_x
    for(int i=0; i<num.size()-2; i++) {
      int x = num[i], l = i+1, r = num.size() - 1, prev_y = x-1; // y !== prev_y initially
      if(x == prev_x) continue;
      while(l < r) {
        int y = num[l], z = num[r];
        if(y == prev_y) {
          ++l;
          continue;
        }
        if(x + y + z == 0) {
          res.push_back(vi{x, y, z});
          ++l, --r;
          prev_y = y;
        }
        else if(x + y + z > 0) --r;
        else ++l;
      }
      prev_x = x;
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  int x;
  vi num;
  while(scanf("%d", &x) != EOF) {
    num.push_back(x);
  }
  vvi res = so.threeSum(num);
  for(auto _vi : res) {
    printf("( ");
    for(auto i: _vi) printf("%d ", i); puts(")");
  }
  return 0;
}
