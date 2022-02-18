/**
 * [1679] Max Number of K-Sum Pairs
 *
 * You are given an integer array nums and an integer k.
In one operation, you can pick two numbers from the array whose sum equals k and remove them from the array.
Return the maximum number of operations you can perform on the array.

Example 1:
Input: nums = [1,2,3,4], k = 5
Output: 2
Explanation: Starting with nums = [1,2,3,4]:
- Remove numbers 1 and 4, then nums = [2,3]
- Remove numbers 2 and 3, then nums = []
There are no more pairs that sum up to 5, hence a total of 2 operations.
Example 2:
Input: nums = [3,1,3,4,3], k = 6
Output: 1
Explanation: Starting with nums = [3,1,3,4,3]:
- Remove the first two 3's, then nums = [1,4,3]
There are no more pairs that sum up to 6, hence a total of 1 operation.

Constraints:

	1 <= nums.length <= 105
	1 <= nums[i] <= 109
	1 <= k <= 109

 */
pub struct Solution {}

// submission codes start here

use std::borrow::Borrow;
use std::collections::HashMap;
use std::cell::RefCell;
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let mut h = HashMap::<i32, RefCell<i32>>::new();

        for x in nums.iter() {
            {
                let e = h.entry(*x).or_insert(RefCell::new(0));
                *e.borrow_mut() += 1;
            };
            let e = &h[x];

            if let Some(c) = h.get(&(k - x)) {
                if x * 2 == k {
                    if *e.borrow_mut() >= 2 {
                        *e.borrow_mut() -= 2;
                        ret += 1;
                    }
                } else {
                    let min = std::cmp::min(*e.borrow(), *c.borrow());
                    ret += min;
                    *e.borrow_mut() -= min;
                    *c.borrow_mut() -= min;
                }
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
        assert_eq!(2, Solution::max_operations(vec![1,2,3,4], 5));
        assert_eq!(1, Solution::max_operations(vec![3,1,3,4,3], 6));
        assert_eq!(2, Solution::max_operations(vec![1,2,3,1,2], 3));
        assert_eq!(2, Solution::max_operations(vec![4,4,1,3,1,3,2,2,5,5,1,5,2,1,2,3,5,4], 2));
    }
}
