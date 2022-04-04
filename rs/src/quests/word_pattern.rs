/**
 * [290] Word Pattern
 *
 * Given a pattern and a string s, find if s follows the same pattern.
Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.
 
Example 1:
Input: pattern = "abba", s = "dog cat cat dog"
Output: true

Example 2:
Input: pattern = "abba", s = "dog cat cat fish"
Output: false

Example 3:
Input: pattern = "aaaa", s = "dog cat cat dog"
Output: false

 
Constraints:

	1 <= pattern.length <= 300
	pattern contains only lower-case English letters.
	1 <= s.length <= 3000
	s contains only lowercase English letters and spaces ' '.
	s does not contain any leading or trailing spaces.
	All the words in s are separated by a single space.

 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut h = HashMap::new();
        let mut rh = HashMap::new();
        let mut i = pattern.chars();
        let mut j = s.split_ascii_whitespace();
        i.clone().count() == j.clone().count() &&
        i.zip(j).all(|(c, s)| {
            let b = rh.contains_key(&s);
            *h.entry(c).or_insert(s) == s &&
            *rh.entry(s).or_insert(c) == c
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(!Solution::word_pattern("abba".into(), "dog dog dog dog".into()));
    }
}
