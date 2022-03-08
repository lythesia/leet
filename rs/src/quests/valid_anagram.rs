/**
 * [242] Valid Anagram
 *
 * Given two strings s and t, return true if t is an anagram of s, and false otherwise.
An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 
Example 1:
Input: s = "anagram", t = "nagaram"
Output: true
Example 2:
Input: s = "rat", t = "car"
Output: false
 
Constraints:

	1 <= s.length, t.length <= 5 * 104
	s and t consist of lowercase English letters.

 
Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }
        let mut h = [0; 26];
        t.as_bytes().iter().for_each(|&b| h[(b - b'a') as usize] += 1);
        for &b in s.as_bytes() {
            let c = &mut h[(b - b'a') as usize];
            *c -= 1;
            if *c < 0 {
                return false; // return early
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
    }
}
