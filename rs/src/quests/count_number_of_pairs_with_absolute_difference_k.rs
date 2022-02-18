/**
 * [2006] Count Number of Pairs With Absolute Difference K
 *
 * Given an integer array nums and an integer k, return the number of pairs (i, j) where i < j such that |nums[i] - nums[j]| == k.
The value of |x| is defined as:

	x if x >= 0.
	-x if x < 0.

 
Example 1:
Input: nums = [1,2,2,1], k = 1
Output: 4
Explanation: The pairs with an absolute difference of 1 are:
- [1,2,2,1]
- [1,2,2,1]
- [1,2,2,1]
- [1,2,2,1]

Example 2:
Input: nums = [1,3], k = 3
Output: 0
Explanation: There are no pairs with an absolute difference of 3.

Example 3:
Input: nums = [3,2,1,5,4], k = 2
Output: 3
Explanation: The pairs with an absolute difference of 2 are:
- [3,2,1,5,4]
- [3,2,1,5,4]
- [3,2,1,5,4]

 
Constraints:

	1 <= nums.length <= 200
	1 <= nums[i] <= 100
	1 <= k <= 99

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let mut h = [0_i32; 101];

        for i in &nums {
            let (x, y) = (i - k, k + i);
            if x > 0 {
                ret += h[x as usize];
            }
            if y < 101 {
                ret += h[y as usize];
            }
            h[*i as usize] += 1;
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
        assert_eq!(4, Solution::count_k_difference(vec![1,2,2,1], 1));
        assert_eq!(0, Solution::count_k_difference(vec![1,3], 3));
        assert_eq!(3, Solution::count_k_difference(vec![3,2,1,5,4], 2));
    }
}
