/**
 * [1790] Check if One String Swap Can Make Strings Equal
 *
 * You are given two strings s1 and s2 of equal length. A string swap is an operation where you choose two indices in a string (not necessarily different) and swap the characters at these indices.
Return true if it is possible to make both strings equal by performing at most one string swap on exactly one of the strings. Otherwise, return false.
 
Example 1:
Input: s1 = "bank", s2 = "kanb"
Output: true
Explanation: For example, swap the first character with the last character of s2 to make "bank".

Example 2:
Input: s1 = "attack", s2 = "defend"
Output: false
Explanation: It is impossible to make them equal with one string swap.

Example 3:
Input: s1 = "kelb", s2 = "kelb"
Output: true
Explanation: The two strings are already equal, so no string swap operation is required.

 
Constraints:

	1 <= s1.length, s2.length <= 100
	s1.length == s2.length
	s1 and s2 consist of only lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut d = 0;
        let (mut a, mut b) = ('0', '0');
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 == c2 { continue; }
            d += 1;
            if d > 2 { return false; }
            if a != '0' { // a b set
                if !(c2 == a && c1 == b) { return false; }
            } else {
                a = c1; b = c2;
            }
        }
        d == 0 || d == 2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::are_almost_equal("bank".into(), "kanb".into()));
        assert!(!Solution::are_almost_equal("ac".into(), "aa".into()));
    }
}
