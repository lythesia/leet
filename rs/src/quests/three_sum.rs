/**
 * [15] 3Sum
 *
 * Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
Notice that the solution set must not contain duplicate triplets.
 
Example 1:
Input: nums = [-1,0,1,2,-1,-4]
Output: [[-1,-1,2],[-1,0,1]]
Example 2:
Input: nums = []
Output: []
Example 3:
Input: nums = [0]
Output: []
 
Constraints:

	0 <= nums.length <= 3000
	-105 <= nums[i] <= 105

 */
pub struct Solution {}

// submission codes start here

use std::collections::HashSet;
impl Solution {
    fn two_sum(a: &[i32], t: i32) -> HashSet<(i32, i32)> {
        let mut h = HashSet::new();
        let mut ans = HashSet::new();
        for &x in a {
            let y = t - x;
            if h.contains(&y) {
                ans.insert((x, y));
            } else {
                h.insert(x);
            }
        }
        ans
    }

    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut h = HashSet::new();
        let mut ans = vec![];
        for (i, &x) in nums.iter().enumerate() {
            if !h.contains(&x) {
                h.insert(x);
                let twos = Self::two_sum(&nums[i+1..], -x);
                for (y, z) in twos {
                    ans.push(vec![x, y, z]);
                }
            }
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
