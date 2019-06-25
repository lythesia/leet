#include <cstdio>
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

/* not bother dp!! */
#if 0
int maxArea(vi &height) {
  vi dp(height.size()-1, 0);
  for(int i=1; i<height.size(); i++) {
    int h = height[i];
    for(int j=0; j<i; j++) {
      dp[j] = max(dp[j], min(h, height[j]) * (i-j));
    }
  }
  int res = 0;
  for(int i : dp) res = max(res, i);
  return res;
}
#endif
typedef vector<int> vi;
class Solution {
public:
  int maxArea(vi &height) {
    int l = 0, r = height.size() - 1, res = 0;
    while(l < r) {
      res = max(res, (r-l)*(min(height[l], height[r])));
      height[l] < height[r] ? ++l : --r; // dir for possible larger volume
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  vi height = {4, 8, 1, 0, 9, 5};
  Solution so;
  cout << so.maxArea(height) << endl;
  return 0;
}
