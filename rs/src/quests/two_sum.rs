/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they add up to a specific target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

Example:

Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].


 */
use std::collections::HashMap;
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            if h.contains_key(x) {
                return vec![h[x], i as i32];
            } else {
                h.insert(target - x, i as i32);
            }
        }
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
