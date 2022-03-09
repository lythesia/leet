/**
 * [152] Maximum Product Subarray
 *
 * Given an integer array nums, find a contiguous non-empty subarray within the array that has the largest product, and return the product.
The test cases are generated so that the answer will fit in a 32-bit integer.
A subarray is a contiguous subsequence of the array.
 
Example 1:
Input: nums = [2,3,-2,4]
Output: 6
Explanation: [2,3] has the largest product 6.

Example 2:
Input: nums = [-2,0,-1]
Output: 0
Explanation: The result cannot be 2, because [-2,-1] is not a subarray.

 
Constraints:

	1 <= nums.length <= 2 * 104
	-10 <= nums[i] <= 10
	The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // dp[i+1]max = if a[i+1] >= 0, dp[i] or dp[i]max * a[i+1]
    pub fn max_product(nums: Vec<i32>) -> i32 {
        use std::cmp::{min, max};
        let mut ans = nums[0];
        let (mut dpmin, mut dpmax) = (ans, ans);
        for &x in &nums[1..] {
            // if x < 0 {
            //     dpmax = max(x, dpmin*x);
            //     dpmin = min(x, dpmax*x);
            // } else if x == 0 {
            //     dpmax = 0;
            //     dpmin = 0;
            // } else {
            //     dpmax = max(x, dpmax*x);
            //     dpmin = min(x, dpmin*x);
            // }
            // simplify to
            if x < 0 {
                std::mem::swap(&mut dpmax, &mut dpmin);
            }
            dpmax = max(x, dpmax*x);
            dpmin = min(x, dpmin*x);
            ans = max(ans, dpmax);
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
    }
}
