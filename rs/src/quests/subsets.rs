/**
 * [78] Subsets
 *
 * Given an integer array nums of unique elements, return all possible subsets (the power set).
The solution set must not contain duplicate subsets. Return the solution in any order.
 
Example 1:
Input: nums = [1,2,3]
Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

Example 2:
Input: nums = [0]
Output: [[],[0]]

 
Constraints:

	1 <= nums.length <= 10
	-10 <= nums[i] <= 10
	All the numbers of nums are unique.

 */
pub struct Solution {}

// submission codes start here

fn dfs(a: &[i32], v: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if a.is_empty() {
        ans.push(v.clone());
        return;
    }
    // take
    v.push(a[0]);
    dfs(&a[1..], v, ans);
    // not take
    v.pop();
    dfs(&a[1..], v, ans);
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut v = vec![];
        let mut ans = vec![];
        dfs(&nums[..], &mut v, &mut ans);
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", Solution::subsets(vec![1,2,3]));
    }
}
