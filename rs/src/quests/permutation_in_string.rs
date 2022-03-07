/**
 * [567] Permutation in String
 *
 * Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
In other words, return true if one of s1's permutations is the substring of s2.
 
Example 1:
Input: s1 = "ab", s2 = "eidbaooo"
Output: true
Explanation: s2 contains one permutation of s1 ("ba").

Example 2:
Input: s1 = "ab", s2 = "eidboaoo"
Output: false

 
Constraints:

	1 <= s1.length, s2.length <= 104
	s1 and s2 consist of lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut h1 = [0; 26];
        let n = s1.len();
        s1.as_bytes().iter().for_each(|&b| h1[(b - b'a') as usize] += 1);

        let s2 = s2.as_bytes();
        let mut h2 = [0; 26];
        for (i, &b) in s2.iter().enumerate() {
            h2[(b - b'a') as usize] += 1; // push last
            if i >= n {
                h2[(s2[i - n] - b'a') as usize] -= 1; // pop fist
            }
            if i >= n - 1 && h1 == h2 { return true; }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert!(Solution::check_inclusion("adc".into(), "dcda".into()));
        assert!(!Solution::check_inclusion("hello".into(), "ooolleoooleh".into()));
    }
}
