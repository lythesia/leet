/**
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
If target is not found in the array, return [-1, -1].
You must write an algorithm with O(log n) runtime complexity.
 
Example 1:
Input: nums = [5,7,7,8,8,10], target = 8
Output: [3,4]
Example 2:
Input: nums = [5,7,7,8,8,10], target = 6
Output: [-1,-1]
Example 3:
Input: nums = [], target = 0
Output: [-1,-1]
 
Constraints:

	0 <= nums.length <= 105
	-109 <= nums[i] <= 109
	nums is a non-decreasing array.
	-109 <= target <= 109

 */
pub struct Solution {}

// submission codes start here
fn binary_search<F: Fn(i32) -> bool>(a: &Vec<i32>, x: i32, pred: F) -> usize {
    let mut n = a.len();
    let (mut lo, mut hi) = (0, n);
    while n > 0 {
        let step = n/2;
        let mi = lo + step;
        if pred(a[mi]) {
            lo = mi + 1;
            n -= step + 1;
        } else { n = step; }
    }
    lo
}

// first (not < x)
fn lower_bound(a: &Vec<i32>, x: i32) -> Result<usize, usize> {
    let pos = binary_search(a, x, |mi| mi < x);
    if pos < a.len() && a[pos] == x { Ok(pos) }
    else { Err(pos) }
}

// first (> x)
fn upper_bound(a: &Vec<i32>, x: i32) -> Result<usize, usize> {
    Ok(binary_search(a, x, |mi| !(mi > x)))
}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match (lower_bound(&nums, target), upper_bound(&nums, target)) {
            (Ok(i), Ok(j)) => vec![i as i32, j as i32 - 1],
            _ => vec![-1, -1],
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![3,4], Solution::search_range(vec![5,7,7,8,8,10], 8));
        assert_eq!(vec![0,0], Solution::search_range(vec![5,7,7,8,8,10], 5));
        assert_eq!(vec![-1,-1], Solution::search_range(vec![5,7,7,8,8,10], 6));
        assert_eq!(vec![-1,-1], Solution::search_range(vec![], 0));
    }
}
