/**
 * [387] First Unique Character in a String
 *
 * Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.
 
Example 1:
Input: s = "leetcode"
Output: 0
Example 2:
Input: s = "loveleetcode"
Output: 2
Example 3:
Input: s = "aabb"
Output: -1
 
Constraints:

	1 <= s.length <= 105
	s consists of only lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut h = [(0, -1); 26];
        for (i, &b) in s.as_bytes().iter().enumerate() {
            let bi = (b - b'a') as usize;
            let (c, idx) = &mut h[bi];
            *c += 1;
            if *c == 1 {
                *idx = i as i32;
            }
        }
        h.iter()
         .filter(|(c, _)| *c == 1)
         .map(|(_, idx)| *idx)
         .min()
         .unwrap_or(-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, Solution::first_uniq_char("leetcode".into()));
        assert_eq!(2, Solution::first_uniq_char("loveleetcode".into()));
        assert_eq!(-1, Solution::first_uniq_char("aabb".into()));
    }
}
