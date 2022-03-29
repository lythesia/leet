/**
 * [33] Search in Rotated Sorted Array
 *
 * There is an integer array nums sorted in ascending order (with distinct values).
Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.
You must write an algorithm with O(log n) runtime complexity.
 
Example 1:
Input: nums = [4,5,6,7,0,1,2], target = 0
Output: 4
Example 2:
Input: nums = [4,5,6,7,0,1,2], target = 3
Output: -1
Example 3:
Input: nums = [1], target = 0
Output: -1
 
Constraints:

	1 <= nums.length <= 5000
	-104 <= nums[i] <= 104
	All values of nums are unique.
	nums is an ascending array that is possibly rotated.
	-104 <= target <= 104

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering::{Greater, Less};
        if nums.is_empty() { return -1; }
        let n = nums.len();
        let mut max = nums[n - 1];
        // pivot is the smallest one(LO, next to largest, HI)
        let pivot = nums[0..n - 1].binary_search_by(|&x| {
            if x >= max { // [asc.. x .. HI LO(pivot) asc .. max]
                max = x;
                Less // pivot in right part: [pos(x) + 1, hi)
            } else { // [asc.. HI LO(pivot) asc .. x .. max]
                Greater // pivot in left part: [0, pos(x))
            }
        }).unwrap_or_else(|i| i);
        let (idx, offset) = if target > nums[n - 1] {
            (nums[0..pivot].binary_search(&target), 0)
        } else {
            (nums[pivot..].binary_search(&target), pivot)
        };
        match idx {
            Ok(i) => (i + offset) as i32,
            _ => -1,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::search(vec![4,5,6,7,0,1,2], 0));
    }
}
