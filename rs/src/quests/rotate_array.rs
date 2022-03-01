/**
 * [189] Rotate Array
 *
 * Given an array, rotate the array to the right by k steps, where k is non-negative.
 
Example 1:
Input: nums = [1,2,3,4,5,6,7], k = 3
Output: [5,6,7,1,2,3,4]
Explanation:
rotate 1 steps to the right: [7,1,2,3,4,5,6]
rotate 2 steps to the right: [6,7,1,2,3,4,5]
rotate 3 steps to the right: [5,6,7,1,2,3,4]

Example 2:
Input: nums = [-1,-100,3,99], k = 2
Output: [3,99,-1,-100]
Explanation: 
rotate 1 steps to the right: [99,-1,-100,3]
rotate 2 steps to the right: [3,99,-1,-100]

 
Constraints:

	1 <= nums.length <= 105
	-231 <= nums[i] <= 231 - 1
	0 <= k <= 105

 
Follow up:

	Try to come up with as many solutions as you can. There are at least three different ways to solve this problem.
	Could you do it in-place with O(1) extra space?

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    /* swap ver: space O(1)
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        if k == 0 { return; }
        let mut tot = n;
        let mut i = 0;
        loop {
            let mut t = nums[i];
            let mut c = 1;
            loop {
                let j = (i + c*k)%n;
                std::mem::swap(&mut nums[j], &mut t);
                tot -= 1;
                if c*k % n == 0 { break; }
                c += 1;
            }
            i += 1;
            if tot == 0 { break; }
        }
    }
    */

    /* extend ver: space O(n)
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        nums.extend_from_within(..);
        nums.drain(..n-k);
        nums.drain(n..);
    }
    */

    // reverse ver
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        let (l, r) = nums.split_at_mut(n-k);
        l.reverse();
        r.reverse();
        nums.reverse();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut v = vec![1,2,3,4,5,6,7];
        Solution::rotate(&mut v, 3);
        assert_eq!(vec![5,6,7,1,2,3,4], v);

        let mut v = vec![-1,-100,3,99];
        Solution::rotate(&mut v, 2);
        assert_eq!(vec![3,99,-1,-100], v);

        let mut v = vec![1,2,3,4,5,6];
        Solution::rotate(&mut v, 4);
        assert_eq!(vec![3,4,5,6,1,2], v);
    }
}
