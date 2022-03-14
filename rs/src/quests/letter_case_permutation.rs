/**
 * [784] Letter Case Permutation
 *
 * Given a string s, you can transform every letter individually to be lowercase or uppercase to create another string.
Return a list of all possible strings we could create. Return the output in any order.
 
Example 1:
Input: s = "a1b2"
Output: ["a1b2","a1B2","A1b2","A1B2"]

Example 2:
Input: s = "3z4"
Output: ["3z4","3Z4"]

 
Constraints:

	1 <= s.length <= 12
	s consists of lowercase English letters, uppercase English letters, and digits.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn dfs(s: &mut Vec<u8>, from: usize, ans: &mut Vec<String>) {
        if let Some(pos) = s[from..].iter().position(|&c| c.is_ascii_alphabetic()) {
            let c = s[from + pos];
            // flip
            s[from + pos] = if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            };
            Self::dfs(s, from + pos + 1, ans);
            // undo flip
            s[from + pos] = c;
            Self::dfs(s, from + pos + 1, ans);

        } else {
            ans.push(unsafe {String::from_utf8_unchecked(s.clone())});
        }
    }

    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut s = s.into_bytes();
        let mut ans = vec![];
        Self::dfs(&mut s, 0, &mut ans);
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", Solution::letter_case_permutation("a1b2".into()));
        println!("{:?}", Solution::letter_case_permutation("3z4".into()));
    }
}
