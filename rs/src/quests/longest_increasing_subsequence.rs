/**
 * [300] Longest Increasing Subsequence
 *
 * Given an integer array nums, return the length of the longest strictly increasing subsequence.
A subsequence is a sequence that can be derived from an array by deleting some or no elements without changing the order of the remaining elements. For example, [3,6,2,7] is a subsequence of the array [0,3,1,6,2,2,7].
 
Example 1:
Input: nums = [10,9,2,5,3,7,101,18]
Output: 4
Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.

Example 2:
Input: nums = [0,1,0,3,2,3]
Output: 4

Example 3:
Input: nums = [7,7,7,7,7,7,7]
Output: 1

 
Constraints:

	1 <= nums.length <= 2500
	-104 <= nums[i] <= 104

 
Follow up: Can you come up with an algorithm that runs in O(n log(n)) time complexity?

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // patience sort
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = Vec::with_capacity(nums.len());
        for x in nums {
            if let Err(i) = dp.binary_search(&x) {
                if i >= dp.len() {
                    dp.push(x);
                } else {
                    dp[i] = x;
                }
            }
        }
        dp.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::length_of_lis(vec![10,9,2,5,3,7,101,18]));
        assert_eq!(4, Solution::length_of_lis(vec![0,1,0,3,2,3]));
    }
}
