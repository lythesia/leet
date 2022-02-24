/**
 * [42] Trapping Rain Water
 *
 * Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.
 
Example 1:
Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6
Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.

Example 2:
Input: height = [4,2,0,3,2,5]
Output: 9

 
Constraints:

	n == height.length
	1 <= n <= 2 * 104
	0 <= height[i] <= 105

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n < 3 { return 0; }
        let (mut left_max, mut right_max) = (height[0], height[n-1]);
        let (mut left, mut right) = (vec![left_max; n], vec![right_max; n]);
        use std::cmp::{min, max};
        (1..n).zip((0..n-1).rev())
            .fold((height[0], height[n-1]), |(left_max, right_max), (i, j)| {
               ({ left[i] = max(left_max, height[i]); left[i] },
                { right[j] = max(right_max, height[j]); right[j] })
            });
        // for (i, j) in (1..n).zip((0..n-1).rev()) {
        //     left_max = max(left_max, height[i]);
        //     left[i] = left_max;
        //     right_max = max(right_max, height[j]);
        //     right[j] = right_max;
        // }
        (0..n).into_iter().fold(0, |acc, i| acc + min(left[i], right[i]) - height[i])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]));
        assert_eq!(9, Solution::trap(vec![4,2,0,3,2,5]));
    }
}
