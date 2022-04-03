/**
 * [58] Length of Last Word
 *
 * Given a string s consisting of some words separated by some number of spaces, return the length of the last word in the string.
A word is a maximal substring consisting of non-space characters only.
 
Example 1:
Input: s = "Hello World"
Output: 5
Explanation: The last word is "World" with length 5.

Example 2:
Input: s = "   fly me   to   the moon  "
Output: 4
Explanation: The last word is "moon" with length 4.

Example 3:
Input: s = "luffy is still joyboy"
Output: 6
Explanation: The last word is "joyboy" with length 6.

 
Constraints:

	1 <= s.length <= 104
	s consists of only English letters and spaces ' '.
	There will be at least one word in s.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut it = s.chars().rev();
        it.position(|c| c != ' ');
        let mut tot = 1;
        match it.position(|c| { tot += 1; c == ' ' }) {
            Some(i) => i as i32 + 1,
            _ => tot,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::length_of_last_word("   fly me   to   the moon  ".into()));
        assert_eq!(5, Solution::length_of_last_word("hello world".into()));
        assert_eq!(6, Solution::length_of_last_word("luffy is still joyboy".into()));
        assert_eq!(1, Solution::length_of_last_word("a".into()));
        assert_eq!(2, Solution::length_of_last_word("ab  ".into()));
    }
}
