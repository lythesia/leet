/**
 * [28] Implement strStr()
 *
 * Implement strStr().
Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
Clarification:
What should we return when needle is an empty string? This is a great question to ask during an interview.
For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's strstr() and Java's indexOf().
 
Example 1:
Input: haystack = "hello", needle = "ll"
Output: 2
Example 2:
Input: haystack = "aaaaa", needle = "bba"
Output: -1
Example 3:
Input: haystack = "", needle = ""
Output: 0
 
Constraints:

	0 <= haystack.length, needle.length <= 5 * 104
	haystack and needle consist of only lower-case English characters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(needle.as_str()) {
            Some(i) => i as i32,
            _ => -1,
        }
    }
}

// submission codes end

use std::convert::TryInto;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, Solution::str_str("haystack".into(), "".into()));
    }
}
