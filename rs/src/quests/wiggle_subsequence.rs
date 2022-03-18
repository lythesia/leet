/**
 * [376] Wiggle Subsequence
 *
 * A wiggle sequence is a sequence where the differences between successive numbers strictly alternate between positive and negative. The first difference (if one exists) may be either positive or negative. A sequence with one element and a sequence with two non-equal elements are trivially wiggle sequences.

	For example, [1, 7, 4, 9, 2, 5] is a wiggle sequence because the differences (6, -3, 5, -7, 3) alternate between positive and negative.
	In contrast, [1, 4, 7, 2, 5] and [1, 7, 4, 5, 5] are not wiggle sequences. The first is not because its first two differences are positive, and the second is not because its last difference is zero.

A subsequence is obtained by deleting some elements (possibly zero) from the original sequence, leaving the remaining elements in their original order.
Given an integer array nums, return the length of the longest wiggle subsequence of nums.
 
Example 1:
Input: nums = [1,7,4,9,2,5]
Output: 6
Explanation: The entire sequence is a wiggle sequence with differences (6, -3, 5, -7, 3).

Example 2:
Input: nums = [1,17,5,10,13,15,10,5,16,8]
Output: 7
Explanation: There are several subsequences that achieve this length.
One is [1, 17, 10, 13, 10, 16, 8] with differences (16, -7, 3, -3, 6, -8).

Example 3:
Input: nums = [1,2,3,4,5,6,7,8,9]
Output: 2

 
Constraints:

	1 <= nums.length <= 1000
	0 <= nums[i] <= 1000

 
Follow up: Could you solve this in O(n) time?

 */
pub struct Solution {}

// submission codes start here

use std::cmp::{min, max};
impl Solution {
    // dp greedy: maintain 2 seq, end-with-up and end-with-down
    // try make the up-seq's last number as big as possible and the down-seq's last number as small as possible
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let (mut last_up, mut last_up_max) = ([1, 1], [nums[0], nums[0]]);
        let (mut last_down, mut last_down_min) = ([1, 1], [nums[0], nums[0]]);
        let mut i = 0;
        for &x in &nums[1..] {
            let j = 1 - i;
            // update up-seq
            if x > last_down_min[i] && last_down[i] + 1 > last_up[i] {
                last_up[j] = last_down[i] + 1;
                last_up_max[j] = x;
            } else {
                last_up[j] = last_up[i];
                last_up_max[j] = max(last_up_max[i], x);
            }
            // update down-seq
            if x < last_up_max[i] && last_up[i] + 1 > last_down[i] {
                last_down[j] = last_up[i] + 1;
                last_down_min[j] = x;
            } else {
                last_down[j] = last_down[i];
                last_down_min[j] = min(last_down_min[i], x);
            }
            i = j;
        }
        max(last_up[i], last_down[i])
    }
    
    // or simpler: just count totoal level changes
    // use std::cmp::Ordering;
    // pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    //     let mut prev = None;
    //     let mut answer = 1;
    //     for i in 1..nums.len() {
    //         let o = (nums[i] - nums[i - 1]).cmp(&0);
    //         if o == Ordering::Equal {
    //             continue;
    //         }
    //         if Some(o) != prev {
    //             answer += 1
    //         }
    //         prev = Some(o);
    //     }
    //     answer
    // }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7, Solution::wiggle_max_length(vec![1,17,5,10,13,15,10,5,16,8]));
    }
}
