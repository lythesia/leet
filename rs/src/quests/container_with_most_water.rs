/**
 * [11] Container With Most Water
 *
 * Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai). n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and (i, 0). Find two lines, which together with x-axis forms a container, such that the container contains the most water.

Note: You may not slant the container and n is at least 2.





The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.



Example:

Input: [1,8,6,2,5,4,8,3,7]
Output: 49

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r, mut ans) = (0, height.len()-1, 0);
        while l < r {
            ans = std::cmp::max(
                ans,
                std::cmp::min(height[l], height[r]) * (r-l) as i32
                );
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
        assert_eq!(Solution::max_area(vec![2,1]), 1);
        assert_eq!(Solution::max_area(vec![10,14,10,4,10,2,6,1,6,12]), 96);
    }
}
