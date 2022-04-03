/**
 * [713] Subarray Product Less Than K
 *
 * Given an array of integers nums and an integer k, return the number of contiguous subarrays where the product of all the elements in the subarray is strictly less than k.
 
Example 1:
Input: nums = [10,5,2,6], k = 100
Output: 8
Explanation: The 8 subarrays that have product less than 100 are:
[10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.

Example 2:
Input: nums = [1,2,3], k = 0
Output: 0

 
Constraints:

	1 <= nums.length <= 3 * 104
	1 <= nums[i] <= 1000
	0 <= k <= 106

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let mut prod = 1;
        let n = nums.len();
        for j in 0..n {
            if nums[j] >= k {
                i += 1;
                // + (j-i) .. 0
                while i <= j {
                    ans += j - i;
                    i += 1;
                }
                prod = 1;
                continue;
            }

            if prod*nums[j] < k {
                prod *= nums[j];
                ans += 1;
            } else {
                while prod*nums[j] >= k {
                    prod /= nums[i];
                    i += 1;
                    ans += j - i;
                }
                prod *= nums[j];
                ans += 1;
            }
        }
        i += 1;
        // + (n-i) .. 1 (or 0)
        while i < n {
            ans += n - i;
            i += 1;
        }
        ans as i32
    }
}

// submission codes end

use core::num;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8, Solution::num_subarray_product_less_than_k(vec![10,5,2,6], 100));
        assert_eq!(18, Solution::num_subarray_product_less_than_k(vec![10,9,10,4,3,8,3,3,6,2,10,10,9,3], 19));
        assert_eq!(31, Solution::num_subarray_product_less_than_k(vec![10,2,2,5,4,4,4,3,7,7], 289));
    }
}
