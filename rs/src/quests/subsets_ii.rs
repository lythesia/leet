/**
 * [90] Subsets II
 *
 * Given an integer array nums that may contain duplicates, return all possible subsets (the power set).
The solution set must not contain duplicate subsets. Return the solution in any order.
 
Example 1:
Input: nums = [1,2,2]
Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
Example 2:
Input: nums = [0]
Output: [[],[0]]
 
Constraints:

	1 <= nums.length <= 10
	-10 <= nums[i] <= 10

 */
pub struct Solution {}

// submission codes start here
fn dfs(a: &[i32], v: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if a.is_empty() {
        ans.push(v.clone());
        return;
    }
    let counts = a.iter().position(|&x| x != a[0]).unwrap_or(a.len());
    // insert dups, which's count from 0..=counts
    use std::iter::repeat;
    for cnt in 0..=counts {
        // take cnt dups
        v.extend(repeat(a[0]).take(cnt));
        // skip rest dups
        dfs(&a[counts..], v, ans);
        // not take
        v.truncate(v.len() - cnt);
    }
}
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut v = vec![];
        let mut ans = vec![];
        nums.sort();
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
        println!("{:?}", Solution::subsets_with_dup(vec![1,2,2]));
    }
}
