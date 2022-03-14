/**
 * [77] Combinations
 *
 * Given two integers n and k, return all possible combinations of k numbers out of the range [1, n].
You may return the answer in any order.
 
Example 1:
Input: n = 4, k = 2
Output:
[
  [2,4],
  [3,4],
  [2,3],
  [1,2],
  [1,3],
  [1,4],
]

Example 2:
Input: n = 1, k = 1
Output: [[1]]

 
Constraints:

	1 <= n <= 20
	1 <= k <= n

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn dfs(i: i32, n: i32, cnt: i32, v: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if cnt == 0 {
            ans.push(v.clone());
            return;
        }
        if i > n {
            return;
        }
        // take i
        v.push(i);
        Self::dfs(i + 1, n, cnt - 1, v, ans);
        // backtrack(not take i)
        v.pop();
        Self::dfs(i + 1, n, cnt, v, ans);
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut v = vec![];
        let mut ans = vec![];
        Self::dfs(1, n, k, &mut v, &mut ans);
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", Solution::combine(4, 2));
        println!("{:?}", Solution::combine(1, 1));
    }
}
