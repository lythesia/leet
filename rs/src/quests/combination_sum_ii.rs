/**
 * [40] Combination Sum II
 *
 * Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.
Each number in candidates may only be used once in the combination.
Note: The solution set must not contain duplicate combinations.
 
Example 1:
Input: candidates = [10,1,2,7,6,1,5], target = 8
Output: 
[
[1,1,6],
[1,2,5],
[1,7],
[2,6]
]

Example 2:
Input: candidates = [2,5,2,1,2], target = 5
Output: 
[
[1,2,2],
[5]
]

 
Constraints:

	1 <= candidates.length <= 100
	1 <= candidates[i] <= 50
	1 <= target <= 30

 */
pub struct Solution {}

// submission codes start here

fn dfs(a: &[i32], t: i32, v: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if t == 0 {
        ans.push(v.clone());
        return;
    }
    if a.is_empty() || a[0] > t { return; }
    // if a[0] dup
    let counts = a.iter().position(|&x| x != a[0]).unwrap_or(a.len());
    use std::iter::repeat;
    for cnt in 0..=counts {
        let repeat_sum = a[0]*cnt as i32;
        if repeat_sum <= t {
            // take cnt dups
            v.extend(repeat(a[0]).take(cnt));
            // skip rest dups
            dfs(&a[counts..], t - repeat_sum, v, ans);
            // restore
            v.truncate(v.len() - cnt);
        } else { break; }
    }
}
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
        println!("{:?}", Solution::combination_sum2(vec![2,5,2,1,2], 5));
    }
}
