/**
 * [198] House Robber
 *
 * You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.
 
Example 1:
Input: nums = [1,2,3,1]
Output: 4
Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
Total amount you can rob = 1 + 3 = 4.

Example 2:
Input: nums = [2,7,9,3,1]
Output: 12
Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
Total amount you can rob = 2 + 9 + 1 = 12.

 
Constraints:

	1 <= nums.length <= 100
	0 <= nums[i] <= 400

 */
pub struct Solution {}

// submission codes start here

const NO_LAST: usize = 0;
const TAKE_LAST: usize = 1;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let n = nums.len();

        // note dp[i][_] only depends dp[i-1], which can be compressed
        // (dp_no_last, dp_last) = (max{dp_no_last, dp_last}, dp_no_last + a[i])
        // let mut dp = [[0; 2]; 101];
        // dp[0][NO_LAST] = 0; dp[0][TAKE_LAST] = nums[0];
        // for i in 1..n {
        //     dp[i][NO_LAST] = max(dp[i-1][NO_LAST], dp[i-1][TAKE_LAST]); // not take a[i]
        //     dp[i][TAKE_LAST] = dp[i-1][NO_LAST] + nums[i]; // take a[i]
        // }
        // max(dp[n-1][NO_LAST], dp[n-1][TAKE_LAST])

        let (mut dp_no_last, mut dp_take_last) = (0, nums[0]);
        for i in 1..n {
            let keep = dp_no_last;
            dp_no_last = max(dp_no_last, dp_take_last);
            dp_take_last = keep + nums[i];
        }
        max(dp_no_last, dp_take_last)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
