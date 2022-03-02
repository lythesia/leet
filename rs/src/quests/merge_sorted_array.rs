/**
 * [88] Merge Sorted Array
 *
 * You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
Merge nums1 and nums2 into a single array sorted in non-decreasing order.
The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
 
Example 1:
Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
Output: [1,2,2,3,5,6]
Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.

Example 2:
Input: nums1 = [1], m = 1, nums2 = [], n = 0
Output: [1]
Explanation: The arrays we are merging are [1] and [].
The result of the merge is [1].

Example 3:
Input: nums1 = [0], m = 0, nums2 = [1], n = 1
Output: [1]
Explanation: The arrays we are merging are [] and [1].
The result of the merge is [1].
Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1.

 
Constraints:

	nums1.length == m + n
	nums2.length == n
	0 <= m, n <= 200
	1 <= m + n <= 200
	-109 <= nums1[i], nums2[j] <= 109

 
Follow up: Can you come up with an algorithm that runs in O(m + n) time?

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // two pointers start from end of two arrays
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);
        let mut ok = m + n - 1;
        let (mut x, mut y) = ((0..m).rev().peekable(), (0..n).rev().peekable());
        loop {
            match (x.peek(), y.peek()) {
                (Some(&i), Some(&j)) => {
                    let x = if nums1[i] >= nums2[j] {
                        x.next();
                        nums1[i]
                    } else {
                        y.next();
                        nums2[j]
                    };
                    nums1[ok] = x;
                    ok -= 1;
                },
                _ => break,
            }
        }
        let rest = y.len();
        if rest > 0 {
            nums1[ok-(rest-1)..=ok].copy_from_slice(&nums2[..rest]);
        }

        // let mut nums2_cnt = n;
        // if m >= 1 && n >= 1 {
        //     let (mut i, mut j) = (m-1, n-1);
        //     loop {
        //         if nums1[i] >= nums2[j] {
        //             nums1[ok] = nums1[i];
        //             ok -= 1;
        //             if i == 0 { break; }
        //             i -= 1;
        //         } else {
        //             nums1[ok] = nums2[j];
        //             ok -= 1;
        //             nums2_cnt -= 1;;
        //             if j == 0 { break; }
        //             j -= 1;
        //         }
        //     }
        // }
        // // left nums2_cnt numbers in nums2
        // nums1[ok-nums2_cnt+1..=ok].copy_from_slice(&nums2[..nums2_cnt]) 
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut v1 = vec![1,2,3,0,0,0]; let mut v2 = vec![2,5,6];
        Solution::merge(&mut v1, 3, &mut v2, 3);
    }
}
