/**
 * [704] Binary Search
 *
 * Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
You must write an algorithm with O(log n) runtime complexity.
 
Example 1:
Input: nums = [-1,0,3,5,9,12], target = 9
Output: 4
Explanation: 9 exists in nums and its index is 4

Example 2:
Input: nums = [-1,0,3,5,9,12], target = 2
Output: -1
Explanation: 2 does not exist in nums so return -1

 
Constraints:

	1 <= nums.length <= 104
	-104 < nums[i], target < 104
	All the integers in nums are unique.
	nums is sorted in ascending order.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn helper(nums: &[i32], start: i32, target: i32) -> i32 {
        if nums.is_empty() { -1 }
        else if nums.len() == 1 {
            if nums[0] == target { start }
            else { -1 }
        } else {
            let n = nums.len() / 2;
            let mid = nums[n];
            if target < mid { Self::helper(&nums[..n], start, target) }
            else if target == mid { start + n as i32 }
            else { Self::helper(&nums[n+1..], start + (n + 1) as i32, target) }
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::helper(&nums[..], 0, target)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::search(vec![-1,0,3,5,9,12], 9));
        assert_eq!(-1, Solution::search(vec![-1,0,3,5,9,12], 2));
    }
}
