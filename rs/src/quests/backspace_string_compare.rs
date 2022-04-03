/**
 * [844] Backspace String Compare
 *
 * Given two strings s and t, return true if they are equal when both are typed into empty text editors. '#' means a backspace character.
Note that after backspacing an empty text, the text will continue empty.
 
Example 1:
Input: s = "ab#c", t = "ad#c"
Output: true
Explanation: Both s and t become "ac".

Example 2:
Input: s = "ab##", t = "c#d#"
Output: true
Explanation: Both s and t become "".

Example 3:
Input: s = "a#c", t = "b"
Output: false
Explanation: s becomes "c" while t becomes "b".

 
Constraints:

	1 <= s.length, t.length <= 200
	s and t only contain lowercase letters and '#' characters.

 
Follow up: Can you solve it in O(n) time and O(1) space?

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let f = |s: String| {
            let mut stack = vec![];
            s.chars().for_each(|c| match c {
                '#' => {stack.pop();},
                _ => stack.push(c),
            });
            stack
        };
        f(s) == f(t)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::backspace_compare("ab##".into(), "c#d#".into()));
    }
}
