#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
  public:
    int lengthOfLIS(vector<int>& nums) {
      if(nums.empty()) return 0;
      int dp[nums.size() + 1], ans = 1;
      memset(dp, 0x3f, sizeof(int) * (nums.size() + 1));
      dp[ans] = nums[0];
      for(size_t i=1; i<nums.size(); i++) {
        if(nums[i] > dp[ans]) dp[++ans] = nums[i];
        else {
          int *p = lower_bound(dp + 1, dp + nums.size() + 1, nums[i]); // *p >= nums[i]
          assert(p != dp + nums.size() + 1);
          *p = nums[i];
        }
      }
      return ans;
    }
};
