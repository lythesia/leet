/**
 * [347] Top K Frequent Elements
 *
 * Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
 
Example 1:
Input: nums = [1,1,1,2,2,3], k = 2
Output: [1,2]
Example 2:
Input: nums = [1], k = 1
Output: [1]
 
Constraints:

	1 <= nums.length <= 105
	k is in the range [1, the number of unique elements in the array].
	It is guaranteed that the answer is unique.

 
Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::{HashMap,BinaryHeap};
        use std::cmp::Reverse;
        let mut h = HashMap::new();
        for x in nums {
            *h.entry(x).or_insert(0) += 1;
        }
        let mut heap = BinaryHeap::new();
        let mut cnt = 0;
        for (x, c) in h.drain() {
            if cnt < k {
                heap.push((Reverse(c), x));
            } else {
                if let Some((Reverse(top), y)) = heap.pop() {
                    if top < c {
                        heap.push((Reverse(c), x));
                    } else {
                        heap.push((Reverse(top), y));
                    }
                }
            }
            cnt += 1;
        }
        heap.into_iter().map(|(_, k)| k).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", Solution::top_k_frequent(vec![1,1,1,2,2,3], 2));
    }
}
