/**
 * [680] Valid Palindrome II
 *
 * Given a string s, return true if the s can be palindrome after deleting at most one character from it.
 
Example 1:
Input: s = "aba"
Output: true

Example 2:
Input: s = "abca"
Output: true
Explanation: You could delete the character 'c'.

Example 3:
Input: s = "abc"
Output: false

 
Constraints:

	1 <= s.length <= 105
	s consists of lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn valid(b: &[u8], l: usize, r: usize, del: bool) -> bool {
        if l < r {
            if b[l] == b[r] { Self::valid(b, l + 1, r - 1, del) }
            else if !del {
                Self::valid(b, l + 1, r, true) || // skip left
                Self::valid(b, l, r - 1, true) // skip right
            } else { false }
        } else {
            true
        }
    }

    pub fn valid_palindrome(s: String) -> bool {
        // Self::valid(s.as_bytes(), 0, s.len() - 1, false)

        // loop ver
        let n = s.len();
        let (mut l, mut r) = (0, n - 1);
        let (mut l_drop, mut r_drop) = (0, 0);
        let mut try_drops = 0;
        let b = s.as_bytes();
        while l < r {
            if b[l] != b[r] {
                match try_drops {
                    // try drop left
                    0 => {
                        l_drop = l;
                        r_drop = r;
                        l += 1;
                    },
                    // left fail, try drop right
                    1 => {
                        l = l_drop;
                        r = r_drop;
                        r -= 1;
                    },
                    _ => { return false; },
                } 
                try_drops += 1;
            } else {
                l += 1;
                r -= 1;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::valid_palindrome("aba".into()));
        assert!(Solution::valid_palindrome("abca".into()));
        assert!(!Solution::valid_palindrome("abc".into()));
        assert!(Solution::valid_palindrome("bc".into()));
        assert!(Solution::valid_palindrome("ececabbacec".into()));
    }
}
