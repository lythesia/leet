/**
 * [516] Longest Palindromic Subsequence
 *
 * Given a string s, find the longest palindromic subsequence's length in s.
A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.
 
Example 1:
Input: s = "bbbab"
Output: 4
Explanation: One possible longest palindromic subsequence is "bbbb".

Example 2:
Input: s = "cbbd"
Output: 2
Explanation: One possible longest palindromic subsequence is "bb".

 
Constraints:

	1 <= s.length <= 1000
	s consists only of lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    /*
    trivial:
    // cal dp[i][j]: from i to j
    // if i == j return 1;
    // if i > j return 0;
    // if dp[i][j] exist, return dp[i][j] 
    // else: dp[i][j] = a[i]==a[j] ? cal(i+1, j-1, dp) + 2 :
                                     max(cal(i+1, j, dp), cal(i, j-1, dp))

    longest common subseq of s and r(s.reverse)
    given s and r, dp[i][j] is LCS of s[0..i-1] and r[0..j-1]
    cutting edge: dp[0][_] = 0 (with no s), dp[_][0] (with no r)
    s[i-1] == r[j-1]: dp[i][j] = dp[i-1][j-1] + 1
    else: dp[i][j] = max(dp[i-1][j], dp[i][j-1])
     */
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut dp = vec![vec![0; 1001]; 1001];
        let n = s.len();
        let r: String = s.chars().rev().collect();
        let sb = s.as_bytes();
        let rb = r.as_bytes();
        for i in 1..=n {
            for j in 1..=n {
                if sb[i - 1] == rb[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[n][n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::longest_palindrome_subseq("bbbab".into()));
        assert_eq!(2, Solution::longest_palindrome_subseq("cbbd".into()));
    }
}
