/**
 * [459] Repeated Substring Pattern
 *
 * Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.
 
Example 1:
Input: s = "abab"
Output: true
Explanation: It is the substring "ab" twice.

Example 2:
Input: s = "aba"
Output: false

Example 3:
Input: s = "abcabcabcabc"
Output: true
Explanation: It is the substring "abc" four times or the substring "abcabc" twice.

 
Constraints:

	1 <= s.length <= 104
	s consists of lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // pub fn repeated_substring_pattern(s: String) -> bool {
    //     let n = s.len();
    //     for i in 1..=n/2 {
    //         if n % i == 0 &&
    //             (i..n).step_by(i).all(|j| s[0..i] == s[j..j+i]) {
    //                 return true;
    //             }
    //     }
    //     false
    // }

    // smart ver
    pub fn repeated_substring_pattern(s: String) -> bool {
        s.repeat(2)[1..2*s.len()-1].contains(&s)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(!Solution::repeated_substring_pattern("aba".into()));
        assert!(Solution::repeated_substring_pattern("abcabcabc".into()));
    }
}
