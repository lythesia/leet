/**
 * [896] Monotonic Array
 *
 * An array is monotonic if it is either monotone increasing or monotone decreasing.
An array nums is monotone increasing if for all i <= j, nums[i] <= nums[j]. An array nums is monotone decreasing if for all i <= j, nums[i] >= nums[j].
Given an integer array nums, return true if the given array is monotonic, or false otherwise.

Example 1:
Input: nums = [1,2,2,3]
Output: true

Example 2:
Input: nums = [6,5,4,4]
Output: true

Example 3:
Input: nums = [1,3,2]
Output: false


Constraints:

	1 <= nums.length <= 105
	-105 <= nums[i] <= 105

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n <= 1 { return true; }
        let sigs = (1..nums.len()).map(|i| (nums[i] - nums[i - 1]).signum()).collect::<Vec<_>>();
        sigs.iter().all(|&x| x >= 0) || sigs.iter().all(|&x| x <= 0)
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
