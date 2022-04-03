/**
 * [209] Minimum Size Subarray Sum
 *
 * Given an array of positive integers nums and a positive integer target, return the minimal length of a contiguous subarray [numsl, numsl+1, ..., numsr-1, numsr] of which the sum is greater than or equal to target. If there is no such subarray, return 0 instead.
 
Example 1:
Input: target = 7, nums = [2,3,1,2,4,3]
Output: 2
Explanation: The subarray [4,3] has the minimal length under the problem constraint.

Example 2:
Input: target = 4, nums = [1,4,4]
Output: 1

Example 3:
Input: target = 11, nums = [1,1,1,1,1,1,1,1]
Output: 0

 
Constraints:

	1 <= target <= 109
	1 <= nums.length <= 105
	1 <= nums[i] <= 105

 
Follow up: If you have figured out the O(n) solution, try coding another solution of which the time complexity is O(n log(n)).
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = usize::MAX;
        let mut i = 0;
        let mut s = 0;
        for j in 0..n {
            if s + nums[j] < target {
                s += nums[j];
            } else {
                while s + nums[j] >= target {
                    ans = ans.min(j - i + 1);
                    s -= nums[i];
                    i += 1;
                }
                // now s + nums[j] < target
                s += nums[j];
            }
        }
        if ans == usize::MAX {0}
        else {ans as i32}
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
