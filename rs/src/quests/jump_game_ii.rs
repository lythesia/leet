/**
 * [45] Jump Game II
 *
 * Given an array of non-negative integers nums, you are initially positioned at the first index of the array.
Each element in the array represents your maximum jump length at that position.
Your goal is to reach the last index in the minimum number of jumps.
You can assume that you can always reach the last index.
 
Example 1:
Input: nums = [2,3,1,1,4]
Output: 2
Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.

Example 2:
Input: nums = [2,3,0,1,4]
Output: 2

 
Constraints:

	1 <= nums.length <= 104
	0 <= nums[i] <= 1000

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // greedy
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_reach = 0;
        let mut cur_max_reach = 0;
        let mut ans = 0;
        for (i, x) in nums.into_iter().enumerate() {
            if i > max_reach { // we need to jump
                ans += 1;
                max_reach = cur_max_reach;
                if max_reach >= n - 1 { break; }
            }
            cur_max_reach = std::cmp::max(cur_max_reach, i + x as usize);
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
