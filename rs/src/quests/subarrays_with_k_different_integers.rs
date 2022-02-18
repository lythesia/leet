/**
 * [992] Subarrays with K Different Integers
 *
 * Given an integer array nums and an integer k, return the number of good subarrays of nums.
A good array is an array where the number of different integers in that array is exactly k.

	For example, [1,2,3,1,2] has 3 different integers: 1, 2, and 3.

A subarray is a contiguous part of an array.
 
Example 1:
Input: nums = [1,2,1,2,3], k = 2
Output: 7
Explanation: Subarrays formed with exactly 2 different integers: [1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2]

Example 2:
Input: nums = [1,2,1,3,4], k = 3
Output: 3
Explanation: Subarrays formed with exactly 3 different integers: [1,2,1,3], [2,1,3], [1,3,4].

 
Constraints:

	1 <= nums.length <= 2 * 104
	1 <= nums[i], k <= nums.length

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn longest_at_most_k(nums: &Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut h = std::collections::HashMap::new();
        let n = nums.len();
        let (mut l, mut r) = (0usize, 0usize);
        let mut ret = 0;

        while r < n {
            *h.entry(nums[r]).or_insert(0) += 1;
            r += 1;

            // shrink window from left
            while l < r && h.len() > k {
                *h.get_mut(&nums[l]).unwrap() -= 1;
                if h[&nums[l]] == 0 {
                    h.remove(&nums[l]);
                }
                l += 1;
            }
            ret += (r - l);
        }

        ret as i32
    }

    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        // sol-1
        // Self::longest_at_most_k(&nums, k) - Self::longest_at_most_k(&nums, k - 1)

        // sol-2
        let k = k as usize;
        let mut ret = 0;
        let mut set = std::collections::HashMap::new();
        let (mut l,  mut in_set_subarray_len) = (0, 0);

        for r in 0..nums.len() {
            *set.entry(nums[r]).or_insert(0) += 1; // expand one more
            // if uniq_set.len() > k, we shrink this window(though it's expanded right above line), and reset `prefix..`
            // why not `while`? because len inc only 1 each iter of loop
            if set.len() > k {
                *set.get_mut(&nums[l]).unwrap() -= 1;
                if set[&nums[l]] == 0 { set.remove(&nums[l]); }
                l += 1;
                in_set_subarray_len = 0;
            }
            // if nums[l] appear more than 1 time in uniq_set, it means with condition of (at most set.len() diff numbers)
            // holding, the subarray length can be decrease from left, thus inc prefix(`in_set_subarray_len`)
            while set[&nums[l]] > 1 {
                *set.get_mut(&nums[l]).unwrap() -= 1;
                l += 1;
                in_set_subarray_len += 1;
            }
            // when uniq_set.len() == k, the count of diff subarrays from index 0 up to index r is (1 + prefix)
            if set.len() == k {
                ret += in_set_subarray_len + 1;
            }
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7, Solution::subarrays_with_k_distinct(vec![1,2,1,2,3], 2));
    }
}
