/**
 * [918] Maximum Sum Circular Subarray
 *
 * Given a circular integer array nums of length n, return the maximum possible sum of a non-empty subarray of nums.
A circular array means the end of the array connects to the beginning of the array. Formally, the next element of nums[i] is nums[(i + 1) % n] and the previous element of nums[i] is nums[(i - 1 + n) % n].
A subarray may only include each element of the fixed buffer nums at most once. Formally, for a subarray nums[i], nums[i + 1], ..., nums[j], there does not exist i <= k1, k2 <= j with k1 % n == k2 % n.
 
Example 1:
Input: nums = [1,-2,3,-2]
Output: 3
Explanation: Subarray [3] has maximum sum 3.

Example 2:
Input: nums = [5,-3,5]
Output: 10
Explanation: Subarray [5,5] has maximum sum 5 + 5 = 10.

Example 3:
Input: nums = [-3,-2,-3]
Output: -2
Explanation: Subarray [-2] has maximum sum -2.

 
Constraints:

	n == nums.length
	1 <= n <= 3 * 104
	-3 * 104 <= nums[i] <= 3 * 104

 */
pub struct Solution {}

// submission codes start here

use std::cmp::max;
impl Solution {
    // Kadane's max sum of subarray
    // dp[i] = max sum subarray ends at a[i]
    // dp[i+1] = a[i+1] + max(dp[i], 0)
    fn kadane(a: &Vec<i32>) -> i32 {
        let (mut ans, mut dp) = (a[0], a[0]);
        for &x in &a[1..] {
            dp = x + max(dp, 0);
            ans = max(ans, dp);
        }
        ans
    }

    // circular has 2 cases:
    // 1. one interval with 1..n: just use Kadane
    // 2. two intervals: 0..i as leftsum and j..n as rightsum where j > i:
    //  for two intervals, 
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 { return nums[0]; }
        // compute one-interval case at first
        let mut ans = Self::kadane(&nums);

        // now two cases:
        // rightsum[i] = a[i] + .. + a[n-1]
        let mut rightsum = vec![0; n];
        rightsum[n-1] = nums[n-1];
        for i in (0..n-1).rev() {
            rightsum[i] = rightsum[i+1] + nums[i];
        }
        // maxright[i] = max rightsum[j] in all j >= i
        let mut maxright = vec![0; n];
        maxright[n-1] = nums[n-1];
        for i in (0..n-1).rev() {
            maxright[i] = max(maxright[i+1], rightsum[i]);
        }
        // now two interval sum is from all leftsum[i](=a[0] + .. a[i]) + maxright[i+2], i in [0..n-2]
        let mut leftsum = 0;
        for i in 0..n-2 {
            leftsum += nums[i];
            ans = max(ans, leftsum + maxright[i+2]);
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::max_subarray_sum_circular(vec![1,-2,3,-2]));
        assert_eq!(10, Solution::max_subarray_sum_circular(vec![5,-3,5]));
        assert_eq!(-2, Solution::max_subarray_sum_circular(vec![-3,-2,-3]));
    }
}
