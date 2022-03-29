/**
 * [162] Find Peak Element
 *
 * A peak element is an element that is strictly greater than its neighbors.
Given an integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks.
You may imagine that nums[-1] = nums[n] = -∞.
You must write an algorithm that runs in O(log n) time.
 
Example 1:
Input: nums = [1,2,3,1]
Output: 2
Explanation: 3 is a peak element and your function should return the index number 2.
Example 2:
Input: nums = [1,2,1,3,5,6,4]
Output: 5
Explanation: Your function can return either index number 1 where the peak element is 2, or index number 5 where the peak element is 6.
 
Constraints:

	1 <= nums.length <= 1000
	-231 <= nums[i] <= 231 - 1
	nums[i] != nums[i + 1] for all valid i.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut lo, mut hi) = (0, n - 1);
        if n <= 1 { return 0; }
        while lo < hi {
            let mi = (lo + hi)/2;
            if nums[mi] > nums[mi + 1] { hi = mi; } // left part must contain one
            else if nums[mi] < nums[mi + 1] { lo = mi + 1; } // or right part
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
    }
}
