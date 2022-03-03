/**
 * [1143] Longest Common Subsequence
 *
 * Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.
A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.

	For example, "ace" is a subsequence of "abcde".

A common subsequence of two strings is a subsequence that is common to both strings.
 
Example 1:
Input: text1 = "abcde", text2 = "ace" 
Output: 3  
Explanation: The longest common subsequence is "ace" and its length is 3.

Example 2:
Input: text1 = "abc", text2 = "abc"
Output: 3
Explanation: The longest common subsequence is "abc" and its length is 3.

Example 3:
Input: text1 = "abc", text2 = "def"
Output: 0
Explanation: There is no such common subsequence, so the result is 0.

 
Constraints:

	1 <= text1.length, text2.length <= 1000
	text1 and text2 consist of only lowercase English characters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    /* memorized search: start from si, and tj, what max common can we get
    fn solve(i: usize, j: usize, m: usize, n: usize,
        x: &[u8], y: &[u8],
        memo: &mut Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;
        if i == m || j == n { return 0; }
        if memo[i][j] >=0 { return memo[i][j]; }
        let res = if x[i] == y[j] {
            Self::solve(i+1, j+1, m, n, x, y, memo) + 1
        } else {
            max(
                Self::solve(i+1, j, m, n, x, y, memo),
                Self::solve(i, j+1, m, n, x, y, memo))
        };
        memo[i][j] = res;
        res
    }

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (m, n) = (text1.len(), text2.len());
        let mut memo = vec![vec![-1; n+1]; m+1];
        for i in 0..=m { memo[i][n] = 0; }
        for j in 0..=n { memo[m][j] = 0; }
        let (x, y) = (text1.as_bytes(), text2.as_bytes());
        Self::solve(0, 0, m, n, x, y, &mut memo);
        memo[0][0]
    }
    */

    // derive dp from memo-search
    // (i,j) <-- (i,j+1)
    //   ^   \
    //   |     \
    // (i+1,j)   (i+1,j+1)
    // si == tj: dp(i+1, j+1) + 1
    // si != tj: max{ dp(i+1,j), dp(i,j+1) }
    //p.s: dp array can be compressed into 2-dim array, rolling it!
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (m, n) = (text1.len(), text2.len());
        let mut dp = vec![vec![-1; n+1]; m+1];
        for i in 0..=m { dp[i][n] = 0; }
        for j in 0..=n { dp[m][j] = 0; }
        let (x, y) = (text1.as_bytes(), text2.as_bytes());
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if x[i] == y[j] {
                    dp[i][j] = dp[i+1][j+1] + 1;
                } else {
                    dp[i][j] = std::cmp::max(dp[i+1][j], dp[i][j+1]);
                }
            }
        }
        dp[0][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::longest_common_subsequence("abcde".into(), "ace".into()));
        assert_eq!(3, Solution::longest_common_subsequence("abc".into(), "abc".into()));
        assert_eq!(0, Solution::longest_common_subsequence("abc".into(), "def".into()));
    }
}
