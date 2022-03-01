/**
 * [35] Search Insert Position
 *
 * Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
You must write an algorithm with O(log n) runtime complexity.
 
Example 1:
Input: nums = [1,3,5,6], target = 5
Output: 2

Example 2:
Input: nums = [1,3,5,6], target = 2
Output: 1

Example 3:
Input: nums = [1,3,5,6], target = 7
Output: 4

 
Constraints:

	1 <= nums.length <= 104
	-104 <= nums[i] <= 104
	nums contains distinct values sorted in ascending order.
	-104 <= target <= 104

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len());
        while lo < hi {
            let mi = lo + (hi - lo)/2;
            if nums[mi] < target {
                lo = mi + 1;
            } else if nums[mi] > target {
                hi = mi;
            } else {
                return mi as i32;
            }
        }
        lo as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(0, Solution::search_insert(vec![1,3,5,6], 0))
        assert_eq!(0, Solution::search_insert(vec![1,3,5,6], 0));
    }
}
