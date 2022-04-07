/**
 * [910] Smallest Range II
 *
 * You are given an integer array nums and an integer k.
For each index i where 0 <= i < nums.length, change nums[i] to be either nums[i] + k or nums[i] - k.
The score of nums is the difference between the maximum and minimum elements in nums.
Return the minimum score of nums after changing the values at each index.
 
Example 1:
Input: nums = [1], k = 0
Output: 0
Explanation: The score is max(nums) - min(nums) = 1 - 1 = 0.

Example 2:
Input: nums = [0,10], k = 2
Output: 6
Explanation: Change nums to be [2, 8]. The score is max(nums) - min(nums) = 8 - 2 = 6.

Example 3:
Input: nums = [1,3,6], k = 3
Output: 3
Explanation: Change nums to be [4, 6, 3]. The score is max(nums) - min(nums) = 6 - 3 = 3.

 
Constraints:

	1 <= nums.length <= 104
	0 <= nums[i] <= 104
	0 <= k <= 104

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut ans = nums[n-1] - nums[0];
        for i in 0..n-1 {
            ans = ans.min(
                (nums[n-1] - k).max(nums[i] + k) - // smaller try +k
                (nums[0] + k).min(nums[i+1] - k)   // larger try -k
            )
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
