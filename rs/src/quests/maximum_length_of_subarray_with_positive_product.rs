/**
 * [1567] Maximum Length of Subarray With Positive Product
 *
 * Given an array of integers nums, find the maximum length of a subarray where the product of all its elements is positive.
A subarray of an array is a consecutive sequence of zero or more values taken out of that array.
Return the maximum length of a subarray with positive product.
 
Example 1:
Input: nums = [1,-2,-3,4]
Output: 4
Explanation: The array nums already has a positive product of 24.

Example 2:
Input: nums = [0,1,-2,-3,-4]
Output: 3
Explanation: The longest subarray with positive product is [1,-2,-3] which has a product of 6.
Notice that we cannot include 0 in the subarray since that'll make the product 0 which is not positive.
Example 3:
Input: nums = [-1,-2,-3,0,1]
Output: 2
Explanation: The longest subarray with positive product is [-1,-2] or [-2,-3].

 
Constraints:

	1 <= nums.length <= 105
	-109 <= nums[i] <= 109

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // dp[i] end at i
    // if a[i] > 0, dp_pos[i] = dp_pos[i-1]+1, dp_neg[i] = dp_neg[i-1]+1
    // if a[i] = 0, dp_pos[i] = dp_neg[i] = 0,
    // if a[i] < 0, dp_pos[i] = dp_neg[i-1]+1, dp_neg[i] = dp_pos[i]+1
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        use std::mem::swap;
        use std::cmp::max;
        if let Some(i) = nums.iter().position(|&x| x != 0) {
            let mut ans = 0;
            let (mut dp_pos, mut dp_neg) = (0, 0);
            for &x in &nums[i..] {
                if x == 0 {
                    dp_neg = 0;
                    dp_pos = 0;
                } else {
                    // if x < 0 {
                    //     dp_neg = dp_pos + 1;
                    //     dp_pos = if dp_neg > 0 {dp_neg+1} else {0};
                    // } else {
                    //     dp_neg = if dp_neg > 0 {dp_neg+1} else {0};
                    //     dp_pos = dp_pos + 1;
                    // }
                    dp_neg = if dp_neg > 0 {dp_neg+1} else {0};
                    dp_pos = dp_pos + 1;
                    if x < 0 {
                        swap(&mut dp_neg, &mut dp_pos);
                    }
                }
                ans = max(ans, dp_pos);
            }
            ans
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::get_max_len(vec![1,-2,-3,4]));
        assert_eq!(3, Solution::get_max_len(vec![0,1,-2,-3,-4]));
        assert_eq!(2, Solution::get_max_len(vec![-1,-2,-3,0,1]));
    }
}
