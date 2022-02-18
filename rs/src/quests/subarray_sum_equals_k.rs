/**
 * [560] Subarray Sum Equals K
 *
 * Given an array of integers nums and an integer k, return the total number of continuous subarrays whose sum equals to k.
 
Example 1:
Input: nums = [1,1,1], k = 2
Output: 2
Example 2:
Input: nums = [1,2,3], k = 3
Output: 2
 
Constraints:

	1 <= nums.length <= 2 * 104
	-1000 <= nums[i] <= 1000
	-107 <= k <= 107

 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
    // sum[i, j) = sum[0, j) - sum[0, i)
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let (mut cnt, mut sum) = (0, 0);
        let mut h = HashMap::<i32, i32>::new();

        for x in nums.iter() {
            // prefix sum[xi]
            sum += x; 
            let d = sum - k;
            if let Some(c) = h.get(&d) {
                cnt += c;
            }
            if d == 0 {
                cnt += 1;
            }
            // remember this sum[xi]
            let e = h.entry(sum).or_insert(0);
            *e += 1;
        }

        cnt
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
        assert_eq!(2, Solution::subarray_sum(vec![1, 2, 3], 3));
    }
}
