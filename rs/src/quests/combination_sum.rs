/**
 * [39] Combination Sum
 *
 * Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.
The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers is different.
It is guaranteed that the number of unique combinations that sum up to target is less than 150 combinations for the given input.
 
Example 1:
Input: candidates = [2,3,6,7], target = 7
Output: [[2,2,3],[7]]
Explanation:
2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
7 is a candidate, and 7 = 7.
These are the only two combinations.

Example 2:
Input: candidates = [2,3,5], target = 8
Output: [[2,2,2,2],[2,3,3],[3,5]]

Example 3:
Input: candidates = [2], target = 1
Output: []

 
Constraints:

	1 <= candidates.length <= 30
	1 <= candidates[i] <= 200
	All elements of candidates are distinct.
	1 <= target <= 500

 */
pub struct Solution {}

// submission codes start here

fn dfs(a: &[i32], t: i32, v: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if t == 0 {
        ans.push(v.clone());
        return;
    }
    if a.is_empty() || a[0] > t { return; }
    v.push(a[0]);
    dfs(a, t - a[0], v, ans);
    v.pop();
    dfs(&a[1..], t, v, ans);
}
impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut v = vec![];
        let mut ans = vec![];
        dfs(&candidates, target, &mut v, &mut ans);
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", Solution::combination_sum(vec![2,3,6,7], 7));
    }
}
