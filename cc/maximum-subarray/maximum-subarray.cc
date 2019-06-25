#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  int maxSubArray(vi nums) {
    int ans = 0, mmax = 0, least = INT_MIN;
    for(int i : nums) {
      if(i >= 0) mmax += i, ans = max(ans, mmax);
      else if(i + mmax > 0) mmax += i;
      else mmax = 0;
      least = max(least, i);
    }
    return ans ? ans : least;
  }

  /*
   * dp[i]: must contain i as end of sub-array
   * dp[i] = a[i] + dp[i-1] > 0 ? dp[i-1] : 0
   * since from i: 0->n, dp[i] just use previous dp[i-1], we can reduce the space
   * dp = a[i] + dp > 0 ? dp : 0, then ans = max(ans, dp) during each i: 0->n
   */
  int alt(vi nums) {
    int n = nums.size(), dp = nums[0], ans = nums[0]; // initial status
    for(int i=1; i<n; i++) dp = nums[i] + (dp > 0 ? dp : 0), ans = max(ans, dp);
    return ans;
  }

  /*
   * divide&conquer
   * 1. select middle a[k], there will be 3 situations
   * 2.1 a[k] is in the max sub-array, then from it on, find the max prefix array on left/suffix on right, sums up
   * 2.2 a[k] is is not in the max sub-array, it's in left, so recursive to left
   * 2.3 ... right
   * 3. pick the maxium of 2.{1,2,3}
   */
  int alt2(vi nums) {
    return alt2aux(nums, 0, nums.size() - 1);
  }

  int alt2aux(const vi &a, int l, int r) {
    if(l == r) return a[l];
    int m = (l + r) / 2, lans = alt2aux(a, l, m), rans = alt2aux(a, m+1, r);
    int prefix = a[m], suffix = a[m+1];
    for(int i=m, t=0; i>=l; i--) t += a[i], prefix = max(prefix, t);
    for(int i=m+1, t=0; i<=r; i++) t += a[i], suffix = max(suffix, t);
    return max(max(lans, rans), prefix + suffix);
  }
};

int main(int argc, const char *argv[])
{
  vi a = {-2,1,-3,4,-1,2,1,-5,4};
  // vi a = {-1};
  Solution so;
  cout << so.alt2(a) << endl;
  return 0;
}
