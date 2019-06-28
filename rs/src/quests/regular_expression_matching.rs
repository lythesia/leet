/**
 * [10] Regular Expression Matching
 *
 * Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.

'.' Matches any single character.
'*' Matches zero or more of the preceding element.


The matching should cover the entire input string (not partial).

Note:


	s could be empty and contains only lowercase letters a-z.
	p could be empty and contains only lowercase letters a-z, and characters like . or *.


Example 1:

Input:
s = "aa"
p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".


Example 2:

Input:
s = "aa"
p = "a*"
Output: true
Explanation: '*' means zero or more of the precedeng element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".


Example 3:

Input:
s = "ab"
p = ".*"
Output: true
Explanation: ".*" means "zero or more (*) of any character (.)".


Example 4:

Input:
s = "aab"
p = "c*a*b"
Output: true
Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore it matches "aab".


Example 5:

Input:
s = "mississippi"
p = "mis*is*p*."
Output: false


 */
pub struct Solution {}

// submission codes start here

/*
 *class Solution(object):
    def isMatch(self, text, pattern):
        dp = [[False] * (len(pattern) + 1) for _ in range(len(text) + 1)]

        dp[-1][-1] = True
        for i in range(len(text), -1, -1):
            for j in range(len(pattern) - 1, -1, -1):
                first_match = i < len(text) and pattern[j] in {text[i], '.'}
                if j+1 < len(pattern) and pattern[j+1] == '*':
                    dp[i][j] = dp[i][j+2] or first_match and dp[i+1][j]
                else:
                    dp[i][j] = first_match and dp[i+1][j+1]

        return dp[0][0]
 */
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::matches(s.as_bytes(), p.as_bytes())
    }

    // recursive
    fn matches(s: &[u8], p: &[u8]) -> bool {
        // if no pattern, source MUST be empty
        if p.is_empty() { return s.is_empty(); }

        let first_match = !s.is_empty() && (s[0] == p[0] || p[0] == b'.');
        // if p == w*
        if p.len() >= 2 && p[1] == b'*' {
            // no match || consume one, then match afterwards
            Self::matches(&s, &p[2..]) || (first_match && Self::matches(&s[1..], &p))
        // if p == ww
        } else {
            first_match && Self::matches(&s[1..], &p[1..])
        }
    }

    // dp[i][j]: s[i..] matches p[j..]
    // trivial: dp[s][p] = true, dp[..s][p] = false
    // let first = s[i] match p[j]
    // 1. p[j -> j+1] == w*: dp[i][j] = dp[i][j+2] or (first and dp[i+1][j])
    // 2. p[j -> j+1] == ww: dp[i][j] = first and dp[i+1][j+1]
    pub fn is_match_dp(s: String, p: String) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let mut dp = vec![vec![false; p.len()+1]; s.len()+1]; // s.len+1 x p.len+1
        dp[s.len()][p.len()] = true;
        for i in (0..s.len()+1).rev() {
            for j in (0..p.len()).rev() {
                // i: [0, slen]
                // j: [0, plen - 1]
                let first_match = i < s.len() && (s[i] == p[j] || p[j] == b'.');
                if j+1 < p.len() && p[j+1] == b'*' {
                    //               j+2 <= plen               i+1<=slen
                    dp[i][j] = dp[i][j+2] || (first_match && dp[i+1][j]);
                } else {
                    // i+1 <= slen
                    // if j+1 == plen, dp[..][j+1] === false
                    dp[i][j] = first_match && dp[i+1][j+1];
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
        assert!(Solution::is_match_dp("baabbbaccbccacacc".to_owned(), "c*..b*a*a.*a..*c".to_owned()));
    }
}
