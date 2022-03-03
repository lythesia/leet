/**
 * [976] Largest Perimeter Triangle
 *
 * Given an integer array nums, return the largest perimeter of a triangle with a non-zero area, formed from three of these lengths. If it is impossible to form any triangle of a non-zero area, return 0.
 
Example 1:
Input: nums = [2,1,2]
Output: 5

Example 2:
Input: nums = [1,2,1]
Output: 0

 
Constraints:

	3 <= nums.length <= 104
	1 <= nums[i] <= 106

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        (0..nums.len()-2).rev()
            .find(|&i| nums[i] + nums[i+1] > nums[i+2])
            .map_or(0, |i| nums[i..i+3].iter().sum())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, Solution::largest_perimeter(vec![2,1,2]));
        assert_eq!(0, Solution::largest_perimeter(vec![2,1,1]));
    }
}
