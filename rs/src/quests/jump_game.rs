/**
 * [55] Jump Game
 *
 * You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.
Return true if you can reach the last index, or false otherwise.
 
Example 1:
Input: nums = [2,3,1,1,4]
Output: true
Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.

Example 2:
Input: nums = [3,2,1,0,4]
Output: false
Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.

 
Constraints:

	1 <= nums.length <= 104
	0 <= nums[i] <= 105

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        // let mut p = 0;
        // while nums[p] > 0 {
        //     let mut two_step_max = p;
        //     for i in 1..=nums[p] {
        //         let i = i as usize;
        //         if p + i >= n { return true; }
        //         // from p, 1 step to at most p + i, 2 step to p + i + nums[p + i]
        //         if p + i + nums[p+i] as usize >= two_step_max + nums[two_step_max] as usize {
        //             two_step_max = p + i;
        //         }
        //     }
        //     p = two_step_max;
        //     if p + nums[p] as usize >= n - 1 { return true; }
        // }
        // false

        // try from backwards greedy
        let mut last = n - 1;
        // from last - 1 to 0
        for i in (0..last).rev() {
            // we can reach last from i
            if i + nums[i] as usize >= last { last = i; }
        }
        last == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(!Solution::can_jump(vec![3,2,1,0,4]));
    }
}
