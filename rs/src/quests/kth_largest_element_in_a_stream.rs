#![feature(array_zip)]
/**
 * [703] Kth Largest Element in a Stream
 *
 * Design a class to find the kth largest element in a stream. Note that it is the kth largest element in the sorted order, not the kth distinct element.
Implement KthLargest class:

	KthLargest(int k, int[] nums) Initializes the object with the integer k and the stream of integers nums.
	int add(int val) Appends the integer val to the stream and returns the element representing the kth largest element in the stream.

 
Example 1:
Input
["KthLargest", "add", "add", "add", "add", "add"]
[[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
Output
[null, 4, 5, 5, 8, 8]
Explanation
KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
kthLargest.add(3);   // return 4
kthLargest.add(5);   // return 5
kthLargest.add(10);  // return 5
kthLargest.add(9);   // return 8
kthLargest.add(4);   // return 8

 
Constraints:

	1 <= k <= 104
	0 <= nums.length <= 104
	-104 <= nums[i] <= 104
	-104 <= val <= 104
	At most 104 calls will be made to add.
	It is guaranteed that there will be at least k elements in the array when you search for the kth element.

 */
pub struct Solution {}

// submission codes start here

use std::{collections::BinaryHeap, cmp::Reverse};
struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    cap: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let n = nums.len().min(k as usize);
        let mut heap = nums.drain(0..n).into_iter().map(Reverse).collect::<BinaryHeap<_>>();
        let mut obj = Self {
            heap,
            cap: k as usize,
        };
        nums.drain(..).for_each(|x| {obj.add(x);});
        obj
    }
    
    fn add(&mut self, val: i32) -> i32 {
        assert!(self.heap.len() + 1 >= self.cap);
        if self.heap.len() < self.cap {
            self.heap.push(Reverse(val));
        } else {
            if let Some(Reverse(top)) = self.heap.pop() {
                if val > top {
                    self.heap.push(Reverse(val));
                } else {
                    self.heap.push(Reverse(top)); // restore
                }
            }
        }
        let Reverse(ans) = self.heap.peek().unwrap();
        *ans
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // let mut k = KthLargest::new(3, vec![4, 5, 8, 2]);
        let mut k = KthLargest::new(1, vec![]);
        for (&x, y) in [3,5,10,9,4].iter().zip([4,5,5,8,8]) {
            assert_eq!(y, k.add(x));
        }
    }
}
