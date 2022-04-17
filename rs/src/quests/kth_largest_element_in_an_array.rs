/**
 * [215] Kth Largest Element in an Array
 *
 * Given an integer array nums and an integer k, return the kth largest element in the array.
Note that it is the kth largest element in the sorted order, not the kth distinct element.
 
Example 1:
Input: nums = [3,2,1,5,6,4], k = 2
Output: 5
Example 2:
Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
Output: 4
 
Constraints:

	1 <= k <= nums.length <= 104
	-104 <= nums[i] <= 104

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for x in nums {
            if heap.len() < k {
                heap.push(Reverse(x));
            } else {
                if let Some(Reverse(top)) = heap.pop() {
                    if x > top {
                        heap.push(Reverse(x));
                    } else {
                        heap.push(Reverse(top));
                    }
                }
            }
        }
        let Reverse(ans) = heap.pop().unwrap();
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{}", Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4));
    }
}
